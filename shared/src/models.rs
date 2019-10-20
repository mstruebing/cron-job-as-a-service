// modules
use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use juniper;

// internal
use crate::database;
use crate::schema::jobs;
use crate::schema::secrets;
use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(juniper::GraphQLInputObject, AsChangeset)]
#[table_name = "users"]
pub struct UpdatedUser {
    pub email: String,
    pub password: String,
}

#[juniper::object(description = "A user")]
impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn password(&self) -> &str {
        self.password.as_str()
    }

    pub fn jobs(&self) -> Vec<Job> {
        let connection = database::establish_connection();

        jobs::dsl::jobs
            .filter(jobs::dsl::user_id.eq(self.id))
            .load::<Job>(&connection)
            .expect("Error loading jobs for user")
    }
}

#[derive(Queryable)]
pub struct Job {
    pub id: i32,
    pub user_id: i32,
    pub schedule: String,
    pub command: String,
    pub last_run: i32,
    pub next_run: i32,
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewJob {
    pub user_id: i32,
    pub schedule: String,
    pub command: String,
}

#[derive(Insertable)]
#[table_name = "jobs"]
// TODO: Consider a better name
pub struct NewJobWithRuns {
    pub user_id: i32,
    pub schedule: String,
    pub command: String,
    pub last_run: i32,
    pub next_run: i32,
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdadedJob {
    pub schedule: String,
    pub command: String,
}

#[derive(AsChangeset)]
#[table_name = "jobs"]
// TODO: Consider a better name
pub struct UpdadedJobWithRuns {
    pub schedule: String,
    pub command: String,
    pub last_run: i32,
    pub next_run: i32,
}

#[juniper::object(description = "A Job of a User")]
impl Job {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn schedule(&self) -> &str {
        self.schedule.as_str()
    }

    pub fn command(&self) -> &str {
        self.command.as_str()
    }

    pub fn last_run(&self) -> i32 {
        self.last_run
    }

    pub fn next_run(&self) -> i32 {
        self.next_run
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn secrets(&self) -> Vec<Secret> {
        let connection = database::establish_connection();

        secrets::dsl::secrets
            .filter(secrets::dsl::job_id.eq(self.id))
            .load::<Secret>(&connection)
            .expect("Error loading jobs for user")
    }
}

#[derive(Queryable)]
pub struct Secret {
    pub id: i32,
    pub job_id: i32,
    pub key: String,
    pub value: String,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "secrets"]
pub struct NewSecret {
    pub job_id: i32,
    pub key: String,
    pub value: String,
}

#[derive(juniper::GraphQLInputObject, AsChangeset)]
#[table_name = "secrets"]
pub struct UpdatedSecret {
    pub key: String,
    pub value: String,
}

#[juniper::object(description = "A Job of a User")]
impl Secret {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn job_id(&self) -> i32 {
        self.job_id
    }
}

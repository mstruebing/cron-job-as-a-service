// stdlib
use std::env;

// modules
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::RootNode;

// internal
use shared::utils;

use crate::schema::jobs;
use crate::schema::secrets;
use crate::schema::users;

pub struct QueryRoot;

pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

#[juniper::object]
impl QueryRoot {
    fn users() -> Vec<User> {
        let connection = establish_connection();

        users::dsl::users
            .load::<User>(&connection)
            .expect("Error loading users")
    }

    fn jobs() -> Vec<Job> {
        let connection = establish_connection();

        jobs::dsl::jobs
            .load::<Job>(&connection)
            .expect("Error loading jobs")
    }

    fn secrets() -> Vec<Secret> {
        let connection = establish_connection();

        secrets::dsl::secrets
            .load::<Secret>(&connection)
            .expect("Error loading secrets")
    }
}

#[juniper::object]
impl MutationRoot {
    fn create_user(data: NewUser) -> User {
        let connection = establish_connection();

        diesel::insert_into(users::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving user")
    }

    fn create_job(data: NewJob) -> Job {
        let connection = establish_connection();
        let last_run = utils::get_current_timestamp();
        let next_run = utils::get_next_run(&data.schedule);

        // TODO: is there a better way to merge these structs?
        let data = NewJobWithRuns {
            user_id: data.user_id,
            schedule: data.schedule,
            command: data.command,
            last_run,
            next_run,
        };

        diesel::insert_into(jobs::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving job")
    }

    fn create_secret(data: NewSecret) -> Secret {
        let connection = establish_connection();

        diesel::insert_into(secrets::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving secret")
    }

    fn update_user(id: i32, data: UpdatedUser) -> User {
        let connection = establish_connection();

        diesel::update(users::dsl::users.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating user")
    }

    fn update_job(id: i32, data: UpdadedJob) -> Job {
        let connection = establish_connection();

        let last_run = utils::get_current_timestamp();
        let next_run = utils::get_next_run(&data.schedule);

        // TODO: is there a better way to merge these structs?
        let data = UpdadedJobWithRuns {
            schedule: data.schedule,
            command: data.command,
            last_run,
            next_run,
        };

        diesel::update(jobs::dsl::jobs.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating job")
    }

    fn update_secret(id: i32, data: UpdatedSecret) -> Secret {
        let connection = establish_connection();

        diesel::update(secrets::dsl::secrets.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating secret")
    }

    fn delete_user(id: i32) -> bool {
        let connection = establish_connection();

        let num_deleted = diesel::delete(users::dsl::users.find(id))
            .execute(&connection)
            .expect("Error deleting user");

        num_deleted != 0
    }

    fn delete_job(id: i32) -> bool {
        let connection = establish_connection();

        let num_deleted = diesel::delete(jobs::dsl::jobs.find(id))
            .execute(&connection)
            .expect("Error deleting job");

        num_deleted != 0
    }

    fn delete_secret(id: i32) -> bool {
        let connection = establish_connection();

        let num_deleted = diesel::delete(secrets::dsl::secrets.find(id))
            .execute(&connection)
            .expect("Error deleting secret");

        num_deleted != 0
    }
}

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
        let connection = establish_connection();

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
        let connection = establish_connection();

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

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

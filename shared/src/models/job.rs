// modules
use diesel::{prelude::*, AsChangeset, Insertable, Queryable};

// internal
use crate::database;
use crate::models::secret::Secret;
use crate::schema::jobs;
use crate::schema::secrets;

#[derive(Queryable, AsChangeset, Identifiable, Debug)]
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
        get_secrets(self)
    }
}

impl Job {
    pub fn secrets(&self) -> Vec<Secret> {
        get_secrets(self)
    }

    pub fn last_run(mut self, last_run: i32) -> Self {
        self.last_run = last_run;
        self
    }

    pub fn next_run(mut self, next_run: i32) -> Self {
        self.next_run = next_run;
        self
    }
}

fn get_secrets(job: &Job) -> Vec<Secret> {
    let connection = database::establish_connection();

    secrets::dsl::secrets
        .filter(secrets::dsl::job_id.eq(job.id))
        .load::<Secret>(&connection)
        .expect("Error loading jobs for user")
}

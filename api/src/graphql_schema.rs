extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use juniper::{EmptyMutation, RootNode};

pub struct QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}

#[juniper::object]
impl QueryRoot {
    fn users() -> Vec<User> {
        use crate::schema::users::dsl::*;
        let connection = establish_connection();
        users
            .load::<User>(&connection)
            .expect("Error loading users")
    }

    fn jobs() -> Vec<Job> {
        use crate::schema::jobs::dsl::*;
        let connection = establish_connection();
        jobs.load::<Job>(&connection).expect("Error loading jobs")
    }

    fn secrets() -> Vec<Secret> {
        use crate::schema::secrets::dsl::*;
        let connection = establish_connection();
        secrets
            .load::<Secret>(&connection)
            .expect("Error loading secrets")
    }
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
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
        use crate::schema::jobs::dsl::*;
        let connection = establish_connection();
        jobs.filter(user_id.eq(self.id))
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
        use crate::schema::secrets::dsl::*;
        let connection = establish_connection();
        secrets
            .filter(job_id.eq(self.id))
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

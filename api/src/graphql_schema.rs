extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use juniper::RootNode;

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
        use crate::schema::users::dsl::users;
        let connection = establish_connection();

        users
            .load::<User>(&connection)
            .expect("Error loading users")
    }

    fn jobs() -> Vec<Job> {
        use crate::schema::jobs::dsl::jobs;
        let connection = establish_connection();

        jobs.load::<Job>(&connection).expect("Error loading jobs")
    }

    fn secrets() -> Vec<Secret> {
        use crate::schema::secrets::dsl::secrets;
        let connection = establish_connection();

        secrets
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
        use crate::schema::users::dsl::users;
        let connection = establish_connection();

        diesel::update(users.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating user")
    }

    fn update_job(id: i32, data: UpdadedJob) -> Job {
        use crate::schema::jobs::dsl::jobs;
        let connection = establish_connection();

        diesel::update(jobs.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating job")
    }

    fn update_secret(id: i32, data: UpdatedSecret) -> Secret {
        use crate::schema::secrets::dsl::secrets;
        let connection = establish_connection();

        diesel::update(secrets.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating secret")
    }

    fn delete_user(id: i32) -> bool {
        use crate::schema::users::dsl::users;
        let connection = establish_connection();

        let num_deleted = diesel::delete(users.find(id))
            .execute(&connection)
            .expect("Error deleting user");

        num_deleted != 0
    }

    fn delete_job(id: i32) -> bool {
        use crate::schema::jobs::dsl::jobs;
        let connection = establish_connection();

        let num_deleted = diesel::delete(jobs.find(id))
            .execute(&connection)
            .expect("Error deleting job");

        num_deleted != 0
    }

    fn delete_secret(id: i32) -> bool {
        use crate::schema::secrets::dsl::secrets;
        let connection = establish_connection();

        let num_deleted = diesel::delete(secrets.find(id))
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
        use crate::schema::jobs::dsl::{jobs, user_id};
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

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "jobs"]
pub struct NewJob {
    pub user_id: i32,
    pub schedule: String,
    pub command: String,
    pub last_run: i32, // TODO: generate self
    pub next_run: i32, // TODO: generate self
}

#[derive(juniper::GraphQLInputObject, AsChangeset)]
#[table_name = "jobs"]
pub struct UpdadedJob {
    pub schedule: String,
    pub command: String,
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
        use crate::schema::secrets::dsl::{job_id, secrets};
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

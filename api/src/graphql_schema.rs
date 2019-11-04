// modules
use diesel::prelude::*;
use juniper::RootNode;

// internal
use shared::{
    models::{job::*, secret::*, user::*},
    schema::{jobs, secrets, users},
    utils, Context,
};

pub struct QueryRoot;

pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

#[juniper::object(
    description = "The query type",
    Context = Context,
)]
impl QueryRoot {
    fn users(context: &Context) -> Vec<User> {
        let connection = context.pool.get().expect("Expected a connection");

        users::dsl::users
            .load::<User>(&connection)
            .expect("Error loading users")
    }

    fn jobs(context: &Context) -> Vec<Job> {
        let connection = context.pool.get().expect("Expected a connection");

        jobs::dsl::jobs
            .load::<Job>(&connection)
            .expect("Error loading jobs")
    }

    fn secrets(context: &Context) -> Vec<Secret> {
        let connection = context.pool.get().expect("Expected a connection");

        secrets::dsl::secrets
            .load::<Secret>(&connection)
            .expect("Error loading secrets")
    }
}

#[juniper::object(
    description = "The mutation type",
    Context = Context,
)]
impl MutationRoot {
    fn create_user(context: &Context, mut data: NewUser) -> User {
        let connection = context.pool.get().expect("Expected a connection");

        // TODO: Error handling
        data.password = utils::hash_password(&data.password).unwrap();

        diesel::insert_into(users::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving user")
    }

    fn login(context: &Context, data: NewUser) -> String {
        let connection = context.pool.get().expect("Expected a connection");

        let result: Vec<String> = users::table
            .find(1)
            .select(users::password)
            .filter(users::dsl::email.eq(data.email))
            .load(&connection)
            .expect("Stuff");

        let hash = result[0].clone();
        let correct = utils::verify_hash(&hash, &data.password).unwrap();

        if correct {}

        "123".to_string()
    }

    fn create_job(context: &Context, data: NewJob) -> Job {
        let connection = context.pool.get().expect("Expected a connection");
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

    fn create_secret(context: &Context, data: NewSecret) -> Secret {
        let connection = context.pool.get().expect("Expected a connection");

        diesel::insert_into(secrets::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving secret")
    }

    fn update_user(context: &Context, id: i32, data: UpdatedUser) -> User {
        let connection = context.pool.get().expect("Expected a connection");

        diesel::update(users::dsl::users.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating user")
    }

    fn update_job(context: &Context, id: i32, data: UpdadedJob) -> Job {
        let connection = context.pool.get().expect("Expected a connection");

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

    fn update_secret(context: &Context, id: i32, data: UpdatedSecret) -> Secret {
        let connection = context.pool.get().expect("Expected a connection");

        diesel::update(secrets::dsl::secrets.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating secret")
    }

    fn delete_user(context: &Context, id: i32) -> bool {
        let connection = context.pool.get().expect("Expected a connection");

        let num_deleted = diesel::delete(users::dsl::users.find(id))
            .execute(&connection)
            .expect("Error deleting user");

        num_deleted != 0
    }

    fn delete_job(context: &Context, id: i32) -> bool {
        let connection = context.pool.get().expect("Expected a connection");

        let num_deleted = diesel::delete(jobs::dsl::jobs.find(id))
            .execute(&connection)
            .expect("Error deleting job");

        num_deleted != 0
    }

    fn delete_secret(context: &Context, id: i32) -> bool {
        let connection = context.pool.get().expect("Expected a connection");

        let num_deleted = diesel::delete(secrets::dsl::secrets.find(id))
            .execute(&connection)
            .expect("Error deleting secret");

        num_deleted != 0
    }
}

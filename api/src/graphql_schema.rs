// modules
use diesel::prelude::*;
use juniper::RootNode;

// internal
use shared::{
    database,
    models::{
        Job, NewJob, NewJobWithRuns, NewSecret, NewUser, Secret, UpdadedJob, UpdadedJobWithRuns,
        UpdatedSecret, UpdatedUser, User,
    },
    schema::{jobs, secrets, users},
    utils,
};

pub struct QueryRoot;

pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

#[juniper::object]
impl QueryRoot {
    fn users() -> Vec<User> {
        let connection = database::establish_connection();

        users::dsl::users
            .load::<User>(&connection)
            .expect("Error loading users")
    }

    fn jobs() -> Vec<Job> {
        let connection = database::establish_connection();

        jobs::dsl::jobs
            .load::<Job>(&connection)
            .expect("Error loading jobs")
    }

    fn secrets() -> Vec<Secret> {
        let connection = database::establish_connection();

        secrets::dsl::secrets
            .load::<Secret>(&connection)
            .expect("Error loading secrets")
    }
}

#[juniper::object]
impl MutationRoot {
    fn create_user(data: NewUser) -> User {
        let connection = database::establish_connection();

        diesel::insert_into(users::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving user")
    }

    fn create_job(data: NewJob) -> Job {
        let connection = database::establish_connection();
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
        let connection = database::establish_connection();

        diesel::insert_into(secrets::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving secret")
    }

    fn update_user(id: i32, data: UpdatedUser) -> User {
        let connection = database::establish_connection();

        diesel::update(users::dsl::users.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating user")
    }

    fn update_job(id: i32, data: UpdadedJob) -> Job {
        let connection = database::establish_connection();

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
        let connection = database::establish_connection();

        diesel::update(secrets::dsl::secrets.find(id))
            .set(&data)
            .get_result(&connection)
            .expect("Error updating secret")
    }

    fn delete_user(id: i32) -> bool {
        let connection = database::establish_connection();

        let num_deleted = diesel::delete(users::dsl::users.find(id))
            .execute(&connection)
            .expect("Error deleting user");

        num_deleted != 0
    }

    fn delete_job(id: i32) -> bool {
        let connection = database::establish_connection();

        let num_deleted = diesel::delete(jobs::dsl::jobs.find(id))
            .execute(&connection)
            .expect("Error deleting job");

        num_deleted != 0
    }

    fn delete_secret(id: i32) -> bool {
        let connection = database::establish_connection();

        let num_deleted = diesel::delete(secrets::dsl::secrets.find(id))
            .execute(&connection)
            .expect("Error deleting secret");

        num_deleted != 0
    }
}

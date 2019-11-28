// modules
use diesel::prelude::*;
use juniper::{graphql_value, FieldResult, RootNode};

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
    fn users(context: &Context) -> FieldResult<Vec<User>> {
        let connection = context.pool.get()?;
        Ok(users::dsl::users.load::<User>(&connection)?)
    }

    fn jobs(context: &Context) -> FieldResult<Vec<Job>> {
        let connection = context.pool.get()?;
        Ok(jobs::dsl::jobs.load::<Job>(&connection)?)
    }

    fn secrets(context: &Context) -> FieldResult<Vec<Secret>> {
        let connection = context.pool.get()?;
        Ok(secrets::dsl::secrets.load::<Secret>(&connection)?)
    }
}

#[juniper::object(
    description = "The mutation type",
    Context = Context,
)]
impl MutationRoot {
    fn create_user(context: &Context, mut data: NewUser) -> FieldResult<User> {
        let connection = context.pool.get()?;
        data.password = utils::hash_password(&data.password)?;

        Ok(diesel::insert_into(users::table)
            .values(&data)
            .get_result(&connection)?)
    }

    fn login(context: &Context, data: NewUser) -> FieldResult<String> {
        let connection = context.pool.get().expect("bla");
        println!("{:?}", data);

        let result: Vec<String> = users::table
            .select(users::password)
            // TODO: Is there a better way than filter? I think this could be slow
            // with many users
            .filter(users::dsl::email.eq(data.email))
            .load(&connection)
            .expect("stuff");

        if result.len() == 0 {
            return Err(juniper::FieldError::new(
                "User not found",
                graphql_value!({ "message": "User not found" }),
            ));
        }

        let hash = result[0].clone();
        let result = utils::verify_hash(&hash, &data.password)?;

        if !result {
            return Err(juniper::FieldError::new(
                "Login not succesfull",
                graphql_value!({ "message": "Wrong credentials" }),
            ));
        }

        // TODO: Generate token
        Ok("123".to_string())
    }

    fn create_job(context: &Context, data: NewJob) -> FieldResult<Job> {
        let connection = context.pool.get()?;
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

        Ok(diesel::insert_into(jobs::table)
            .values(&data)
            .get_result(&connection)?)
    }

    fn create_secret(context: &Context, data: NewSecret) -> FieldResult<Secret> {
        let connection = context.pool.get()?;

        Ok(diesel::insert_into(secrets::table)
            .values(&data)
            .get_result(&connection)?)
    }

    fn update_user(context: &Context, id: i32, data: UpdatedUser) -> FieldResult<User> {
        let connection = context.pool.get()?;

        Ok(diesel::update(users::dsl::users.find(id))
            .set(&data)
            .get_result(&connection)?)
    }

    fn update_job(context: &Context, id: i32, data: UpdadedJob) -> FieldResult<Job> {
        let connection = context.pool.get()?;

        let last_run = utils::get_current_timestamp();
        let next_run = utils::get_next_run(&data.schedule);

        // TODO: is there a better way to merge these structs?
        let data = UpdadedJobWithRuns {
            schedule: data.schedule,
            command: data.command,
            last_run,
            next_run,
        };

        Ok(diesel::update(jobs::dsl::jobs.find(id))
            .set(&data)
            .get_result(&connection)?)
    }

    fn update_secret(context: &Context, id: i32, data: UpdatedSecret) -> FieldResult<Secret> {
        let connection = context.pool.get()?;

        Ok(diesel::update(secrets::dsl::secrets.find(id))
            .set(&data)
            .get_result(&connection)?)
    }

    fn delete_user(context: &Context, id: i32) -> FieldResult<bool> {
        let connection = context.pool.get()?;
        let num_deleted = diesel::delete(users::dsl::users.find(id)).execute(&connection)?;
        Ok(num_deleted != 0)
    }

    fn delete_job(context: &Context, id: i32) -> FieldResult<bool> {
        let connection = context.pool.get()?;
        let num_deleted = diesel::delete(jobs::dsl::jobs.find(id)).execute(&connection)?;
        Ok(num_deleted != 0)
    }

    fn delete_secret(context: &Context, id: i32) -> FieldResult<bool> {
        let connection = context.pool.get()?;
        let num_deleted = diesel::delete(secrets::dsl::secrets.find(id)).execute(&connection)?;
        Ok(num_deleted != 0)
    }
}

// modules
use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use juniper;

// internal
use crate::models::job::Job;
use crate::schema::jobs;
use crate::schema::users;
use crate::Context;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(juniper::GraphQLInputObject, Insertable, Debug)]
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

#[juniper::object(
    description = "A user",
    Context = Context,
)]
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

    pub fn jobs(&self, context: &Context) -> Vec<Job> {
        let connection = context.pool.get().expect("Expected a connection");

        jobs::dsl::jobs
            .filter(jobs::dsl::user_id.eq(self.id))
            .load::<Job>(&connection)
            .expect("Error loading jobs for user")
    }
}

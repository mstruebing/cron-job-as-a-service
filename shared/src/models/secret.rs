use crate::schema::secrets;
use diesel::{AsChangeset, Insertable, Queryable};

#[derive(Queryable, Debug)]
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

impl Secret {
    pub fn get_as_string(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

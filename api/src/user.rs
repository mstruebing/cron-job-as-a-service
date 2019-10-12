use postgres::Error;

use crate::job;
use shared::database;
use shared::model::user::User;

pub fn delete(user: User) -> Result<(), Error> {
    match user.id {
        Some(id) => {
            let connection = database::connection()?;
            connection.execute("DELETE FROM users WHERE id = $1", &[&id])?;
            Ok(())
        }
        None => Ok(()),
    }
}

pub fn save(mut user: User) -> Result<User, Error> {
    let connection = database::connection()?;

    for row in &connection.query(
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id;",
        &[&user.email, &user.password],
    )? {
        let id: i32 = row.get(0);
        user.id = Some(id);
    }

    for (index, job) in user.jobs.clone().iter().enumerate() {
        user.jobs[index] = job::save(job.clone(), user.id.unwrap())?;
    }

    Ok(user)
}

pub fn update(mut user: User) -> Result<User, Error> {
    let connection = database::connection()?;

    let query = "UPDATE users SET (email, password) = ($1, $2) WHERE id = $3;";
    connection.execute(query, &[&user.email, &user.password, &user.id.unwrap()])?;

    for (index, job) in user.jobs.clone().iter().enumerate() {
        user.jobs[index] = job::save(job.clone(), user.id.unwrap())?;
    }

    Ok(user)
}

// internal
use crate::database;
use crate::error::Result;
use crate::model::user::User;
use crate::repository::job;

pub fn delete(user: User) -> Result<()> {
    match user.id {
        Some(id) => {
            let connection = database::connection()?;
            connection.execute("DELETE FROM users WHERE id = $1", &[&id])?;
            Ok(())
        }
        None => Ok(()),
    }
}

pub fn save(mut user: User) -> Result<User> {
    let connection = database::connection()?;

    for row in &connection.query(
        "INSERT INTO users (email, password)
        VALUES ($1, $2)
        RETURNING id;",
        &[&user.email, &user.password],
    )? {
        let id: i32 = row.get(0);
        user.id = Some(id);
    }

    // convert iterator over results to result of an iterator. if a single element in the iterator
    // is an `Err`, the result will be an `Err`
    let jobs: Result<Vec<_>, _> = user
        .jobs
        .iter()
        .map(|job| job::save(job.clone(), user.id.unwrap()))
        .collect();
    user.jobs = jobs?;

    Ok(user)
}

pub fn update(mut user: User) -> Result<User> {
    let connection = database::connection()?;

    connection.execute(
        "UPDATE users SET (email, password) = ($1, $2) WHERE id = $3;",
        &[&user.email, &user.password, &user.id.unwrap()],
    )?;

    let jobs: Result<Vec<_>, _> = user
        .jobs
        .iter()
        .map(|job| job::save(job.clone(), user.id.unwrap()))
        .collect();
    user.jobs = jobs?;

    Ok(user)
}

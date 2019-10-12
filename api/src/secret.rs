use postgres::Error;

use shared::database;
use shared::model::secret::Secret;

pub fn save(mut secret: Secret, job_id: i32) -> Result<Secret, Error> {
    let connection = database::connection()?;
    let query = "INSERT INTO secrets (job_id, key, value) VALUES ($1, $2, $3) RETURNING id";
    let rows = connection.query(query, &[&job_id, &secret.key, &secret.value])?;

    for row in rows.iter() {
        let id: i32 = row.get(0);
        secret.id = Some(id);
    }

    Ok(secret)
}

pub fn update(secret: Secret, job_id: i32) -> Result<Secret, Error> {
    let connection = database::connection()?;

    let query = "UPDATE secrets SET job_id = $1, key = $2, value = $3 WHERE id = $4;";
    connection.execute(
        query,
        &[&job_id, &secret.key, &secret.value, &secret.id.unwrap()],
    )?;

    Ok(secret)
}

pub fn delete(secret: Secret) -> Result<(), Error> {
    match secret.id {
        Some(id) => {
            let connection = database::connection()?;
            connection.execute("DELETE FROM secrets WHERE id = $1", &[&id])?;
            Ok(())
        }
        None => Ok(()),
    }
}

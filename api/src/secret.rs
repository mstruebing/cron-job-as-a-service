// own modules
use shared::database;
use shared::error::Result;
use shared::model::secret::Secret;

pub fn save(mut secret: Secret, job_id: i32) -> Result<Secret> {
    let connection = database::connection()?;
    for row in &connection.query(
        "INSERT INTO secrets (job_id, key, value)
        VALUES ($1, $2, $3)
        RETURNING id",
        &[&job_id, &secret.key, &secret.value],
    )? {
        let id: i32 = row.get(0);
        secret.id = Some(id);
    }

    Ok(secret)
}

pub fn update(secret: Secret, job_id: i32) -> Result<Secret> {
    let connection = database::connection()?;

    connection.execute(
        "UPDATE secrets SET job_id = $1, key = $2, value = $3 WHERE id = $4;",
        &[&job_id, &secret.key, &secret.value, &secret.id.unwrap()],
    )?;

    Ok(secret)
}

pub fn delete(secret: Secret) -> Result<()> {
    match secret.id {
        Some(id) => {
            let connection = database::connection()?;
            connection.execute("DELETE FROM secrets WHERE id = $1", &[&id])?;
            Ok(())
        }
        None => Ok(()),
    }
}

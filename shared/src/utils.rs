// stdlib
use std::str::FromStr;
use std::time;

// modules
use chrono::Utc;
use cron::Schedule;
use postgres::rows::Row;

// internal
use crate::database;
use crate::error::Result;
use crate::model::job::Job;
use crate::model::secret::Secret;

pub fn get_current_timestamp() -> i32 {
    let start = time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(time::UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() as i32
}

pub fn get_next_run(schedule: &str) -> i32 {
    let schedule = &transform_to_modified_cron_format(schedule);
    let schedule = Schedule::from_str(schedule).unwrap();

    match schedule.upcoming(Utc).take(1).next() {
        Some(d) => d.timestamp() as i32,
        None => 0,
    }
}

// sec   min    hour   day of month    month of year   day of week   year
//                                  to
//       min    hour   day of month    month of year   day of week
pub fn transform_to_original_cron_format(schedule: &str) -> String {
    let schedule_parts: Vec<&str> = schedule.split(' ').collect();
    let mut transformed = "".to_owned();

    for (index, schedule_part) in schedule_parts.iter().enumerate() {
        if index == 0 || index == schedule_parts.len() - 1 {
            continue;
        }

        transformed.push_str(schedule_part);
    }

    transformed
}

//       min    hour   day of month    month of year   day of week
//                                  to
// sec   min    hour   day of month    month of year   day of week   year
pub fn transform_to_modified_cron_format(schedule: &str) -> String {
    let mut transformed = "0 ".to_owned();
    transformed.push_str(schedule);
    transformed.push_str(" *");
    transformed
}

pub fn convert_row_to_job(row: Row) -> Job {
    let id: Option<i32> = Some(row.get("id"));
    let command: String = row.get("command");
    let schedule: String = row.get("schedule");
    let next_run: i32 = row.get("next_run");
    let last_run: i32 = row.get("last_run");

    Job::new()
        .id(id)
        .command(&command)
        .next_run(next_run)
        .last_run(last_run)
        .schedule(&schedule)
}

pub fn convert_row_to_secret(row: Row) -> Secret {
    let key: String = row.get("key");
    let value: String = row.get("value");
    let id: Option<i32> = Some(row.get("id"));

    Secret::new().id(id).key(&key).value(&value)
}

pub fn get_secrets_for_job(job: &Job) -> Result<Vec<Secret>> {
    let connection = database::connection()?;
    let mut secrets = vec![];

    for row in &connection.query(
        "SELECT id, key, value
        FROM secrets
        WHERE job_id = $1",
        &[&job.id.unwrap()],
    )? {
        secrets.push(convert_row_to_secret(row));
    }

    Ok(secrets)
}

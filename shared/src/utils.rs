extern crate chrono;
extern crate cron;

use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp() -> i32 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
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

// stdlib
use std::{process::Command, str::FromStr, time};

// modules
use chrono::Utc;
use cron::Schedule;

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
    let mut transformed = String::new();

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
    format!("0 {} *", schedule)
}

pub fn is_installed(command: &str) -> bool {
    Command::new(command).output().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_installed() {
        assert_eq!(is_installed("docker"), true);
        assert_eq!(is_installed("ddoocckkeerr"), false);

        // to have one command which fails with an exit code != 0 with no arguments
        assert_eq!(is_installed("grep"), true);
    }
}

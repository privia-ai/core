use std::io;
use std::io::{stdout, Write};
use std::time::Duration;
use candid::types::TypeInner::Principal;
use chrono::{DateTime, Local, TimeDelta};
use regex::Regex;

pub fn nanos_to_localtime(nanos: u64) -> DateTime<Local> {
    let secs = (nanos / 1_000_000_000) as i64;
    let sub_nanos = (nanos % 1_000_000_000) as u32;

    let utc = DateTime::from_timestamp(secs, sub_nanos).unwrap();
    let local = utc.with_timezone(&Local);

    local
}

pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

pub fn console_log(msg: String) {
    let now = Local::now().format("%H:%M:%S").to_string();
    println!("{}: {}", now, msg);
}

pub fn press_enter(text: Option<&str>) {
    if let Some(text) = text {
        println!("{}", text);
    }
    else {
        println!("\nPress ENTER to continue...");
    }
    stdout().flush().unwrap(); // make sure it's printed

    let _ = io::stdin().read_line(&mut String::new()).unwrap();
}

pub async fn sleep_until(wake_up_time: &DateTime<Local>) {
    let now = Local::now();
    let duration = *wake_up_time - now;

    if duration < TimeDelta::seconds(0) {
        panic!("cannot sleep until past");
    }

    println!("\nsleep until {}...\n", wake_up_time.format("%H:%M:%S"));

    let duration_secs = duration.num_milliseconds();
    let duration = Duration::from_millis( duration_secs as u64);
    tokio::time::sleep(duration).await;
}

pub async fn sleep_seconds(seconds: u64) {
    let sleep_duration = Duration::from_secs(seconds);
    console_log(format!("sleep for {} seconds...", sleep_duration.as_secs()));
    tokio::time::sleep(sleep_duration).await;
}

#[allow(unused_variables)]
pub fn pretty_print_reject(error_text: &str) {
    let canister_re = Regex::new(r"Canister (?:called )?([a-z0-9-]+)").unwrap();
    let message_re = Regex::new(r"Panicked at '([^']+)'").unwrap();
    let location_re = Regex::new(r"([\w/.\-]+\.rs:\d+:\d+)").unwrap();

    let canister_id = canister_re
        .captures(error_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap();
    let message = message_re
        .captures(error_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap();
    let location = location_re
        .captures(error_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap();

    println!("message: {message}");
    // println!("canister_id: {canister_id}");
    // println!("location: {location}");
}
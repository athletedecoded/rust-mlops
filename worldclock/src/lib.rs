use chrono::{Timelike, Utc};
use chrono_tz::Tz;

// Convert the current date and time to a specified timezone date and time
pub fn get_time(tz_code: &str) -> String {
    let tz_str = match tz_code {
        "PST" => "America/Los_Angeles",
        "EST" => "America/New_York",
        "AEST" => "Australia/Sydney",
        "MST" => "America/Denver",
        _ => "UTC",
    };
    let tz: Tz = tz_str.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    let (is_pm, hour) = now.hour12();

    // Return format string of date and time
    format!(
        "It is {} {}:{}:{} {} in {}",
        now.format("%d %B %Y"),
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" },
        if tz_str == "UTC" { tz_str } else { tz_code }
    )
}

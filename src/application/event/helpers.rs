use chrono::{Duration, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

pub fn today_midnight(timezone: &String) -> NaiveDateTime {
    let tz: Tz = timezone.clone().parse().unwrap();
    let utc_now = Utc::now().naive_utc();
    tz.from_utc_datetime(&utc_now)
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
        .naive_utc()
}

pub fn tomorrow_midnight(timezone: &String) -> NaiveDateTime {
    today_midnight(timezone) + Duration::hours(24)
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn today_midnight_test() {
        assert_eq!(today_midnight(&String::from("Africa/Ndjamena")).hour(), 23)
    }
}

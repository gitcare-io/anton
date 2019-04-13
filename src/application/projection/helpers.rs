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

pub fn date_midnight(date: &NaiveDateTime, timezone: &String) -> NaiveDateTime {
    let tz: Tz = timezone.clone().parse().unwrap();
    tz.from_utc_datetime(date)
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

// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn today_midnight_test() {
        assert_eq!(today_midnight(&String::from("Africa/Ndjamena")).hour(), 23)
    }

    #[test]
    fn date_midnight_test() {
        let date = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
        assert_eq!(
            "2016-07-07 22:00:00",
            date_midnight(&date, &String::from("Europe/Warsaw")).to_string()
        );
    }
}

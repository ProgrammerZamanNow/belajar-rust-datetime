fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};
    use chrono_tz::Asia;

    #[test]
    fn test_date(){
        let date: NaiveDate = NaiveDate::from_ymd_opt(2024, 10, 25).unwrap();

        println!("{}", date.year());
        println!("{}", date.month());
        println!("{}", date.day());
    }

    #[test]
    fn test_duration(){
        let date: NaiveDate = NaiveDate::from_ymd_opt(2024, 10, 25).unwrap();
        let duration: Duration = Duration::days(10);
        let new_date: NaiveDate = date - duration;

        println!("{}", date);
        println!("{}", new_date);
    }

    #[test]
    fn test_time(){
        let time: NaiveTime = NaiveTime::from_hms_milli_opt(10, 10, 10, 500).unwrap();

        println!("{}", time.hour());
        println!("{}", time.minute());
        println!("{}", time.second());
        println!("{}", time.nanosecond());
    }

    #[test]
    fn test_date_time(){
        let date_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 10, 10).unwrap(),
            NaiveTime::from_hms_opt(12, 12, 12).unwrap()
        );

        println!("{}", date_time.date().year());
        println!("{}", date_time.date().month());
        println!("{}", date_time.date().day());
        println!("{}", date_time.time().hour());
        println!("{}", date_time.time().minute());
        println!("{}", date_time.time().second());
        println!("{}", date_time.time().nanosecond());
    }

    #[test]
    fn test_offset(){
        let utc_date_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 10, 30).unwrap(),
            NaiveTime::from_hms_opt(10, 30, 0).unwrap()
        );

        let asia_jakarta: FixedOffset = FixedOffset::east_opt(7 * 3600).unwrap();
        let asia_jakarta_date_time = asia_jakarta.from_utc_datetime(&utc_date_time);

        println!("{}", utc_date_time);
        println!("{}", asia_jakarta_date_time);
    }

    #[test]
    fn test_time_zone(){
        let utc_date_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 10, 30).unwrap(),
            NaiveTime::from_hms_opt(10, 30, 0).unwrap()
        );

        let asia_jakarta = Asia::Jakarta;
        let asia_jakarta_date_time = asia_jakarta.from_utc_datetime(&utc_date_time);

        println!("{}", utc_date_time);
        println!("{}", asia_jakarta_date_time);
    }
}
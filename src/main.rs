fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
    use chrono_tz::{Asia, Tz};

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

    #[test]
    fn test_date_time_with_time_zone(){
        let utc_date_time: DateTime<Utc> = Utc::now();
        let asia_jakarta_date_time : DateTime<Tz> = Asia::Jakarta.from_utc_datetime(&utc_date_time.naive_utc());

        println!("{}", utc_date_time);
        println!("{}", asia_jakarta_date_time);

        let local_date_time: DateTime<Local> = Local::now();
        let asia_jakarta_date_time: DateTime<Tz> = Asia::Jakarta.from_local_datetime(&local_date_time.naive_local()).unwrap();

        println!("{}", local_date_time);
        println!("{}", asia_jakarta_date_time);
    }

    #[test]
    fn test_parsing(){
        let string = String::from("2024-10-25 10:09:08 +0700");
        let time = DateTime::parse_from_str(&string, "%Y-%m-%d %H:%M:%S %z").unwrap();

        println!("{}", time.year());
        println!("{}", time.month());
        println!("{}", time.day());
        println!("{}", time.hour());
        println!("{}", time.minute());
        println!("{}", time.second());
        println!("{}", time.nanosecond());
        println!("{}", time.timezone());
    }

    #[test]
    fn test_format(){
        let time = Local::now();
        let formatted = time.format("%Y-%m-%d %H:%M:%S %z").to_string();

        println!("{}", formatted);
    }
}
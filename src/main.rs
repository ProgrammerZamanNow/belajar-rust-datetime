fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Timelike};

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
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDate};

    #[test]
    fn test_date(){
        let date: NaiveDate = NaiveDate::from_ymd_opt(2024, 10, 25).unwrap();

        println!("{}", date.year());
        println!("{}", date.month());
        println!("{}", date.day());
    }
}
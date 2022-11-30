use chrono::{DateTime, NaiveDate, TimeZone, Utc};
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let inputs = &args[1..];
        for input in inputs {
            match input.parse::<i64>() {
                Ok(unix_timestamp) => {
                    println!("{}", to_date(unix_timestamp).format("%Y-%m-%d %H:%M:%S %z"))
                }
                Err(_) => println!("{}", parse_date(&input).timestamp_millis()),
            }
        }
    } else {
        println!("{}", Utc::now().timestamp_millis());
    }
}

fn parse_date(date_str: &str) -> DateTime<Utc> {
    const SUPPORTED_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    const SUPPORTED_DATE_FORMAT: &str = "%Y-%m-%d";
    let datetime_parse_res = Utc.datetime_from_str(date_str, SUPPORTED_DATETIME_FORMAT);
    let date_parse_res = NaiveDate::parse_from_str(date_str, SUPPORTED_DATE_FORMAT)
        .map(|date| Utc.from_utc_date(&date))
        .map(|date| date.and_hms(0, 0, 0));
    datetime_parse_res.or(date_parse_res).expect(
        format!(
            "Either {} or {} time format should be used!",
            SUPPORTED_DATETIME_FORMAT,
            SUPPORTED_DATE_FORMAT
        )
        .as_str(),
    )
}

fn to_date(unix_timestamp: i64) -> DateTime<Utc> {
    let max_epoch_seconds = Utc.ymd(2099, 12, 31).and_hms(0, 0, 0).timestamp();

    if unix_timestamp > max_epoch_seconds {
        // Parse as epoch millis
        Utc.timestamp_millis(unix_timestamp)
    } else {
        Utc.timestamp(unix_timestamp, 0)
    }
}

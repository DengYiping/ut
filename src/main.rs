use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};
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
    // Try parsing relative time expressions first
    if let Some(datetime) = parse_relative_time(date_str) {
        return datetime;
    }

    // Fall back to absolute date parsing
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

fn parse_relative_time(input: &str) -> Option<DateTime<Utc>> {
    let input = input.trim();
    
    // Check if it starts with '-' for past time
    if !input.starts_with('-') {
        return None;
    }
    
    let input = &input[1..]; // Remove the '-' prefix
    
    if input.is_empty() {
        return None;
    }
    
    // Extract the numeric part and unit
    let (number_str, unit) = if let Some(last_char) = input.chars().last() {
        match last_char {
            'd' | 'h' | 'm' | 's' => {
                let number_part = &input[..input.len() - 1];
                (number_part, last_char)
            }
            _ => return None,
        }
    } else {
        return None;
    };
    
    // Parse the number
    let number: i64 = number_str.parse().ok()?;
    
    if number < 0 {
        return None; // Don't allow double negatives
    }
    
    // Calculate the duration to subtract
    let duration = match unit {
        'd' => Duration::days(number),
        'h' => Duration::hours(number),
        'm' => Duration::minutes(number),
        's' => Duration::seconds(number),
        _ => return None,
    };
    
    // Return current time minus the duration
    Some(Utc::now() - duration)
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_parse_relative_time_days() {
        let result = parse_relative_time("-7d");
        assert!(result.is_some());
        let parsed = result.unwrap();
        let expected = Utc::now() - Duration::days(7);
        // Allow for small time differences due to execution time
        assert!((parsed.timestamp() - expected.timestamp()).abs() < 2);
    }

    #[test]
    fn test_parse_relative_time_hours() {
        let result = parse_relative_time("-1h");
        assert!(result.is_some());
        let parsed = result.unwrap();
        let expected = Utc::now() - Duration::hours(1);
        assert!((parsed.timestamp() - expected.timestamp()).abs() < 2);
    }

    #[test]
    fn test_parse_relative_time_minutes() {
        let result = parse_relative_time("-30m");
        assert!(result.is_some());
        let parsed = result.unwrap();
        let expected = Utc::now() - Duration::minutes(30);
        assert!((parsed.timestamp() - expected.timestamp()).abs() < 2);
    }

    #[test]
    fn test_parse_relative_time_seconds() {
        let result = parse_relative_time("-45s");
        assert!(result.is_some());
        let parsed = result.unwrap();
        let expected = Utc::now() - Duration::seconds(45);
        assert!((parsed.timestamp() - expected.timestamp()).abs() < 2);
    }

    #[test]
    fn test_parse_relative_time_invalid_formats() {
        assert!(parse_relative_time("7d").is_none()); // Missing minus
        assert!(parse_relative_time("-").is_none()); // Just minus
        assert!(parse_relative_time("-d").is_none()); // Missing number
        assert!(parse_relative_time("-7").is_none()); // Missing unit
        assert!(parse_relative_time("-7x").is_none()); // Invalid unit
        assert!(parse_relative_time("--7d").is_none()); // Double minus
        assert!(parse_relative_time("-abc7d").is_none()); // Invalid number
    }

    #[test]
    fn test_parse_relative_time_with_whitespace() {
        let result = parse_relative_time(" -7d ");
        assert!(result.is_some());
    }
}

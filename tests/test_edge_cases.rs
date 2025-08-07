use date_component::date_component::*;
use chrono::prelude::*;

#[test]
fn test_leap_year_calculation() {
    // leap year
    let from = Utc.with_ymd_and_hms(2020, 2, 28, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2020, 3, 1, 0, 0, 0).unwrap();

    let diff = calculate(&from, &to);
    assert_eq!(diff.interval_days, 2);
    assert_eq!(diff.invert, false);

    // compare with non leap year
    let from = Utc.with_ymd_and_hms(2023, 2, 28, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();

    let diff = calculate(&from, &to);
    assert_eq!(diff.interval_days, 1);
    assert_eq!(diff.invert, false);
}

#[test]
fn test_month_edge_cases() {
    // from big month to small month
    let from = Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap();
    let diff = calculate(&from, &to);
    assert_eq!(diff.interval_days, 1);

    // from small month to big month
    let from = Utc.with_ymd_and_hms(2023, 2, 28, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();
    let diff = calculate(&from, &to);
    assert_eq!(diff.interval_days, 1);

    // from big month to big month
    let from = Utc.with_ymd_and_hms(2023, 7, 31, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 8, 1, 0, 0, 0).unwrap();
    let diff = calculate(&from, &to);
    assert_eq!(diff.interval_days, 1);
}

#[test]
fn test_timezone_crossing_date_line() {
    use chrono_tz::Pacific::Auckland;
    use chrono_tz::America::Los_Angeles;

    // forward test
    let from = Auckland.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let to = Los_Angeles.with_ymd_and_hms(2022, 12, 31, 0, 0, 0).unwrap();

    let diff = calculate(&from, &to);
    assert!(diff.invert);
    assert!(diff.interval_hours > 0);

    // backward test
    let from = Los_Angeles.with_ymd_and_hms(2022, 12, 31, 0, 0, 0).unwrap();
    let to = Auckland.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

    let diff = calculate(&from, &to);
    assert!(!diff.invert);
    assert!(diff.interval_hours > 0);
}

#[test]
fn test_exact_one_year_period() {
    // non leap year
    let from = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

    let diff = calculate(&from, &to);
    assert_eq!(diff.year, 1);
    assert_eq!(diff.month, 0);
    assert_eq!(diff.day, 0);
    assert_eq!(diff.interval_days, 365);
    assert!(!diff.invert);

    // leap year
    let from = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();

    let diff = calculate(&from, &to);
    assert_eq!(diff.year, 1);
    assert_eq!(diff.month, 0);
    assert_eq!(diff.day, 0);
    assert_eq!(diff.interval_days, 366);
    assert!(!diff.invert);
}

#[test]
fn test_same_timestamp_different_timezone() {
    use chrono_tz::America::New_York;
    use chrono_tz::Europe::London;

    let t = 1672531200; // 2023-01-01 00:00:00 UTC
    let from = Utc.timestamp_opt(t, 0).unwrap().with_timezone(&New_York);
    let to = Utc.timestamp_opt(t, 0).unwrap().with_timezone(&London);

    let diff = calculate(&from, &to);
    assert_eq!(diff.year, 0);
    assert_eq!(diff.month, 0);
    assert_eq!(diff.day, 0);
    assert_eq!(diff.hour, 0);
    assert_eq!(diff.minute, 0);
    assert_eq!(diff.second, 0);
    assert_eq!(diff.interval_days, 0);
    assert!(!diff.invert);
}

#[test]
fn test_large_time_span() {
    let from = Utc.with_ymd_and_hms(1000, 1, 1, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(3000, 1, 1, 0, 0, 0).unwrap();
    let diff = calculate(&from, &to);
    assert_eq!(diff.year, 2000);
    assert_eq!(diff.month, 0);
    assert_eq!(diff.day, 0);
    assert!(!diff.invert);
}

#[test]
fn test_across_leap_day() {
    let from = Utc.with_ymd_and_hms(2024, 2, 28, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
    let diff = calculate(&from, &to);
    assert_eq!(diff.interval_days, 2);
    assert!(!diff.invert);
}

#[test]
fn test_same_start_and_end_date() {
    let from = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

    let diff = calculate(&from, &to);
    assert_eq!(diff.year, 0);
    assert_eq!(diff.month, 0);
    assert_eq!(diff.day, 0);
    assert_eq!(diff.hour, 0);
    assert_eq!(diff.minute, 0);
    assert_eq!(diff.second, 0);
    assert_eq!(diff.interval_days, 0);
    assert!(!diff.invert);
}

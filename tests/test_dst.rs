use date_component::date_component::*;
use chrono::prelude::*;
use chrono_tz::America::Los_Angeles;
use chrono_tz::Europe::Paris;
use chrono_tz::Australia::Sydney;

#[test]
fn test_difference_during_dst() {
    let start = Los_Angeles.with_ymd_and_hms(2022, 3, 14, 1, 30, 0).unwrap();
    let end = Los_Angeles.with_ymd_and_hms(2022, 3, 14, 3, 30, 0).unwrap();
    let diff = calculate(&start, &end);
    assert_eq!(diff.hour, 2);
}

#[test]
fn test_dst_transition_backward() {
    let start = Los_Angeles.with_ymd_and_hms(2022, 11, 6, 0, 30, 0).unwrap();
    let end = Los_Angeles.with_ymd_and_hms(2022, 11, 6, 2, 30, 0).unwrap();
    let diff = calculate(&start, &end);
    println!("{:?}", diff);
    assert_eq!(diff.hour, 3);  // Ensure that the difference is 3 hours due to the DST transition
}

fn assert_date_component_eq(actual: DateComponent, expected: DateComponent) {
    assert_eq!(actual.year, expected.year);
    assert_eq!(actual.month, expected.month);
    assert_eq!(actual.week, expected.week);
    assert_eq!(actual.modulo_days, expected.modulo_days);
    assert_eq!(actual.day, expected.day);
    assert_eq!(actual.hour, expected.hour);
    assert_eq!(actual.minute, expected.minute);
    assert_eq!(actual.second, expected.second);
    assert_eq!(actual.interval_seconds, expected.interval_seconds);
    assert_eq!(actual.interval_minutes, expected.interval_minutes);
    assert_eq!(actual.interval_hours, expected.interval_hours);
    assert_eq!(actual.interval_days, expected.interval_days);
    assert_eq!(actual.invert, expected.invert);
}

#[test]
fn test_dst_start_transition_los_angeles() {
    let before_dst_start = Los_Angeles.with_ymd_and_hms(2022, 3, 13, 1, 59, 59).unwrap();
    let after_dst_start = Los_Angeles.with_ymd_and_hms(2022, 3, 13, 3, 0, 0).unwrap();
    let diff = calculate(&before_dst_start, &after_dst_start);
    let expected = DateComponent {
        year: 0,
        month: 0,
        week: 0,
        modulo_days: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 1,
        interval_seconds: 1,
        interval_minutes: 0,
        interval_hours: 0,
        interval_days: 0,
        invert: false,
    };
    assert_date_component_eq(diff, expected);
}

#[test]
fn test_one_year_span_with_dst() {
    let start = Los_Angeles.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
    let end = Los_Angeles.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let diff = calculate(&start, &end);
    assert_eq!(diff.year, 1);
    assert_eq!(diff.month, 0);
    assert_eq!(diff.day, 0);
    assert_eq!(diff.hour, 0);
    assert_eq!(diff.minute, 0);
    assert_eq!(diff.second, 0);
    assert!(!diff.invert);
}

#[test]
fn test_dst_start_transition_sydney() {
    let before_dst_start = Sydney.with_ymd_and_hms(2022, 10, 2, 1, 59, 59).unwrap();
    let after_dst_start = Sydney.with_ymd_and_hms(2022, 10, 2, 3, 0, 0).unwrap();
    let diff = calculate(&before_dst_start, &after_dst_start);
    let expected = DateComponent {
        year: 0,
        month: 0,
        week: 0,
        modulo_days: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 1,
        interval_seconds: 1,
        interval_minutes: 0,
        interval_hours: 0,
        interval_days: 0,
        invert: false,
    };
    assert_date_component_eq(diff, expected);
}

#[test]
fn test_dst_end_transition_los_angeles() {
    let before_dst_end = Los_Angeles.with_ymd_and_hms(2022, 11, 6, 0, 59, 59).unwrap();
    let after_dst_end = Los_Angeles.with_ymd_and_hms(2022, 11, 6, 2, 0, 0).unwrap();
    let diff = calculate(&before_dst_end, &after_dst_end);
    let expected = DateComponent {
        year: 0,
        month: 0,
        week: 0,
        modulo_days: 0,
        day: 0,
        hour: 2,
        minute: 0,
        second: 1,
        interval_seconds: 7201,
        interval_minutes: 120,
        interval_hours: 2,
        interval_days: 0,
        invert: false,
    };
    assert_date_component_eq(diff, expected);
}

#[test]
fn test_dst_start_transition_paris() {
    let before_dst_start = Paris.with_ymd_and_hms(2022, 3, 27, 1, 59, 59).unwrap();
    let after_dst_start = Paris.with_ymd_and_hms(2022, 3, 27, 3, 0, 0).unwrap();
    let diff = calculate(&before_dst_start, &after_dst_start);
    let expected = DateComponent {
        year: 0,
        month: 0,
        week: 0,
        modulo_days: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 1,
        interval_seconds: 1,
        interval_minutes: 0,
        interval_hours: 0,
        interval_days: 0,
        invert: false,
    };
    assert_date_component_eq(diff, expected);
}

#[test]
fn test_dst_end_transition_paris() {
    let before_dst_end = Paris.with_ymd_and_hms(2022, 10, 30, 3, 0, 0).unwrap();
    let after_dst_end = Paris.with_ymd_and_hms(2022, 10, 30, 1, 59, 59).unwrap();
    let diff = calculate(&before_dst_end, &after_dst_end);
    let expected = DateComponent {
        year: 0,
        month: 0,
        week: 0,
        modulo_days: 0,
        day: 0,
        hour: 2,
        minute: 0,
        second: 1,
        interval_seconds: 7201,
        interval_minutes: 120,
        interval_hours: 2,
        interval_days: 0,
        invert: true,
    };
    assert_date_component_eq(diff, expected);
}

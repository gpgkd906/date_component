use date_component::date_component::*;
use chrono::prelude::*;

#[test]
fn test_next_year_month_day_hour_minute_second() {
    let from = Utc.with_ymd_and_hms(2020, 1, 6, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2021, 2, 14, 1, 1, 1).unwrap();
    let duration = to.signed_duration_since(from);

    let sut = calculate(&from, &to);
    assert_eq!(
        sut,
        DateComponent {
            year: 1,
            month: 1,
            week: 1,
            modulo_days: 1,
            day: 8,
            hour: 1,
            minute: 1,
            second: 1,
            interval_days: duration.num_days().abs() as isize,
            interval_hours: duration.num_hours().abs() as isize,
            interval_minutes: duration.num_minutes().abs() as isize,
            interval_seconds: duration.num_seconds().abs() as isize,
            invert: false,
        }
    );
}

#[test]
fn test_previous_year_month_day_hour_minute_second() {
    let from = Utc.with_ymd_and_hms(2021, 2, 14, 1, 1, 1).unwrap();
    let to = Utc.with_ymd_and_hms(2020, 1, 6, 0, 0, 0).unwrap();
    let duration = to.signed_duration_since(from);

    let sut = calculate(&from, &to);
    assert_eq!(
        sut,
        DateComponent {
            year: 1,
            month: 1,
            week: 1,
            modulo_days: 1,
            day: 8,
            hour: 1,
            minute: 1,
            second: 1,
            interval_days: duration.num_days().abs() as isize,
            interval_hours: duration.num_hours().abs() as isize,
            interval_minutes: duration.num_minutes().abs() as isize,
            interval_seconds: duration.num_seconds().abs() as isize,
            invert: true,
        }
    );
}

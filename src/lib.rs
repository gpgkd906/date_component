pub mod date_component {
    use chrono::prelude::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DateComponent {
        /// Number of years.
        pub year: isize,
        /// Number of months.
        pub month: isize,
        /// Number of weeks.
        pub week: isize,
        /// Number of days remaining when using weeks.
        pub modulo_days: isize,
        /// Number of days.
        pub day: isize,
        /// Number of hours.
        pub hour: isize,
        /// Number of minutes.
        pub minute: isize,
        /// Number of seconds.
        pub second: isize,
        /// total number of seconds between the start and end dates.
        pub interval_seconds: isize,
        /// total number of minutes between the start and end dates.
        pub interval_minutes: isize,
        /// total number of hours between the start and end dates.
        pub interval_hours: isize,
        /// total number of days between the start and end dates
        pub interval_days: isize,
        /// Is true if the interval represents a negative time period and false otherwise
        pub invert: bool,
    }

    /// Returns a DateComponent object that represents the difference between the from and to datetime.
    pub fn calculate<T: chrono::TimeZone>(from_datetime: &DateTime<T>, to_datetime: &DateTime<T>) -> DateComponent {
        let timezone = from_datetime.timezone();
        let to_datetime_in_from_tz = to_datetime.with_timezone(&timezone);

        let duration = from_datetime.clone().signed_duration_since(to_datetime.clone());
        let seconds = duration.num_seconds();
        let (start, end, invert) = match seconds {
            x if x <= 0 => (from_datetime.clone(), to_datetime_in_from_tz, false),
            _ => (to_datetime_in_from_tz, from_datetime.clone(), true),
        };

        // Use mutable variables for interval components
        let mut year = end.year() as i64 - start.year() as i64;
        let mut month = end.month() as i64 - start.month() as i64;
        let mut day = end.day() as i64 - start.day() as i64;
        
        // For DST handling, we need to use duration for time components
        // instead of calculating differences directly
        let duration_hours = duration.num_hours().abs() % 24;
        let duration_minutes = duration.num_minutes().abs() % 60;
        let duration_seconds = duration.num_seconds().abs() % 60;

        // Now handle date borrowing (days -> months -> years)
        let (previous_year, previous_month) = if end.month() == 1 {
            (end.year() - 1, 12)
        } else {
            (end.year(), end.month() - 1)
        };

        if day < 0 {
            month -= 1;
            // Add days in the month *before* the end date's month.
            // Use get_nearest_day_before to find the last day of that month.
            let last_day_of_prev_month = get_nearest_day_before(
                previous_year,
                previous_month,
                31, // Try 31, it will be adjusted down correctly
                0, 0, 0, // Time doesn't matter for finding the last day
                &timezone
            );
            day += last_day_of_prev_month.day() as i64;
        }

        if month < 0 {
            month += 12;
            year -= 1;
        };

        // Calculate week and modulo_days based on the final adjusted day value
        let week = day / 7;
        let modulo_days = day % 7;

        // Return the final DateComponent
        DateComponent {
            year: year as isize,
            month: month as isize,
            week: week as isize,
            modulo_days: modulo_days as isize,
            day: day as isize,       // Store the final adjusted day count
            hour: duration_hours as isize,     // Use duration-based hours for DST handling
            minute: duration_minutes as isize, // Use duration-based minutes for DST handling
            second: duration_seconds as isize, // Use duration-based seconds for DST handling
            interval_seconds: duration.num_seconds().abs() as isize,
            interval_minutes: duration.num_minutes().abs() as isize,
            interval_hours: duration.num_hours().abs() as isize,
            interval_days: duration.num_days().abs() as isize,
            invert,
        }
    }

    /// Given date specified by year / month / day where the `day` may be invalid,
    /// (e.g. 2021-02-30), return the nearest valid day before it
    /// (e.g. 2021-02-28).
    fn get_nearest_day_before<T: TimeZone>(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        timezone: &T
    ) -> DateTime<T> {
        let mut subtract = 0;
        loop {
            match timezone.with_ymd_and_hms(year, month, day - subtract, hour, min, sec) {
                chrono::LocalResult::None => subtract += 1,
                chrono::LocalResult::Single(d) => {
                    return d;
                }
                chrono::LocalResult::Ambiguous(d, _) => {
                    return d;
                }
            }
        }
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn test_get_nearest_day_before_regular() {
        let dt = get_nearest_day_before(2023, 2, 30, 0, 0, 0, &Utc);
        assert_eq!(dt.day(), 28);
    }

    #[test]
    fn test_get_nearest_day_before_leap() {
        let dt = get_nearest_day_before(2024, 2, 30, 0, 0, 0, &Utc);
        assert_eq!(dt.day(), 29);
    }

    #[test]
    fn test_get_nearest_day_before_big_month() {
        let dt = get_nearest_day_before(2023, 1, 32, 0, 0, 0, &Utc);
        assert_eq!(dt.day(), 31);
    }
}

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
        let utc_from = from_datetime.with_timezone(&Utc);
        let utc_to = to_datetime.with_timezone(&Utc);
        
        let duration = utc_from.signed_duration_since(utc_to);
        let seconds = duration.num_seconds();
        let (start, end, invert) = match seconds {
            x if x <= 0 => (from_datetime, to_datetime, false),
            _ => (to_datetime, from_datetime, true),
        };

        let interval_year = end.year() as i64 - start.year() as i64;
        let interval_month = end.month() as i64 - start.month() as i64;
        let interval_day = end.day() as i64 - start.day() as i64;
        let interval_hour = end.hour() as i64 - start.hour() as i64;
        let interval_minute = end.minute() as i64 - start.minute() as i64;
        let interval_second = end.second() as i64 - start.second() as i64;

        // as with dst in some timezone，the duration may different with interval
        // so we need to use the duration in the later
        let duration_hours = duration.num_hours().abs() % 24;
        let duration_minutes = duration.num_minutes().abs() % 60;
        let duration_seconds = duration.num_seconds().abs() % 60;

        let (previous_year, previous_month) = if end.month() == 1 {
            (end.year() - 1, 12)
        } else {
            (end.year(), end.month() - 1)
        };

        let (interval_month, interval_day) = if interval_day < 0 {
            (
                interval_month - 1,
                get_nearest_day_before(
                    previous_year,
                    previous_month,
                    start.day(),
                    end.hour(),
                    end.minute(),
                    end.second(),
                    &timezone,
                )
                .signed_duration_since(end)
                .num_days()
                .abs(),
            )
        } else {
            (
                interval_month,
                timezone.with_ymd_and_hms(
                    end.year(),
                    end.month(),
                    start.day(),
                    end.hour(),
                    end.minute(),
                    end.second(),
                )
                .unwrap()
                .signed_duration_since(end)
                .num_days()
                .abs(),
            )
        };

        let (interval_year, interval_month) = if interval_month < 0 {
            (interval_year - 1, interval_month + 12)
        } else {
            (interval_year, interval_month)
        };

        let (interval_week, modulo_days) = if interval_day < 0 {
            (
                get_nearest_day_before(
                    previous_year,
                    previous_month,
                    start.day(),
                    end.hour(),
                    end.minute(),
                    end.second(),
                    &timezone,
                )
                .signed_duration_since(end)
                .num_days()
                .abs()
                    / 7,
                interval_day + 7,
            )
        } else {
            (interval_day / 7, interval_day % 7)
        };

        let (interval_hour, interval_minute) = if interval_hour < 0 {
            (
                get_nearest_day_before(
                    previous_year,
                    previous_month,
                    start.day(),
                    end.hour(),
                    end.minute(),
                    end.second(),
                    &timezone,
                )
                .signed_duration_since(end)
                .num_hours()
                .abs(),
                interval_minute - 1,
            )
        } else {
            (duration_hours, duration_minutes)
        };

        let (interval_minute, interval_second) = if interval_minute < 0 {
            (
                get_nearest_day_before(
                    previous_year,
                    previous_month,
                    start.day(),
                    end.hour(),
                    end.minute(),
                    end.second(),
                    &timezone,
                )
                .signed_duration_since(end)
                .num_minutes()
                .abs(),
                interval_second - 1,
            )
        } else {
            (duration_minutes, duration_seconds)
        };

        DateComponent {
            year: interval_year as isize,
            month: interval_month as isize,
            week: interval_week as isize,
            modulo_days: modulo_days as isize,
            day: interval_day as isize,
            hour: interval_hour as isize,
            minute: interval_minute as isize,
            second: interval_second as isize,
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
mod tests;
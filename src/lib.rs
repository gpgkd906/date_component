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
            (interval_hour, interval_minute)
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
            (interval_minute, interval_second)
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
mod tests {
    use super::date_component::*;
    use chrono::prelude::*;
    use test_case::test_case;
    use chrono_tz::US::Pacific;
    use chrono_tz::Asia::Shanghai;
    use chrono_tz::Asia::Tokyo;
    use chrono_tz::Europe::Paris;
    use chrono_tz::Pacific::Midway;
    use chrono_tz::Africa::Lome;


    #[test_case(1998, 1999; "world cup")]
    #[test_case(1999, 2000; "end of century")]
    #[test_case(2000, 2001; "start of century")]
    #[test_case(2009, 2010; "great recession")]
    fn test_next_year(year_start: i32, year_end: i32) {
        let from = Utc.with_ymd_and_hms(year_start, 1, 1, 0, 0, 0).unwrap();
        let to = Utc.with_ymd_and_hms(year_end, 1, 1, 0, 0, 0).unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.year, 1);
        assert_eq!(sut.invert, false);

        // with shared timezone
        let from = Pacific.with_ymd_and_hms(year_start, 1, 1, 0, 0, 0).unwrap();
        let to = Pacific.with_ymd_and_hms(year_end, 1, 1, 0, 0, 0).unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.year, 1);
        assert_eq!(sut.invert, false);
    }

    #[test_case(1999, 1998; "world cup")]
    #[test_case(2000, 1999; "end of century")]
    #[test_case(2001, 2000; "start of century")]
    #[test_case(2010, 2009; "great recession")]
    fn test_previous_year(year_start: i32, year_end: i32) {
        let from = Utc.with_ymd_and_hms(year_start, 1, 1, 0, 0, 0).unwrap();
        let to = Utc.with_ymd_and_hms(year_end, 1, 1, 0, 0, 0).unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.year, 1);
        assert_eq!(sut.invert, true);

        // with shared timezone
        let from = Shanghai.with_ymd_and_hms(year_start, 1, 1, 0, 0, 0).unwrap();
        let to = Shanghai.with_ymd_and_hms(year_end, 1, 1, 0, 0, 0).unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.year, 1);
        assert_eq!(sut.invert, true);
    }

    #[test_case(2020, 1, 2020, 2; "January to February")]
    #[test_case(2020, 2, 2020, 3; "February to March")]
    #[test_case(2020, 3, 2020, 4; "March to April")]
    #[test_case(2020, 4, 2020, 5; "April to May")]
    #[test_case(2020, 5, 2020, 6; "May to June")]
    #[test_case(2020, 6, 2020, 7; "June to July")]
    #[test_case(2020, 7, 2020, 8; "July to August")]
    #[test_case(2020, 8, 2020, 9; "August to September")]
    #[test_case(2020, 9, 2020, 10; "September to October")]
    #[test_case(2020, 10, 2020, 11; "October to November")]
    #[test_case(2020, 11, 2020, 12; "November to December")]
    #[test_case(2020, 12, 2021, 1; "December to January")]
    fn test_next_month(year_start: i32, month_start: u32, year_end: i32, month_end: u32) {
        let from = Utc
            .with_ymd_and_hms(year_start, month_start, 1, 0, 0, 0)
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, 1, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        println!("utf_result: {:?}", sut);
        assert_eq!(sut.month, 1);
        assert_eq!(sut.invert, false);

        // with shared timezone
        let from = Tokyo
            .with_ymd_and_hms(year_start, month_start, 1, 0, 0, 0)
            .unwrap();
        let to = Tokyo
            .with_ymd_and_hms(year_end, month_end, 1, 0, 0, 0)
            .unwrap();
        
        let sut = calculate(&from, &to);
        println!("tokyo_result: {:?}", sut);
        assert_eq!(sut.month, 1);
        assert_eq!(sut.invert, false);
    }

    #[test_case(2020, 2, 2020, 1; "February to January")]
    #[test_case(2020, 3, 2020, 2; "March to February")]
    #[test_case(2020, 4, 2020, 3; "April to March")]
    #[test_case(2020, 5, 2020, 4; "May to April")]
    #[test_case(2020, 6, 2020, 5; "June to May")]
    #[test_case(2020, 7, 2020, 6; "July to June")]
    #[test_case(2020, 8, 2020, 7; "August to July")]
    #[test_case(2020, 9, 2020, 8; "September to August")]
    #[test_case(2020, 10, 2020, 9; "October to September")]
    #[test_case(2020, 11, 2020, 10; "November to October")]
    #[test_case(2020, 12, 2020, 11; "December to November")]
    #[test_case(2021, 1, 2020, 12; "January to December")]
    fn test_previous_month(year_start: i32, month_start: u32, year_end: i32, month_end: u32) {
        let from = Utc
            .with_ymd_and_hms(year_start, month_start, 1, 0, 0, 0)
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, 1, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.month, 1);
        assert_eq!(sut.invert, true);

        // with shared timezone
        let from = Paris
            .with_ymd_and_hms(year_start, month_start, 1, 0, 0, 0)
            .unwrap();
        let to = Paris
            .with_ymd_and_hms(year_end, month_end, 1, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.month, 1);
        assert_eq!(sut.invert, true);
    }

    #[test_case(2019, 12, 30, 2020, 1, 6; "December 30 to January 6")]
    #[test_case(2020, 1, 6, 2020, 1, 13; "January 6 to January 13")]
    #[test_case(2020, 1, 13, 2020, 1, 20; "January 13 to January 20")]
    #[test_case(2020, 1, 20, 2020, 1, 27; "January 20 to January 27")]
    #[test_case(2020, 1, 27, 2020, 2, 3; "January 27 to February 3")]
    #[test_case(2020, 2, 3, 2020, 2, 10; "February 3 to February 10")]
    #[test_case(2020, 2, 10, 2020, 2, 17; "February 10 to February 17")]
    #[test_case(2020, 2, 17, 2020, 2, 24; "February 17 to February 24")]
    #[test_case(2020, 2, 24, 2020, 3, 2; "February 24 to March 2")]
    #[test_case(2020, 3, 2, 2020, 3, 9; "March 2 to March 9")]
    #[test_case(2020, 3, 9, 2020, 3, 16; "March 9 to March 16")]
    #[test_case(2020, 3, 16, 2020, 3, 23; "March 16 to March 23")]
    #[test_case(2020, 3, 23, 2020, 3, 30; "March 23 to March 30")]
    #[test_case(2020, 3, 30, 2020, 4, 6; "March 30 to April 6")]
    #[test_case(2020, 4, 6, 2020, 4, 13; "April 6 to April 13")]
    #[test_case(2020, 4, 13, 2020, 4, 20; "April 13 to April 20")]
    #[test_case(2020, 4, 20, 2020, 4, 27; "April 20 to April 27")]
    #[test_case(2020, 4, 27, 2020, 5, 4; "April 27 to May 4")]
    #[test_case(2020, 5, 4, 2020, 5, 11; "May 4 to May 11")]
    #[test_case(2020, 5, 11, 2020, 5, 18; "May 11 to May 18")]
    #[test_case(2020, 5, 18, 2020, 5, 25; "May 18 to May 25")]
    #[test_case(2020, 5, 25, 2020, 6, 1; "May 25 to June 1")]
    #[test_case(2020, 6, 1, 2020, 6, 8; "June 1 to June 8")]
    #[test_case(2020, 6, 8, 2020, 6, 15; "June 8 to June 15")]
    #[test_case(2020, 6, 15, 2020, 6, 22; "June 15 to June 22")]
    #[test_case(2020, 6, 22, 2020, 6, 29; "June 22 to June 29")]
    #[test_case(2020, 6, 29, 2020, 7, 6; "June 29 to July 6")]
    #[test_case(2020, 7, 6, 2020, 7, 13; "July 6 to July 13")]
    #[test_case(2020, 7, 13, 2020, 7, 20; "July 13 to July 20")]
    #[test_case(2020, 7, 20, 2020, 7, 27; "July 20 to July 27")]
    #[test_case(2020, 7, 27, 2020, 8, 3; "July 27 to August 3")]
    #[test_case(2020, 8, 3, 2020, 8, 10; "August 3 to August 10")]
    #[test_case(2020, 8, 10, 2020, 8, 17; "August 10 to August 17")]
    #[test_case(2020, 8, 17, 2020, 8, 24; "August 17 to August 24")]
    #[test_case(2020, 8, 24, 2020, 8, 31; "August 24 to August 31")]
    #[test_case(2020, 8, 31, 2020, 9, 7; "August 31 to September 7")]
    #[test_case(2020, 9, 7, 2020, 9, 14; "September 7 to September 14")]
    #[test_case(2020, 9, 14, 2020, 9, 21; "September 14 to September 21")]
    #[test_case(2020, 9, 21, 2020, 9, 28; "September 21 to September 28")]
    #[test_case(2020, 9, 28, 2020, 10, 5; "September 28 to October 5")]
    #[test_case(2020, 10, 5, 2020, 10, 12; "October 5 to October 12")]
    #[test_case(2020, 10, 12, 2020, 10, 19; "October 12 to October 19")]
    #[test_case(2020, 10, 19, 2020, 10, 26; "October 19 to October 26")]
    #[test_case(2020, 10, 26, 2020, 11, 2; "October 26 to November 2")]
    #[test_case(2020, 11, 2, 2020, 11, 9; "November 2 to November 9")]
    #[test_case(2020, 11, 9, 2020, 11, 16; "November 9 to November 16")]
    #[test_case(2020, 11, 16, 2020, 11, 23; "November 16 to November 23")]
    #[test_case(2020, 11, 23, 2020, 11, 30; "November 23 to November 30")]
    #[test_case(2020, 11, 30, 2020, 12, 7; "November 30 to December 7")]
    #[test_case(2020, 12, 7, 2020, 12, 14; "December 7 to December 14")]
    #[test_case(2020, 12, 14, 2020, 12, 21; "December 14 to December 21")]
    #[test_case(2020, 12, 21, 2020, 12, 28; "December 21 to December 28")]
    fn test_next_week(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(year_start, month_start, day_start, 0, 0, 0)
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, day_end, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.week, 1);
        assert_eq!(sut.invert, false);

        // with shared timezone
        let from = Midway
        .with_ymd_and_hms(year_start, month_start, day_start, 0, 0, 0)
        .unwrap();
        let to = Midway
            .with_ymd_and_hms(year_end, month_end, day_end, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.week, 1);
        assert_eq!(sut.invert, false);
    }

    #[test_case(2020, 12, 28, 2020, 12, 21; "December 28 to December 21")]
    #[test_case(2020, 12, 21, 2020, 12, 14; "December 21 to December 14")]
    #[test_case(2020, 12, 14, 2020, 12, 7; "December 14 to December 7")]
    #[test_case(2020, 12, 7, 2020, 11, 30; "December 7 to November 30")]
    #[test_case(2020, 11, 30, 2020, 11, 23; "November 30 to November 23")]
    #[test_case(2020, 11, 23, 2020, 11, 16; "November 23 to November 16")]
    #[test_case(2020, 11, 16, 2020, 11, 9; "November 16 to November 9")]
    #[test_case(2020, 11, 9, 2020, 11, 2; "November 9 to November 2")]
    #[test_case(2020, 11, 2, 2020, 10, 26; "November 2 to October 26")]
    #[test_case(2020, 10, 26, 2020, 10, 19; "October 26 to October 19")]
    #[test_case(2020, 10, 19, 2020, 10, 12; "October 19 to October 12")]
    #[test_case(2020, 10, 12, 2020, 10, 5; "October 12 to October 5")]
    #[test_case(2020, 10, 5, 2020, 9, 28; "October 5 to September 28")]
    #[test_case(2020, 9, 28, 2020, 9, 21; "September 28 to September 21")]
    #[test_case(2020, 9, 21, 2020, 9, 14; "September 21 to September 14")]
    #[test_case(2020, 9, 14, 2020, 9, 7; "September 14 to September 7")]
    #[test_case(2020, 9, 7, 2020, 8, 31; "September 7 to August 31")]
    #[test_case(2020, 8, 31, 2020, 8, 24; "August 31 to August 24")]
    #[test_case(2020, 8, 24, 2020, 8, 17; "August 24 to August 17")]
    #[test_case(2020, 8, 17, 2020, 8, 10; "August 17 to August 10")]
    #[test_case(2020, 8, 10, 2020, 8, 3; "August 10 to August 3")]
    #[test_case(2020, 8, 3, 2020, 7, 27; "August 3 to July 27")]
    #[test_case(2020, 7, 27, 2020, 7, 20; "July 27 to July 20")]
    #[test_case(2020, 7, 20, 2020, 7, 13; "July 20 to July 13")]
    #[test_case(2020, 7, 13, 2020, 7, 6; "July 13 to July 6")]
    #[test_case(2020, 7, 6, 2020, 6, 29; "July 6 to June 29")]
    #[test_case(2020, 6, 29, 2020, 6, 22; "June 29 to June 22")]
    #[test_case(2020, 6, 22, 2020, 6, 15; "June 22 to June 15")]
    #[test_case(2020, 6, 15, 2020, 6, 8; "June 15 to June 8")]
    #[test_case(2020, 6, 8, 2020, 6, 1; "June 8 to June 1")]
    #[test_case(2020, 6, 1, 2020, 5, 25; "June 1 to May 25")]
    #[test_case(2020, 5, 25, 2020, 5, 18; "May 25 to May 18")]
    #[test_case(2020, 5, 18, 2020, 5, 11; "May 18 to May 11")]
    #[test_case(2020, 5, 11, 2020, 5, 4; "May 11 to May 4")]
    #[test_case(2020, 5, 4, 2020, 4, 27; "May 4 to April 27")]
    #[test_case(2020, 4, 27, 2020, 4, 20; "April 27 to April 20")]
    #[test_case(2020, 4, 20, 2020, 4, 13; "April 20 to April 13")]
    #[test_case(2020, 4, 13, 2020, 4, 6; "April 13 to April 6")]
    #[test_case(2020, 4, 6, 2020, 3, 30; "April 6 to March 30")]
    #[test_case(2020, 3, 30, 2020, 3, 23; "March 30 to March 23")]
    #[test_case(2020, 3, 23, 2020, 3, 16; "March 23 to March 16")]
    #[test_case(2020, 3, 16, 2020, 3, 9; "March 16 to March 9")]
    #[test_case(2020, 3, 9, 2020, 3, 2; "March 9 to March 2")]
    #[test_case(2020, 3, 2, 2020, 2, 24; "March 2 to February 24")]
    #[test_case(2020, 2, 24, 2020, 2, 17; "February 24 to February 17")]
    #[test_case(2020, 2, 17, 2020, 2, 10; "February 17 to February 10")]
    #[test_case(2020, 2, 10, 2020, 2, 3; "February 10 to February 3")]
    #[test_case(2020, 2, 3, 2020, 1, 27; "February 3 to January 27")]
    #[test_case(2020, 1, 27, 2020, 1, 20; "January 27 to January 20")]
    #[test_case(2020, 1, 20, 2020, 1, 13; "January 20 to January 13")]
    #[test_case(2020, 1, 13, 2020, 1, 6; "January 13 to January 6")]
    #[test_case(2020, 1, 6, 2019, 12, 30; "January 6 to December 30")]
    fn test_previous_week(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(year_start, month_start, day_start, 0, 0, 0)
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, day_end, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.week, 1);
        assert_eq!(sut.invert, true);

        // with shared timezone
        let from = Lome
            .with_ymd_and_hms(year_start, month_start, day_start, 0, 0, 0)
            .unwrap();
        let to = Lome
            .with_ymd_and_hms(year_end, month_end, day_end, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.week, 1);
        assert_eq!(sut.invert, true);
    }

    #[test_case(2019, 12, 29, 2019, 12, 30; "Sunday to Monday")]
    #[test_case(2019, 12, 30, 2019, 12, 31; "Monday to Tuesday")]
    #[test_case(2019, 12, 31, 2020, 1, 1; "Tuesday to Wednesday")]
    #[test_case(2020, 1, 1, 2020, 1, 2; "Wednesday to Thursday")]
    #[test_case(2020, 1, 2, 2020, 1, 3; "Thursday to Friday")]
    #[test_case(2020, 1, 3, 2020, 1, 4; "Friday to Saturday")]
    #[test_case(2020, 1, 4, 2020, 1, 5; "Saturday to Sunday")]
    fn test_next_day(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(year_start, month_start, day_start, 0, 0, 0)
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, day_end, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.day, 1);
        assert_eq!(sut.invert, false);

        // with shared timezone
        let from = Shanghai
            .with_ymd_and_hms(year_start, month_start, day_start, 0, 0, 0)
            .unwrap();
        let to = Shanghai
            .with_ymd_and_hms(year_end, month_end, day_end, 0, 0, 0)
            .unwrap();
        let sut = calculate(&from, &to);
        assert_eq!(sut.day, 1);
        assert_eq!(sut.invert, false);
    }

    #[test_case(2020, 1, 5, 2020, 1, 4; "Sunday to Saturday")]
    #[test_case(2020, 1, 4, 2020, 1, 3; "Saturday to Friday")]
    #[test_case(2020, 1, 3, 2020, 1, 2; "Friday to Thursday")]
    #[test_case(2020, 1, 2, 2020, 1, 1; "Thursday to Wednesday")]
    #[test_case(2020, 1, 1, 2019, 12, 31; "Wednesday to Tuesday")]
    #[test_case(2019, 12, 31, 2019, 12, 30; "Tuesday to Monday")]
    #[test_case(2019, 12, 30, 2019, 12, 29; "Monday to Sunday")]
    fn test_previous_day(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(year_start, month_start, day_start, 0, 0, 0)
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, day_end, 0, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.day, 1);
        assert_eq!(sut.invert, true);

        // with shared timezone
        let from = Tokyo
            .with_ymd_and_hms(year_start, month_start, day_start, 0, 0, 0)
            .unwrap();
        let to = Tokyo
            .with_ymd_and_hms(year_end, month_end, day_end, 0, 0, 0)
            .unwrap();
        println!("from: {:?}, to: {:?}", from, to);
        println!("from: {:?}, to: {:?}", from.with_timezone(&Utc), to.with_timezone(&Utc));
        let sut = calculate(&from, &to);
        assert_eq!(sut.day, 1);
        assert_eq!(sut.invert, true);
    }

    #[test_case(2019, 12, 31, 23, 2020, 1, 1, 0; "New year's eve to midnight")]
    #[test_case(2020, 1, 1, 0, 2020, 1, 1, 1; "Midnight to new year's day")]
    #[test_case(2020, 1, 1, 2, 2020, 1, 1, 3; "Two to three at night")]
    #[test_case(2020, 1, 1, 3, 2020, 1, 1, 4; "Three to four at night")]
    #[test_case(2020, 1, 1, 4, 2020, 1, 1, 5; "Four to five at night")]
    #[test_case(2020, 1, 1, 5, 2020, 1, 1, 6; "Five to six in the morning")]
    #[test_case(2020, 1, 1, 6, 2020, 1, 1, 7; "Six to seven in the morning")]
    #[test_case(2020, 1, 1, 7, 2020, 1, 1, 8; "Seven to eight in the morning")]
    #[test_case(2020, 1, 1, 8, 2020, 1, 1, 9; "Eight to nine in the morning")]
    #[test_case(2020, 1, 1, 9, 2020, 1, 1, 10; "Nine to ten in the morning")]
    #[test_case(2020, 1, 1, 10, 2020, 1, 1, 11; "Ten to eleven in the morning")]
    #[test_case(2020, 1, 1, 11, 2020, 1, 1, 12; "Eleven to Noon")]
    #[test_case(2020, 1, 1, 12, 2020, 1, 1, 13; "Noon to one in the afternoon")]
    #[test_case(2020, 1, 1, 13, 2020, 1, 1, 14; "One to two in the afternoon")]
    #[test_case(2020, 1, 1, 14, 2020, 1, 1, 15; "Two to three in the afternoon")]
    #[test_case(2020, 1, 1, 15, 2020, 1, 1, 16; "Three to four in the afternoon")]
    #[test_case(2020, 1, 1, 16, 2020, 1, 1, 17; "Four to five in the afternoon")]
    #[test_case(2020, 1, 1, 17, 2020, 1, 1, 18; "Five to six in the evening")]
    #[test_case(2020, 1, 1, 18, 2020, 1, 1, 19; "Six to seven in the evening")]
    #[test_case(2020, 1, 1, 19, 2020, 1, 1, 20; "Seven to eight in the evening")]
    #[test_case(2020, 1, 1, 20, 2020, 1, 1, 21; "Eight to nine at night")]
    #[test_case(2020, 1, 1, 21, 2020, 1, 1, 22; "Nine to ten at night")]
    #[test_case(2020, 1, 1, 22, 2020, 1, 1, 23; "Ten to eleven at night")]
    fn test_next_hours(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        hour_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
        hour_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(year_start, month_start, day_start, hour_start, 0, 0)
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, day_end, hour_end, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_hours, 1);
        assert_eq!(sut.invert, false);

        // with shared timezone
        let from = Paris
            .with_ymd_and_hms(year_start, month_start, day_start, hour_start, 0, 0)
            .unwrap();
        let to = Paris
            .with_ymd_and_hms(year_end, month_end, day_end, hour_end, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_hours, 1);
        assert_eq!(sut.invert, false);
    }

    #[test_case(2020, 1, 1, 23, 2020, 1, 1, 22; "Eleve to ten at night")]
    #[test_case(2020, 1, 1, 22, 2020, 1, 1, 21; "Ten to nine at night")]
    #[test_case(2020, 1, 1, 21, 2020, 1, 1, 20; "Nine to eight at night")]
    #[test_case(2020, 1, 1, 20, 2020, 1, 1, 19; "Eight to seven in the evening")]
    #[test_case(2020, 1, 1, 19, 2020, 1, 1, 18; "Seven to six in the evening")]
    #[test_case(2020, 1, 1, 18, 2020, 1, 1, 17; "Six to five in the evening")]
    #[test_case(2020, 1, 1, 17, 2020, 1, 1, 16; "Five to four in the afternoon")]
    #[test_case(2020, 1, 1, 16, 2020, 1, 1, 15; "Four to three in the afternoon")]
    #[test_case(2020, 1, 1, 15, 2020, 1, 1, 14; "Three to two in the afternoon")]
    #[test_case(2020, 1, 1, 14, 2020, 1, 1, 13; "Two to one in the afternoon")]
    #[test_case(2020, 1, 1, 13, 2020, 1, 1, 12; "One to Noon")]
    #[test_case(2020, 1, 1, 12, 2020, 1, 1, 11; "Nooon to eleven in the morning")]
    #[test_case(2020, 1, 1, 11, 2020, 1, 1, 10; "Eleven to ten in the morning")]
    #[test_case(2020, 1, 1, 10, 2020, 1, 1, 9; "Ten to nine in the morning")]
    #[test_case(2020, 1, 1, 9, 2020, 1, 1, 8; "Nine to eight in the morning")]
    #[test_case(2020, 1, 1, 8, 2020, 1, 1, 7; "Eight to seven in the morning")]
    #[test_case(2020, 1, 1, 7, 2020, 1, 1, 6; "Seven to six in the morning")]
    #[test_case(2020, 1, 1, 6, 2020, 1, 1, 5; "Six to five in the morning")]
    #[test_case(2020, 1, 1, 5, 2020, 1, 1, 4; "Five to four at night")]
    #[test_case(2020, 1, 1, 4, 2020, 1, 1, 3; "Four to three at night")]
    #[test_case(2020, 1, 1, 3, 2020, 1, 1, 2; "Three to two at night")]
    #[test_case(2020, 1, 1, 2, 2020, 1, 1, 1; "Two to one at night")]
    #[test_case(2020, 1, 1, 1, 2020, 1, 1, 0; "Midnight to new year's day")]
    #[test_case(2020, 1, 1, 0, 2019, 12, 31, 23; "New year's eve to midnight")]
    fn test_previous_hours(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        hour_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
        hour_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(year_start, month_start, day_start, hour_start, 0, 0)
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, day_end, hour_end, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_hours, 1);
        assert_eq!(sut.invert, true);

        // with shared timezone
        let from = Midway
            .with_ymd_and_hms(year_start, month_start, day_start, hour_start, 0, 0)
            .unwrap();
        let to = Midway
            .with_ymd_and_hms(year_end, month_end, day_end, hour_end, 0, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_hours, 1);
        assert_eq!(sut.invert, true);
    }

    #[test_case(2019, 12, 31, 23, 30, 2019, 12, 31, 23, 31; "30 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 31, 2019, 12, 31, 23, 32; "29 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 32, 2019, 12, 31, 23, 33; "28 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 33, 2019, 12, 31, 23, 34; "27 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 34, 2019, 12, 31, 23, 35; "26 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 35, 2019, 12, 31, 23, 36; "25 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 36, 2019, 12, 31, 23, 37; "24 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 37, 2019, 12, 31, 23, 38; "23 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 38, 2019, 12, 31, 23, 39; "22 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 39, 2019, 12, 31, 23, 40; "21 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 40, 2019, 12, 31, 23, 41; "20 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 41, 2019, 12, 31, 23, 42; "19 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 42, 2019, 12, 31, 23, 43; "18 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 43, 2019, 12, 31, 23, 44; "17 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 44, 2019, 12, 31, 23, 45; "16 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 45, 2019, 12, 31, 23, 46; "15 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 46, 2019, 12, 31, 23, 47; "14 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 47, 2019, 12, 31, 23, 48; "13 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 48, 2019, 12, 31, 23, 49; "12 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 49, 2019, 12, 31, 23, 50; "11 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 50, 2019, 12, 31, 23, 51; "10 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 51, 2019, 12, 31, 23, 52; "09 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 52, 2019, 12, 31, 23, 53; "08 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 53, 2019, 12, 31, 23, 54; "07 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 54, 2019, 12, 31, 23, 55; "06 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 55, 2019, 12, 31, 23, 56; "05 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 56, 2019, 12, 31, 23, 57; "04 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 57, 2019, 12, 31, 23, 58; "03 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 58, 2019, 12, 31, 23, 59; "02 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 2020, 1, 1, 0, 0; "01 minute before midnight")]
    #[test_case(2020, 1, 1, 0, 0, 2020, 1, 1, 0, 1; "01 minute after midnight")]
    #[test_case(2020, 1, 1, 0, 1, 2020, 1, 1, 0, 2; "02 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 2, 2020, 1, 1, 0, 3; "03 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 3, 2020, 1, 1, 0, 4; "04 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 4, 2020, 1, 1, 0, 5; "05 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 5, 2020, 1, 1, 0, 6; "06 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 6, 2020, 1, 1, 0, 7; "07 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 7, 2020, 1, 1, 0, 8; "08 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 8, 2020, 1, 1, 0, 9; "09 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 9, 2020, 1, 1, 0, 10; "10 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 10, 2020, 1, 1, 0, 11; "11 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 11, 2020, 1, 1, 0, 12; "12 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 12, 2020, 1, 1, 0, 13; "13 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 13, 2020, 1, 1, 0, 14; "14 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 14, 2020, 1, 1, 0, 15; "15 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 15, 2020, 1, 1, 0, 16; "16 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 16, 2020, 1, 1, 0, 17; "17 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 17, 2020, 1, 1, 0, 18; "18 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 18, 2020, 1, 1, 0, 19; "19 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 19, 2020, 1, 1, 0, 20; "20 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 20, 2020, 1, 1, 0, 21; "21 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 21, 2020, 1, 1, 0, 22; "22 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 22, 2020, 1, 1, 0, 23; "23 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 23, 2020, 1, 1, 0, 24; "24 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 24, 2020, 1, 1, 0, 25; "25 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 25, 2020, 1, 1, 0, 26; "26 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 26, 2020, 1, 1, 0, 27; "27 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 27, 2020, 1, 1, 0, 28; "28 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 28, 2020, 1, 1, 0, 29; "29 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 29, 2020, 1, 1, 0, 30; "30 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 30, 2020, 1, 1, 0, 31; "31 minutes after midnight")]
    fn test_next_minutes(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        hour_start: u32,
        minute_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
        hour_end: u32,
        minute_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(
                year_start,
                month_start,
                day_start,
                hour_start,
                minute_start,
                0,
            )
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, day_end, hour_end, minute_end, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_minutes, 1);
        assert_eq!(sut.invert, false);

        // with shared timezone
        let from = Lome
            .with_ymd_and_hms(
                year_start,
                month_start,
                day_start,
                hour_start,
                minute_start,
                0,
            )
            .unwrap();
        let to = Lome
            .with_ymd_and_hms(year_end, month_end, day_end, hour_end, minute_end, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_minutes, 1);
        assert_eq!(sut.invert, false);
    }

    #[test_case(2020, 1, 1, 0, 30, 2020, 1, 1, 0, 29; "30 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 29, 2020, 1, 1, 0, 28; "29 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 28, 2020, 1, 1, 0, 27; "28 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 27, 2020, 1, 1, 0, 26; "27 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 26, 2020, 1, 1, 0, 25; "26 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 25, 2020, 1, 1, 0, 24; "25 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 24, 2020, 1, 1, 0, 23; "24 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 23, 2020, 1, 1, 0, 22; "23 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 22, 2020, 1, 1, 0, 21; "22 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 21, 2020, 1, 1, 0, 20; "21 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 20, 2020, 1, 1, 0, 19; "20 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 19, 2020, 1, 1, 0, 18; "19 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 18, 2020, 1, 1, 0, 17; "18 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 17, 2020, 1, 1, 0, 16; "17 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 16, 2020, 1, 1, 0, 15; "16 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 15, 2020, 1, 1, 0, 14; "15 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 14, 2020, 1, 1, 0, 13; "14 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 13, 2020, 1, 1, 0, 12; "13 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 12, 2020, 1, 1, 0, 11; "12 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 11, 2020, 1, 1, 0, 10; "11 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 10, 2020, 1, 1, 0, 9; "10 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 9, 2020, 1, 1, 0, 8; "9 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 8, 2020, 1, 1, 0, 7; "8 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 7, 2020, 1, 1, 0, 6; "7 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 6, 2020, 1, 1, 0, 5; "6 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 5, 2020, 1, 1, 0, 4; "5 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 4, 2020, 1, 1, 0, 3; "4 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 3, 2020, 1, 1, 0, 2; "3 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 2, 2020, 1, 1, 0, 1; "2 minutes after midnight")]
    #[test_case(2020, 1, 1, 0, 1, 2020, 1, 1, 0, 0; "1 minute after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 2019, 12, 31, 23, 59; "1 minute before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 2019, 12, 31, 23, 58; "2 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 58, 2019, 12, 31, 23, 57; "3 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 57, 2019, 12, 31, 23, 56; "4 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 56, 2019, 12, 31, 23, 55; "5 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 55, 2019, 12, 31, 23, 54; "6 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 54, 2019, 12, 31, 23, 53; "7 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 53, 2019, 12, 31, 23, 52; "8 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 52, 2019, 12, 31, 23, 51; "9 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 51, 2019, 12, 31, 23, 50; "10 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 50, 2019, 12, 31, 23, 49; "11 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 49, 2019, 12, 31, 23, 48; "12 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 48, 2019, 12, 31, 23, 47; "13 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 47, 2019, 12, 31, 23, 46; "14 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 46, 2019, 12, 31, 23, 45; "15 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 45, 2019, 12, 31, 23, 44; "16 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 44, 2019, 12, 31, 23, 43; "17 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 43, 2019, 12, 31, 23, 42; "18 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 42, 2019, 12, 31, 23, 41; "19 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 41, 2019, 12, 31, 23, 40; "20 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 40, 2019, 12, 31, 23, 39; "21 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 39, 2019, 12, 31, 23, 38; "22 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 38, 2019, 12, 31, 23, 37; "23 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 37, 2019, 12, 31, 23, 36; "24 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 36, 2019, 12, 31, 23, 35; "25 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 35, 2019, 12, 31, 23, 34; "26 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 34, 2019, 12, 31, 23, 33; "27 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 33, 2019, 12, 31, 23, 32; "28 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 32, 2019, 12, 31, 23, 31; "29 minutes before midnight")]
    #[test_case(2019, 12, 31, 23, 31, 2019, 12, 31, 23, 30; "30 minutes before midnight")]
    fn test_previous_minutes(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        hour_start: u32,
        minute_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
        hour_end: u32,
        minute_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(
                year_start,
                month_start,
                day_start,
                hour_start,
                minute_start,
                0,
            )
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(year_end, month_end, day_end, hour_end, minute_end, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_minutes, 1);
        assert_eq!(sut.invert, true);

        // with shared timezone
        let from = Pacific
            .with_ymd_and_hms(
                year_start,
                month_start,
                day_start,
                hour_start,
                minute_start,
                0,
            )
            .unwrap();
        let to = Pacific
            .with_ymd_and_hms(year_end, month_end, day_end, hour_end, minute_end, 0)
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_minutes, 1);
        assert_eq!(sut.invert, true);
    }

    #[test_case(2019, 12, 31, 23, 59, 30, 2019, 12, 31, 23, 59, 31; "30 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 31, 2019, 12, 31, 23, 59, 32; "29 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 32, 2019, 12, 31, 23, 59, 33; "28 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 33, 2019, 12, 31, 23, 59, 34; "27 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 34, 2019, 12, 31, 23, 59, 35; "26 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 35, 2019, 12, 31, 23, 59, 36; "25 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 36, 2019, 12, 31, 23, 59, 37; "24 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 37, 2019, 12, 31, 23, 59, 38; "23 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 38, 2019, 12, 31, 23, 59, 39; "22 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 39, 2019, 12, 31, 23, 59, 40; "21 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 40, 2019, 12, 31, 23, 59, 41; "20 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 41, 2019, 12, 31, 23, 59, 42; "19 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 42, 2019, 12, 31, 23, 59, 43; "18 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 43, 2019, 12, 31, 23, 59, 44; "17 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 44, 2019, 12, 31, 23, 59, 45; "16 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 45, 2019, 12, 31, 23, 59, 46; "15 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 46, 2019, 12, 31, 23, 59, 47; "14 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 47, 2019, 12, 31, 23, 59, 48; "13 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 48, 2019, 12, 31, 23, 59, 49; "12 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 49, 2019, 12, 31, 23, 59, 50; "11 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 50, 2019, 12, 31, 23, 59, 51; "10 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 51, 2019, 12, 31, 23, 59, 52; "9 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 52, 2019, 12, 31, 23, 59, 53; "8 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 53, 2019, 12, 31, 23, 59, 54; "7 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 54, 2019, 12, 31, 23, 59, 55; "6 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 55, 2019, 12, 31, 23, 59, 56; "5 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 56, 2019, 12, 31, 23, 59, 57; "4 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 57, 2019, 12, 31, 23, 59, 58; "3 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 58, 2019, 12, 31, 23, 59, 59; "2 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 59, 2020, 1, 1, 0, 0, 0; "1 second before midnight")]
    #[test_case(2020, 1, 1, 0, 0, 0, 2020, 1, 1, 0, 0, 1; "1 second after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 1, 2020, 1, 1, 0, 0, 2; "2 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 2, 2020, 1, 1, 0, 0, 3; "3 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 3, 2020, 1, 1, 0, 0, 4; "4 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 4, 2020, 1, 1, 0, 0, 5; "5 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 5, 2020, 1, 1, 0, 0, 6; "6 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 6, 2020, 1, 1, 0, 0, 7; "7 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 7, 2020, 1, 1, 0, 0, 8; "8 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 8, 2020, 1, 1, 0, 0, 9; "9 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 9, 2020, 1, 1, 0, 0, 10; "10 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 10, 2020, 1, 1, 0, 0, 11; "11 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 11, 2020, 1, 1, 0, 0, 12; "12 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 12, 2020, 1, 1, 0, 0, 13; "13 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 13, 2020, 1, 1, 0, 0, 14; "14 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 14, 2020, 1, 1, 0, 0, 15; "15 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 15, 2020, 1, 1, 0, 0, 16; "16 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 16, 2020, 1, 1, 0, 0, 17; "17 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 17, 2020, 1, 1, 0, 0, 18; "18 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 18, 2020, 1, 1, 0, 0, 19; "19 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 19, 2020, 1, 1, 0, 0, 20; "20 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 20, 2020, 1, 1, 0, 0, 21; "21 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 21, 2020, 1, 1, 0, 0, 22; "22 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 22, 2020, 1, 1, 0, 0, 23; "23 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 23, 2020, 1, 1, 0, 0, 24; "24 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 24, 2020, 1, 1, 0, 0, 25; "25 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 25, 2020, 1, 1, 0, 0, 26; "26 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 26, 2020, 1, 1, 0, 0, 27; "27 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 27, 2020, 1, 1, 0, 0, 28; "28 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 28, 2020, 1, 1, 0, 0, 29; "29 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 29, 2020, 1, 1, 0, 0, 30; "30 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 30, 2020, 1, 1, 0, 0, 31; "31 seconds after midnight")]
    fn test_next_seconds(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        hour_start: u32,
        minute_start: u32,
        second_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
        hour_end: u32,
        minute_end: u32,
        second_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(
                year_start,
                month_start,
                day_start,
                hour_start,
                minute_start,
                second_start,
            )
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(
                year_end, month_end, day_end, hour_end, minute_end, second_end,
            )
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_seconds, 1);
        assert_eq!(sut.invert, false);

        // with shared timezone
        let from = Pacific
            .with_ymd_and_hms(
                year_start,
                month_start,
                day_start,
                hour_start,
                minute_start,
                second_start,
            )
            .unwrap();
        let to = Pacific
            .with_ymd_and_hms(
                year_end, month_end, day_end, hour_end, minute_end, second_end,
            )
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_seconds, 1);
        assert_eq!(sut.invert, false);
    }

    #[test_case(2020, 1, 1, 0, 0, 30, 2020, 1, 1, 0, 0, 29; "30 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 29, 2020, 1, 1, 0, 0, 28; "29 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 28, 2020, 1, 1, 0, 0, 27; "28 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 27, 2020, 1, 1, 0, 0, 26; "27 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 26, 2020, 1, 1, 0, 0, 25; "26 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 25, 2020, 1, 1, 0, 0, 24; "25 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 24, 2020, 1, 1, 0, 0, 23; "24 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 23, 2020, 1, 1, 0, 0, 22; "23 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 22, 2020, 1, 1, 0, 0, 21; "22 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 21, 2020, 1, 1, 0, 0, 20; "21 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 20, 2020, 1, 1, 0, 0, 19; "20 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 19, 2020, 1, 1, 0, 0, 18; "19 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 18, 2020, 1, 1, 0, 0, 17; "18 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 17, 2020, 1, 1, 0, 0, 16; "17 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 16, 2020, 1, 1, 0, 0, 15; "16 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 15, 2020, 1, 1, 0, 0, 14; "15 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 14, 2020, 1, 1, 0, 0, 13; "14 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 13, 2020, 1, 1, 0, 0, 12; "13 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 12, 2020, 1, 1, 0, 0, 11; "12 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 11, 2020, 1, 1, 0, 0, 10; "11 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 10, 2020, 1, 1, 0, 0, 9; "10 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 9, 2020, 1, 1, 0, 0, 8; "9 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 8, 2020, 1, 1, 0, 0, 7; "8 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 7, 2020, 1, 1, 0, 0, 6; "7 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 6, 2020, 1, 1, 0, 0, 5; "6 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 5, 2020, 1, 1, 0, 0, 4; "5 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 4, 2020, 1, 1, 0, 0, 3; "4 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 3, 2020, 1, 1, 0, 0, 2; "3 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 2, 2020, 1, 1, 0, 0, 1; "2 seconds after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 1, 2020, 1, 1, 0, 0, 0; "1 second after midnight")]
    #[test_case(2020, 1, 1, 0, 0, 0, 2019, 12, 31, 23, 59, 59; "1 second before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 59, 2019, 12, 31, 23, 59, 58; "2 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 58, 2019, 12, 31, 23, 59, 57; "3 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 57, 2019, 12, 31, 23, 59, 56; "4 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 56, 2019, 12, 31, 23, 59, 55; "5 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 55, 2019, 12, 31, 23, 59, 54; "6 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 54, 2019, 12, 31, 23, 59, 53; "7 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 53, 2019, 12, 31, 23, 59, 52; "8 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 52, 2019, 12, 31, 23, 59, 51; "9 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 51, 2019, 12, 31, 23, 59, 50; "10 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 50, 2019, 12, 31, 23, 59, 49; "11 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 49, 2019, 12, 31, 23, 59, 48; "12 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 48, 2019, 12, 31, 23, 59, 47; "13 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 47, 2019, 12, 31, 23, 59, 46; "14 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 46, 2019, 12, 31, 23, 59, 45; "15 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 45, 2019, 12, 31, 23, 59, 44; "16 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 44, 2019, 12, 31, 23, 59, 43; "17 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 43, 2019, 12, 31, 23, 59, 42; "18 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 42, 2019, 12, 31, 23, 59, 41; "19 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 41, 2019, 12, 31, 23, 59, 40; "20 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 40, 2019, 12, 31, 23, 59, 39; "21 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 39, 2019, 12, 31, 23, 59, 38; "22 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 38, 2019, 12, 31, 23, 59, 37; "23 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 37, 2019, 12, 31, 23, 59, 36; "24 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 36, 2019, 12, 31, 23, 59, 35; "25 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 35, 2019, 12, 31, 23, 59, 34; "26 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 34, 2019, 12, 31, 23, 59, 33; "27 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 33, 2019, 12, 31, 23, 59, 32; "28 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 32, 2019, 12, 31, 23, 59, 31; "29 seconds before midnight")]
    #[test_case(2019, 12, 31, 23, 59, 31, 2019, 12, 31, 23, 59, 30; "30 seconds before midnight")]
    fn test_previous_seconds(
        year_start: i32,
        month_start: u32,
        day_start: u32,
        hour_start: u32,
        minute_start: u32,
        second_start: u32,
        year_end: i32,
        month_end: u32,
        day_end: u32,
        hour_end: u32,
        minute_end: u32,
        second_end: u32,
    ) {
        let from = Utc
            .with_ymd_and_hms(
                year_start,
                month_start,
                day_start,
                hour_start,
                minute_start,
                second_start,
            )
            .unwrap();
        let to = Utc
            .with_ymd_and_hms(
                year_end, month_end, day_end, hour_end, minute_end, second_end,
            )
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_seconds, 1);
        assert_eq!(sut.invert, true);

        // with shared timezone
        let from = Pacific
            .with_ymd_and_hms(
                year_start,
                month_start,
                day_start,
                hour_start,
                minute_start,
                second_start,
            )
            .unwrap();
        let to = Pacific
            .with_ymd_and_hms(
                year_end, month_end, day_end, hour_end, minute_end, second_end,
            )
            .unwrap();

        let sut = calculate(&from, &to);
        assert_eq!(sut.interval_seconds, 1);
        assert_eq!(sut.invert, true);
    }

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
}

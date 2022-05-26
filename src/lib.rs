pub mod date_component {
    use chrono::prelude::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DateComponent {
        /// Number of years.
        pub year: isize,
        /// Number of months.
        pub month:isize,
        /// Number of days.
        pub day: isize,
        /// Number of seconds.
        pub seconds: isize,
        /// Number of minutes.
        pub minutes: isize,
        /// Number of hours.
        pub hours: isize,
        /// total number of days between the start and end dates
        pub interval_day: isize,
        /// Is true if the interval represents a negative time period and false otherwise
        pub invert: bool,
    }
    
    pub fn calculate(date1: &DateTime<Utc>, date2: &DateTime<Utc>) -> DateComponent {
        let duration = date1.signed_duration_since(*date2);
        let seconds = duration.num_seconds();
        let (from, to, invert) = match seconds {
            x if x <=0 => (date1, date2, false),
            _ => (date2, date1, true)
        };
        let interval_seconds: isize = seconds.abs() as isize;
        let interval_minutes: isize = interval_seconds / 60;
        let interval_hours: isize = interval_minutes / 60;
        let diff_year  = to.year() - from.year();
        let diff_month = to.month() as i32 - from.month() as i32;
        let diff_day   = to.day() as i32 - from.day() as i32;

        let (to_year, to_month) = match to.month() {
            x if x > 1 => (to.year(), to.month() - 1),
            _ => (to.year() - 1, 12),
        };

        let (interval_day, interval_month) = match diff_day {
            x if x < 0 => (adjust_ymd(to_year, to_month, from.day())
                        .and_hms(to.hour(), to.minute(), to.second())
                        .signed_duration_since(*to).num_days().abs(), diff_month - 1),
            _ => (Utc.ymd(to.year(), to.month(), from.day())
                        .and_hms(to.hour(), to.minute(), to.second())
                        .signed_duration_since(*to).num_days().abs(), diff_month)
        };

        let (interval_year, interval_month) = match interval_month {
            x if x < 0 => (diff_year - 1, interval_month + 12),
            _ => (diff_year, interval_month),
        };

        DateComponent {
            year: interval_year as isize,
            month: interval_month as isize,
            day: interval_day as isize,
            seconds: interval_seconds,
            minutes: interval_minutes,
            hours: interval_hours,
            interval_day: duration.num_days().abs() as isize,
            invert,
        }
    }

    /// Given date specified by year / month / day where the `day` may be invalid,
    /// (e.g. 2021-02-30), return the nearest valid day before it
    /// (e.g. 2021-02-28).
    fn adjust_ymd(year: i32, month: u32, day: u32) -> Date<Utc> {
        let mut subtract = 0;
        loop {
            match Utc.ymd_opt(year, month, day - subtract) {
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
    use chrono::prelude::*;
    use super::date_component::*;

    #[test]
    fn case1() {
        let date1 = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
        let date2 =  Utc.ymd(2015, 12, 19).and_hms(0, 0, 0);
        
        let date_interval = calculate(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 7,
            day: 29,
            seconds: 20995200,
            minutes: 349920,
            hours: 5832,
            interval_day: 243,
            invert: false,
        });
    }
    
    #[test]
    fn case2() {
        let date1 =  Utc.ymd(2015, 12, 21).and_hms(0, 0, 0);
        let date2 = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
    
        let date_interval = calculate(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 8,
            day: 1,
            seconds: 21168000,
            minutes: 352800,
            hours: 5880,
            interval_day: 245,
            invert: true,
        });
    }
    
    #[test]
    fn case3() {
        let date1 =  Utc.ymd(2016, 2, 19).and_hms(0, 0, 0);
        let date2 = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
    
        let date_interval = calculate(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 9,
            day: 30,
            seconds: 26352000,
            minutes: 439200,
            hours: 7320,
            interval_day: 305,
            invert: true,
        });
    }
    
    #[test]
    fn case4() {
        let date1 = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
        let date2 =  Utc.ymd(2016, 2, 21).and_hms(0, 0, 0);
    
        let date_interval = calculate(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 10,
            day: 1,
            seconds: 26524800,
            minutes: 442080,
            hours: 7368,
            interval_day: 307,
            invert: false,
        });
    }
    
    #[test]
    fn case5() {
        let date1 = Utc.ymd(2016, 3, 20).and_hms(0, 0, 0);
        let date2 =  Utc.ymd(2016, 3, 20).and_hms(0, 0, 0);
    
        let date_interval = calculate(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 0,
            day: 0,
            seconds: 0,
            minutes: 0,
            hours: 0,
            interval_day: 0,
            invert: false,
        });
    }

    #[test]
    fn case6() {
        let date1 = Utc.ymd(2015, 12, 30).and_hms(0, 0, 0);
        let date2 = Utc.ymd(2016, 1, 1).and_hms(0, 0, 0);

        let date_interval = calculate(&date1, &date2);
        assert_eq!(
            date_interval,
            DateComponent {
                year: 0,
                month: 0,
                day: 2,
                seconds: 172800,
                minutes: 2880,
                hours: 48,
                interval_day: 2,
                invert: false,
            }
        );
    }

    #[test]
    fn case7() {
        let date1 = Utc.ymd(2020, 2, 29).and_hms(0, 0, 0);
        let date2 = Utc.ymd(2021, 2, 1).and_hms(0, 0, 0);

        let date_interval = calculate(&date1, &date2);
        assert_eq!(
            date_interval,
            DateComponent {
                year: 0,
                month: 11,
                day: 3,
                seconds: 29203200,
                minutes: 486720,
                hours: 8112,
                interval_day: 338,
                invert: false,
            }
        );
    }

    #[test]
    fn case8() {
        let date1 = Utc.ymd(2010, 11, 06).and_hms(0, 0, 0);
        let date2 = Utc.ymd(2010, 10, 04).and_hms(0, 0, 0);

        let date_interval = calculate(&date1, &date2);
        assert_eq!(
            date_interval,
            DateComponent {
                year: 0,
                month: 1,
                day: 2,
                seconds: 2851200,
                minutes: 47520,
                hours: 792,
                interval_day: 33,
                invert: true,
            }
        );
    }

    #[test]
    fn case9() {
        let date1 = Utc.ymd(2010, 11, 07).and_hms(0, 0, 0);
        let date2 = Utc.ymd(2010, 11, 06).and_hms(0, 0, 0);

        let date_interval = calculate(&date1, &date2);
        assert_eq!(
            date_interval,
            DateComponent {
                year: 0,
                month: 0,
                day: 1,
                seconds: 86400,
                minutes: 1440,
                hours: 24,
                interval_day: 1,
                invert: true,
            }
        );
    }

    #[test]
    fn case10() {
        let date1 = Utc.ymd(2022, 1, 30).and_hms(0, 0, 0);
        let date2 = Utc.ymd(2022, 3, 1).and_hms(0, 0, 0);

        let date_interval = calculate(&date1, &date2);
        assert_eq!(
            date_interval,
            DateComponent {
                year: 0,
                month: 1,
                day: 1,
                seconds: 2592000,
                minutes: 43200,
                hours: 720,
                interval_day: 30,
                invert: false,
            }
        );
    }
}

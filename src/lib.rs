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
        /// total number of days between the start and end dates
        pub interval_day: isize,
        /// Is true if the interval represents a negative time period and false otherwise
        pub invert: bool,
    }
    
    pub fn calculate(date1: &DateTime<Utc>, date2: &DateTime<Utc>) -> DateComponent {
        let duration = date1.signed_duration_since(*date2);
        let (from, to, invert) = match duration.num_seconds() {
            x if x <=0 => (date1, date2, false),
            _ => (date2, date1, true)
        };
        let diff_year  = to.year() - from.year();
        let diff_month = to.month() as i32 - from.month() as i32;
        let diff_day   = to.day() as i32 - from.day() as i32;

        let (to_year, to_month) = match to.month() {
            x if x > 1 => (to.year(), to.month() - 1),
            _ => (to.year() - 1, 12),
        };

        let (interval_day, interval_month) = match diff_day {
            x if x < 0 => (Utc.ymd(to_year, to_month, from.day())
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
            interval_day: duration.num_days().abs() as isize,
            invert,
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
                interval_day: 1,
                invert: true,
            }
        );
    }
}

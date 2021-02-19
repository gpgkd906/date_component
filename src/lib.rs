pub mod date_component {
    use chrono::prelude::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DateComponent {
        pub year: isize,
        pub month:isize,
        pub day: isize,
        pub interval_day: isize,
    }
    
    pub fn calculate_date_component(date1: &DateTime<Utc>, date2: &DateTime<Utc>) -> DateComponent {
        let duration = date1.signed_duration_since(*date2);
        let (from, to) = match duration.num_seconds() {
            x if x <0 => (date1, date2),
            _ => (date2, date1)
        };
        let diff_year  = to.year() - from.year();
        
        let diff_month = to.month() as i32 - from.month() as i32;
        let (interval_year, interval_month) = match diff_month {
            x if x < 0 => (diff_year - 1, diff_month + 12),
            _ => (diff_year, diff_month),
        };

        let diff_day   = to.day() as i32 - from.day() as i32;
        let (interval_day, interval_month) = match diff_day {
            x if x < 0 => (Utc.ymd(to.year(), to.month() - 1, from.day())
                        .and_hms(to.hour(), to.minute(), to.second())
                        .signed_duration_since(*to).num_days().abs(), interval_month - 1),
            _ => (Utc.ymd(to.year(), to.month(), from.day())
                        .and_hms(to.hour(), to.minute(), to.second())
                        .signed_duration_since(*to).num_days().abs(), interval_month)
        };
        DateComponent {
            year: interval_year as isize,
            month: interval_month as isize,
            day: interval_day as isize,
            interval_day: duration.num_days().abs() as isize,
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
        
        let date_interval = calculate_date_component(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 7,
            day: 29,
            interval_day: 243,
        });
    }
    
    #[test]
    fn case2() {
        let date1 = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
        let date2 =  Utc.ymd(2015, 12, 21).and_hms(0, 0, 0);
    
        let date_interval = calculate_date_component(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 8,
            day: 1,
            interval_day: 245,
        });
    }
    
    #[test]
    fn case3() {
        let date1 = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
        let date2 =  Utc.ymd(2016, 2, 19).and_hms(0, 0, 0);
    
        let date_interval = calculate_date_component(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 9,
            day: 30,
            interval_day: 305,
        });
    }
    
    #[test]
    fn case4() {
        let date1 = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
        let date2 =  Utc.ymd(2016, 2, 21).and_hms(0, 0, 0);
    
        let date_interval = calculate_date_component(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 10,
            day: 1,
            interval_day: 307,
        });
    }
    
    #[test]
    fn case5() {
        let date1 = Utc.ymd(2016, 3, 20).and_hms(0, 0, 0);
        let date2 =  Utc.ymd(2016, 3, 20).and_hms(0, 0, 0);
    
        let date_interval = calculate_date_component(&date1, &date2);
        assert_eq!(date_interval, DateComponent {
            year: 0,
            month: 0,
            day: 0,
            interval_day: 0,
        });
    }
}

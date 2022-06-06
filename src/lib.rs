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
    /// Number of days.
    pub day: isize,
    /// Number of seconds.
    pub interval_seconds: isize,
    /// Number of minutes.
    pub interval_minutes: isize,
    /// Number of hours.
    pub interval_hours: isize,
    /// total number of days between the start and end dates
    pub interval_day: isize,
    /// Is true if the interval represents a negative time period and false otherwise
    pub invert: bool,
  }

  /// Returns a DateComponent object that represents the difference between the from and to datetime.
  pub fn calculate(from_datetime: &DateTime<Utc>, to_datetime: &DateTime<Utc>) -> DateComponent {
    let duration = from_datetime.signed_duration_since(*to_datetime);
    let seconds = duration.num_seconds();
    let (from, to, invert) = match seconds {
      x if x <= 0 => (from_datetime, to_datetime, false),
      _ => (to_datetime, from_datetime, true),
    };
    let interval_seconds: isize = seconds.abs() as isize;
    let interval_minutes: isize = interval_seconds / 60;
    let interval_hours: isize = interval_minutes / 60;
    let diff_year = to.year() - from.year();
    let diff_month = to.month() as i32 - from.month() as i32;
    let diff_day = to.day() as i32 - from.day() as i32;

    let (to_year, to_month) = match to.month() {
      x if x > 1 => (to.year(), to.month() - 1),
      _ => (to.year() - 1, 12),
    };

    let (interval_day, interval_month) = match diff_day {
      x if x < 0 => (
        adjust_ymd(to_year, to_month, from.day())
          .and_hms(to.hour(), to.minute(), to.second())
          .signed_duration_since(*to)
          .num_days()
          .abs(),
        diff_month - 1,
      ),
      _ => (
        Utc
          .ymd(to.year(), to.month(), from.day())
          .and_hms(to.hour(), to.minute(), to.second())
          .signed_duration_since(*to)
          .num_days()
          .abs(),
        diff_month,
      ),
    };

    let (interval_year, interval_month) = match interval_month {
      x if x < 0 => (diff_year - 1, interval_month + 12),
      _ => (diff_year, interval_month),
    };

    let (interval_week, interval_day) = match interval_day {
      x if x < 0 => (
        adjust_ymd(to_year, to_month, from.day())
          .and_hms(to.hour(), to.minute(), to.second())
          .signed_duration_since(*to)
          .num_days()
          .abs(),
        interval_day + 7,
      ),
      _ => (interval_day / 7, interval_day % 7),
    };

    DateComponent {
      year: interval_year as isize,
      month: interval_month as isize,
      week: interval_week as isize,
      day: interval_day as isize,
      interval_seconds: interval_seconds,
      interval_minutes: interval_minutes,
      interval_hours: interval_hours,
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
  use super::date_component::*;
  use chrono::prelude::*;

  #[test]
  fn case1() {
    let from_datetime = Utc.ymd(2012, 4, 20).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2015, 12, 19).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 3,
        month: 7,
        week: 4,
        day: 1,
        interval_seconds: 115603200,
        interval_minutes: 1926720,
        interval_hours: 32112,
        interval_day: 1338,
        invert: false,
      }
    );
  }

  #[test]
  fn case2() {
    let from_datetime = Utc.ymd(2013, 12, 21).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 1,
        month: 3,
        week: 4,
        day: 2,
        interval_seconds: 41904000,
        interval_minutes: 698400,
        interval_hours: 11640,
        interval_day: 485,
        invert: false,
      }
    );
  }

  #[test]
  fn case3() {
    let from_datetime = Utc.ymd(2016, 2, 19).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 0,
        month: 9,
        week: 4,
        day: 2,
        interval_seconds: 26352000,
        interval_minutes: 439200,
        interval_hours: 7320,
        interval_day: 305,
        invert: true,
      }
    );
  }

  #[test]
  fn case4() {
    let from_datetime = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2016, 2, 21).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 0,
        month: 10,
        week: 0,
        day: 1,
        interval_seconds: 26524800,
        interval_minutes: 442080,
        interval_hours: 7368,
        interval_day: 307,
        invert: false,
      }
    );
  }

  #[test]
  fn case5() {
    let from_datetime = Utc.ymd(2016, 3, 20).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2016, 3, 20).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 0,
        month: 0,
        week: 0,
        day: 0,
        interval_seconds: 0,
        interval_minutes: 0,
        interval_hours: 0,
        interval_day: 0,
        invert: false,
      }
    );
  }

  #[test]
  fn case6() {
    let from_datetime = Utc.ymd(2015, 12, 30).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2016, 1, 1).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 0,
        month: 0,
        week: 0,
        day: 2,
        interval_seconds: 172800,
        interval_minutes: 2880,
        interval_hours: 48,
        interval_day: 2,
        invert: false,
      }
    );
  }

  #[test]
  fn case7() {
    let from_datetime = Utc.ymd(2020, 2, 29).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2021, 2, 1).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 0,
        month: 11,
        week: 0,
        day: 3,
        interval_seconds: 29203200,
        interval_minutes: 486720,
        interval_hours: 8112,
        interval_day: 338,
        invert: false,
      }
    );
  }

  #[test]
  fn case8() {
    let from_datetime = Utc.ymd(2010, 11, 06).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2010, 10, 04).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 0,
        month: 1,
        week: 0,
        day: 2,
        interval_seconds: 2851200,
        interval_minutes: 47520,
        interval_hours: 792,
        interval_day: 33,
        invert: true,
      }
    );
  }

  #[test]
  fn case9() {
    let from_datetime = Utc.ymd(2010, 11, 07).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2010, 11, 06).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 0,
        month: 0,
        week: 0,
        day: 1,
        interval_seconds: 86400,
        interval_minutes: 1440,
        interval_hours: 24,
        interval_day: 1,
        invert: true,
      }
    );
  }

  #[test]
  fn case10() {
    let from_datetime = Utc.ymd(2022, 1, 30).and_hms(0, 0, 0);
    let to_datetime = Utc.ymd(2022, 3, 1).and_hms(0, 0, 0);

    let date_interval = calculate(&from_datetime, &to_datetime);
    assert_eq!(
      date_interval,
      DateComponent {
        year: 0,
        month: 1,
        week: 0,
        day: 1,
        interval_seconds: 2592000,
        interval_minutes: 43200,
        interval_hours: 720,
        interval_day: 30,
        invert: false,
      }
    );
  }
}

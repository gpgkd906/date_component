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
    /// Number of hours.
    pub hour: isize,
    /// Number of minutes.
    pub minute: isize,
    /// Number of seconds.
    pub second: isize,
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
    let diff_year = to.year() - from.year();
    let diff_month = to.month() as i32 - from.month() as i32;
    let diff_day = to.day() as i32 - from.day() as i32;
    let diff_hour = to.hour() as i64 - from.hour() as i64;
    let diff_minute = to.minute() as i64 - from.minute() as i64;
    let diff_second = to.second() as i64 - from.second() as i64;

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

    let (interval_hour, interval_minute) = match diff_hour {
      x if x < 0 => (
        adjust_ymd(to_year, to_month, from.day())
          .and_hms(to.hour(), to.minute(), to.second())
          .signed_duration_since(*to)
          .num_hours()
          .abs(),
        diff_minute - 1,
      ),
      _ => (diff_hour, diff_minute),
    };

    let (interval_minute, interval_second) = match diff_minute {
      x if x < 0 => (
        adjust_ymd(to_year, to_month, from.day())
          .and_hms(to.hour(), to.minute(), to.second())
          .signed_duration_since(*to)
          .num_minutes()
          .abs(),
        diff_minute - 1,
      ),
      _ => (interval_minute, diff_second),
    };

    DateComponent {
      year: interval_year as isize,
      month: interval_month as isize,
      week: interval_week as isize,
      day: interval_day as isize,
      hour: interval_hour as isize,
      minute: interval_minute as isize,
      second: interval_second as isize,
      interval_seconds: duration.num_seconds().abs() as isize,
      interval_minutes: duration.num_minutes().abs() as isize,
      interval_hours: duration.num_hours().abs() as isize,
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
  fn test_1_year_interval() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.year, 1);
    assert_eq!(sut.invert, false);
  }

  #[test]
  fn test_1_year_inverted_interval() {
    let from = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.year, 1);
    assert_eq!(sut.invert, true);
  }

  #[test]
  fn test_1_month_interval() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 2, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.month, 1);
    assert_eq!(sut.invert, false);
  }

  #[test]
  fn test_1_month_inverted_interval() {
    let from = Utc.ymd(2020, 2, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.month, 1);
    assert_eq!(sut.invert, true);
  }

  #[test]
  fn test_1_week_interval() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 8).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.week, 1);
    assert_eq!(sut.invert, false);
  }

  #[test]
  fn test_1_week_inverted_interval() {
    let from = Utc.ymd(2020, 1, 8).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.week, 1);
    assert_eq!(sut.invert, true);
  }

  #[test]
  fn test_1_day_interval() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 2).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.day, 1);
    assert_eq!(sut.invert, false);
  }

  #[test]
  fn test_1_day_inverted_interval() {
    let from = Utc.ymd(2020, 1, 2).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.day, 1);
    assert_eq!(sut.invert, true);
  }

  #[test]
  fn test_interval_hours() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(1, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.interval_hours, 1);
    assert_eq!(sut.invert, false);
  }

  #[test]
  fn test_inverted_interval_hours() {
    let from = Utc.ymd(2020, 1, 1).and_hms(1, 0, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.interval_hours, 1);
    assert_eq!(sut.invert, true);
  }

  #[test]
  fn test_interval_minutes() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 1, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.interval_minutes, 1);
    assert_eq!(sut.invert, false);
  }

  #[test]
  fn test_inverted_interval_minutes() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 1, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.interval_minutes, 1);
    assert_eq!(sut.invert, true);
  }

  #[test]
  fn test_interval_seconds() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 0, 1);

    let sut = calculate(&from, &to);
    assert_eq!(sut.interval_seconds, 1);
    assert_eq!(sut.invert, false);
  }

  #[test]
  fn test_inverted_interval_seconds() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 1);
    let to = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    let sut = calculate(&from, &to);
    assert_eq!(sut.interval_seconds, 1);
    assert_eq!(sut.invert, true);
  }

  #[test]
  fn test_next_year_month_day_hour_minute_second() {
    let from = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let to = Utc.ymd(2021, 2, 8).and_hms(1, 1, 1);

    let sut = calculate(&from, &to);
    assert_eq!(
      sut,
      DateComponent {
        year: 1,
        month: 1,
        week: 1,
        day: 1,
        hour: 1,
        minute: 1,
        second: 1,
        interval_day: 365 + 31 + 7 + 1,
        interval_hours: (365 + 31 + 7 + 1) * 24 + 1,
        interval_minutes: ((365 + 31 + 7 + 1) * 24 + 1) * 60 + 1,
        interval_seconds: (((365 + 31 + 7 + 1) * 24 + 1) * 60 + 1) * 60 + 1,
        invert: false,
      }
    );
  }
}

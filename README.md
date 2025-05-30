# date_component

calculate dates interval between 2 dates with shared timezone with chrono.
and the calculation is also considered with DST if necessary.

# API
https://gpgkd906.github.io/date_component/date_component/

# Example

```rust
use chrono::prelude::*;
use date_component::date_component;

fn main() {
    let date1 = Utc.ymd(2015, 4, 20).and_hms(0, 0, 0);
    let date2 =  Utc.ymd(2015, 12, 19).and_hms(0, 0, 0);
    
    let date_interval = date_component::calculate(&date1, &date2);
    println!("{:?}", date_interval);
}
// DateComponent { year: 0, month: 7, week: 4, modulo_days: 1, day: 29, hour: 0, minute: 0, second: 0, interval_seconds: 20995200, interval_minutes: 349920, interval_hours: 5832, interval_days: 243, invert: false }
```

# Tests
Run tests with `cargo test`. see `src/lib.rs`.

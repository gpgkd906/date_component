# date_component

calculate date interval with chrono.

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
// DateComponent { year: 0, month: 7, day: 29, seconds: 20995200, minutes: 349920, hours: 5832, interval_day: 243, invert: false }
```
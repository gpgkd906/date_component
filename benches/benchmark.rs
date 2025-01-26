use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chrono::{TimeZone, Utc, Local};
use date_component::date_component;

fn benchmark_calculate(c: &mut Criterion) {
    let timezone = Utc;

    let date1 = timezone.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let date2 = timezone.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap();
    
    let date3 = timezone.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap();
    let date4 = timezone.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap();
    
    let date5 = timezone.with_ymd_and_hms(2022, 12, 31, 23, 59, 59).unwrap();
    let date6 = timezone.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    
    let date7 = Local.with_ymd_and_hms(2023, 3, 12, 1, 30, 0).unwrap();
    let date8 = Local.with_ymd_and_hms(2023, 3, 12, 3, 30, 0).unwrap();


    c.bench_function("calculate simple", |b| {
        b.iter(|| date_component::calculate(black_box(&date1), black_box(&date2)))
    });

    c.bench_function("calculate cross month", |b| {
        b.iter(|| date_component::calculate(black_box(&date3), black_box(&date4)))
    });

    c.bench_function("calculate cross year", |b| {
        b.iter(|| date_component::calculate(black_box(&date5), black_box(&date6)))
    });

    c.bench_function("calculate daylight saving", |b| {
        b.iter(|| date_component::calculate(black_box(&date7), black_box(&date8)))
    });
}

criterion_group!(benches, benchmark_calculate);
criterion_main!(benches);
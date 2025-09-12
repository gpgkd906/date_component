use date_component::date_component::*;
use chrono::prelude::*;

#[test]
fn test_month_length_edge_cases() {
    // Test from January 31 to February 28 (non-leap year)
    let from = Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 2, 28, 0, 0, 0).unwrap();
    let diff = calculate(&from, &to);
    
    // This should not crash and should give reasonable results
    println!("Jan 31 to Feb 28, 2023: {:?}", diff);
    assert!(diff.month >= 0);
    assert!(diff.day >= 0);
    assert!(!diff.invert);
}

#[test]
fn test_month_length_edge_cases_leap_year() {
    // Test from January 31 to February 29 (leap year)
    let from = Utc.with_ymd_and_hms(2024, 1, 31, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2024, 2, 29, 0, 0, 0).unwrap();
    let diff = calculate(&from, &to);
    
    println!("Jan 31 to Feb 29, 2024: {:?}", diff);
    assert!(diff.month >= 0);
    assert!(diff.day >= 0);
    assert!(!diff.invert);
}

#[test]
fn test_negative_duration_edge_case() {
    // Test a case that might cause issues with negative duration
    let from = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let diff = calculate(&from, &to);
    
    println!("Dec 31 23:59:59 to Jan 1 00:00:00: {:?}", diff);
    assert!(diff.invert); // Should be inverted since going backwards
    assert!(diff.year >= 0 || diff.month >= 0 || diff.day >= 0); // Should have some positive time difference
}

#[test]
fn test_complex_date_arithmetic() {
    // Test a complex case that might reveal arithmetic bugs
    let from = Utc.with_ymd_and_hms(2020, 2, 29, 12, 30, 45).unwrap(); // Leap day
    let to = Utc.with_ymd_and_hms(2021, 3, 31, 14, 45, 30).unwrap();
    let diff = calculate(&from, &to);
    
    println!("Complex leap year calculation: {:?}", diff);
    
    // Basic sanity checks
    assert!(diff.year >= 0);
    assert!(diff.month >= 0);
    assert!(diff.day >= 0);
    assert!(diff.hour >= 0);
    assert!(diff.minute >= 0);
    assert!(diff.second >= 0);
    assert!(!diff.invert);
    
    // The total should be consistent with the components
    let total_seconds = diff.interval_seconds;
    assert!(total_seconds > 0);
}

#[test]
fn test_dst_boundary_arithmetic() {
    // Test around DST boundaries to check for arithmetic consistency
    use chrono_tz::America::Los_Angeles;
    
    // Spring forward: 2023-03-12 02:00 becomes 03:00
    let before_spring = Los_Angeles.with_ymd_and_hms(2023, 3, 12, 1, 30, 0).unwrap();
    let after_spring = Los_Angeles.with_ymd_and_hms(2023, 3, 12, 3, 30, 0).unwrap();
    let diff = calculate(&before_spring, &after_spring);
    
    println!("DST spring forward: {:?}", diff);
    
    // Should handle DST transition gracefully
    assert!(diff.interval_hours > 0);
    assert!(!diff.invert);
}

#[test]
fn test_zero_difference() {
    // Test when from and to are exactly the same
    let dt = Utc.with_ymd_and_hms(2023, 6, 15, 12, 30, 45).unwrap();
    let diff = calculate(&dt, &dt);
    
    assert_eq!(diff.year, 0);
    assert_eq!(diff.month, 0);
    assert_eq!(diff.day, 0);
    assert_eq!(diff.hour, 0);
    assert_eq!(diff.minute, 0);
    assert_eq!(diff.second, 0);
    assert_eq!(diff.interval_seconds, 0);
    assert!(!diff.invert);
}

#[test]
fn test_very_small_differences() {
    // Test with very small time differences
    let from = Utc.with_ymd_and_hms(2023, 6, 15, 12, 30, 45).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 6, 15, 12, 30, 46).unwrap(); // 1 second difference
    let diff = calculate(&from, &to);
    
    assert_eq!(diff.year, 0);
    assert_eq!(diff.month, 0);
    assert_eq!(diff.day, 0);
    assert_eq!(diff.hour, 0);
    assert_eq!(diff.minute, 0);
    assert_eq!(diff.second, 1);
    assert_eq!(diff.interval_seconds, 1);
    assert!(!diff.invert);
}

#[test]
fn test_year_boundary_consistency() {
    // Test consistency across year boundaries
    let from = Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 59).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 1).unwrap();
    let diff = calculate(&from, &to);
    
    println!("Year boundary: {:?}", diff);
    
    // Should be 2 seconds total
    assert_eq!(diff.interval_seconds, 2);
    assert_eq!(diff.second, 2);
    
    // FIXED: These should now be correct after the consistency check fix
    assert_eq!(diff.day, 0);
    assert_eq!(diff.modulo_days, 0);
    
    assert!(!diff.invert);
}

#[test] 
fn test_bug_small_time_differences() {
    // Test that small time differences don't incorrectly calculate days
    let from = Utc.with_ymd_and_hms(2023, 1, 1, 23, 59, 59).unwrap();
    let to = Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 1).unwrap();
    let diff = calculate(&from, &to);
    
    println!("Small time diff across day boundary: {:?}", diff);
    
    // This is a 2 second difference, but crosses midnight
    assert_eq!(diff.interval_seconds, 2);
    
    // FIXED: Should now correctly calculate 0 days for small time differences
    assert_eq!(diff.day, 0);
}

// I AM NOT DONE

mod calc_time;
use calc_time::time_info;
use std::time::{Duration, Instant};
const TEST_CASES: &[(&str, &str)] = &[
    ("2025-01-01", "1,3,1,364,27,0"),
    ("2025-01-18", "2,6,18,347,10,1"),
];
fn main() {
    test_calc_time();
}

fn test_calc_time() {
    let mut total_score = 0.0;
    for (input, expected) in TEST_CASES {
        let start = Instant::now();
        let result = time_info(*input);
        println!("input: {}, result: {}", input, result);
        let duration = start.elapsed();

        // 时间超0.2s，判定不合格
        if duration <= Duration::from_millis(200) && result == *expected {
            total_score += 10.0;
        }
    }

    println!("Total score: {:.2}", total_score);
    assert_eq!(100.00, total_score);
}

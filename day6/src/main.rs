mod day_code;

use day_code::{day_result, day_result_pt2};

fn main() {
    println!("RESTULT {}", day_result("data/data.txt".to_string()));
    println!("RESTULT DAY 2 {}", day_result_pt2("data/data.txt".to_string()));
}

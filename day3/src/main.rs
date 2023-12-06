mod day_code;

use day_code::{day_result, day_result2};

fn main() {
    println!("RESTULT {}", day_result("data/data.txt".to_string()));
    println!("RESTULT {}", day_result2("data/data.txt".to_string()));
}

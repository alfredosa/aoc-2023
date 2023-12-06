use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_result(path: String) -> u64 {
    Some(read_input(path));
    55
}

fn read_input(path: String) -> io::Result<()> {
    let path = Path::new(&path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        result.push(numbers);
    }

    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_result() {
        assert_eq!(30, day_result(String::from("data/test.txt")));
    }

    // #[test]
    // fn test_day_result2() {
    //     assert_eq!(467835, day_result2(String::from("data/test.txt")));
    // }
}

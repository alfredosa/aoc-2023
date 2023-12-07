use std::fs::File;
use std::io::{self, BufRead};
use std::os::unix::process;
use std::path::Path;

pub fn day_result(path: String) -> usize {
    let data = read_input(path).unwrap();
    let time = &data[0];
    let distance = &data[1];

    let mut result = Vec::new();
    for i in 0..time.len() {
        let time_t = time[i];
        let distance_t = distance[i];

        let results = results_from_time_distance(time_t, distance_t);

        println!("RESULTS {:?}", results);
        result.push(results);
    }
    
    let mut final_sum = 1;
    for val in result {
        final_sum *= val.len();
    }
    final_sum
}

fn results_from_time_distance(time: i32, distance: i32) -> Vec<i32> {
    let mut result = Vec::new();
    
    for i in 0..time {
        let vel = time - i;
        let remaining_time = time - vel;
        let final_distance = remaining_time * vel;

        if final_distance > distance{
            result.push(final_distance);
        }
    }

    result
}

fn read_input(path: String) -> io::Result<Vec<Vec<i32>>> {
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
    Ok(result)
}


fn read_input_pt2(path: String) -> io::Result<Vec<String>> {
    let path = Path::new(&path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line
            .split_whitespace()
            .collect();

        result.push(numbers.concat());
    }

    println!("{:?}", result);
    Ok(result)
}

pub fn day_result_pt2(path: String) -> usize {
    let data = read_input_pt2(path).unwrap();
    let time: Vec<&str> = data[0].split(":").collect();
    let distance: Vec<&str> = data[1].split(":").collect();

    let distance_value = distance[1].parse::<i64>().unwrap();
    let time_value = time[1].parse::<i64>().unwrap();
    
    let result = results_from_time_distance2(time_value, distance_value);
    result.len()
}

fn results_from_time_distance2(time: i64, distance: i64) -> Vec<i64> {
    let mut result = Vec::new();
    
    for i in 0..time {
        let vel = time - i;
        let remaining_time = time - vel;
        let final_distance = remaining_time * vel;

        if final_distance > distance{
            result.push(final_distance);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_result() {
        assert_eq!(288, day_result(String::from("data/test.txt")));
    }

    #[test]
    fn test_day_result2() {
        assert_eq!(71503, day_result_pt2(String::from("data/test.txt")));
    }
}

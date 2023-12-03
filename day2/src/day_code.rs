
use std::fs;

fn read_input(file_path: String) -> String {

    println!("search path file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    contents
}

pub fn day_result(path: String) -> u64 {
    let file_data = read_input(path);
    let file_input = file_data.split("\n").into_iter().collect::<Vec<&str>>();
    
    get_game_id_sum(file_input)
}

fn get_game_id_sum(file_input: Vec<&str>) -> u64 {
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut game_id_sum: u64 = 0;
    file_input.iter().for_each(|line| {
        // repurposed count is now the biggest number in the category
        let mut red_count: u64 = 0;
        let mut green_count: u64 = 0;
        let mut blue_count: u64 = 0;
        let mut game_id: u32 = 0;

        let split_line = line.split(":").into_iter().collect::<Vec<&str>>();
        game_id = split_line[0].split(" ").into_iter().collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        
        // this is a set, and we need to check if the amount of cubes is less than the max
        // let mut add = true;
        // split_line[1].split(";").into_iter().for_each(|set| {
        //     let set_split = set.split(",").into_iter().collect::<Vec<&str>>();
        //     set_split.iter().for_each(|line| {

        //         let set_draw = line.split(" ").into_iter().collect::<Vec<&str>>();
        //         let color = set_draw[2];
        //         let amount = set_draw[1].parse::<i8>().unwrap();

        //         match color {
        //             "red" => {
        //                 if amount > max_red {
        //                     add = false;
        //                 }
        //             },
        //             "green" => {
        //                 if amount > max_green {
        //                     add = false;
        //                 }
        //             },
        //             "blue" => {
        //                 if amount > max_blue {
        //                     add = false;
        //                 }
        //             },
        //             _ => println!("Unknown color"),
        //         }
        //     });
        // });

        // if add {
        //     game_id_sum += game_id;
        // }

        split_line[1].split(";").into_iter().for_each(|set| {
            let set_split = set.split(",").into_iter().collect::<Vec<&str>>();
            set_split.iter().for_each(|line| {

                let set_draw = line.split(" ").into_iter().collect::<Vec<&str>>();
                let color = set_draw[2];
                let amount = set_draw[1].parse::<u64>().unwrap();

                match color {
                    "red" => {
                        if amount > red_count {
                            red_count = amount;
                        }
                    },
                    "green" => {
                        if amount > green_count{
                            green_count = amount;
                        }
                    },
                    "blue" => {
                        if amount > blue_count {
                            blue_count = amount;
                        }
                    },
                    _ => println!("Unknown color"),
                }
            });
        });

        let power_cubes: u64 = red_count * green_count * blue_count;
        game_id_sum += power_cubes;
    });
    
    game_id_sum
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_result() {
        assert_eq!(2286, day_result(String::from("data/test.txt")));
    }
}
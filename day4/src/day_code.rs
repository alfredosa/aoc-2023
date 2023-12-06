use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning_numbers: i32,
    copies: i32,
}

pub fn day_result(path: String) -> u64 {
    let file_data = read_input(path);
    let file_input: Vec<String> = file_data.split("\n").map(|s| s.to_string()).collect();

    let all_cards: Arc<Mutex<Vec<Card>>> = Arc::new(Mutex::new(Vec::new()));

    for i in 1..file_input.len() + 1 {
        let card = Card {
            id: i as i32,
            winning_numbers: 0,
            copies: 1,
        };
        all_cards.lock().unwrap().push(card);
    };

    for line in file_input {
        let cards = Arc::clone(&all_cards);
        process_line_p2(&line, &cards);
    };

    get_all_copies(Arc::clone(&all_cards))
}


fn get_all_copies(all_cards: Arc<Mutex<Vec<Card>>>) -> u64 {
    let mut sum = 0;
    let all_cards = all_cards.lock().unwrap();
    for card in all_cards.iter() {
        sum += card.copies as u64;
    }
    sum
}

fn read_input(file_path: String) -> String {
    // println!("search path file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

fn process_line_p2(line: &str, sum: &Arc<Mutex<Vec<Card>>>) {
    let divided_line = line.split(" | ").into_iter().collect::<Vec<&str>>();
    let left_side =  divided_line[0].split(":").into_iter().collect::<Vec<&str>>();
    let card_split = left_side[0].split(" ").into_iter().collect::<Vec<&str>>();
    let card: i32 = card_split[card_split.len()-1].parse().unwrap();
    let winning_numbers = left_side[1].split(" ").into_iter().collect::<Vec<&str>>();
    let picked_numbers = divided_line[1].split(" ").into_iter().collect::<Vec<&str>>();

    let mut found_numbers = 0;
    picked_numbers.iter().for_each(|&x| {
        if winning_numbers.contains(&x) && x != "" {
            found_numbers += 1;
        }
    });

    let mut current_card = sum.lock().unwrap().iter().find(|x| x.id == card).unwrap().clone();
    current_card.winning_numbers = found_numbers as i32;

    

    if found_numbers > 0 {

        println!("card {} \n found numbers {}", card, found_numbers);
        println!("card currnet {:?} \n", current_card);

        for i in card+1..card+found_numbers+1 as i32 {
            let mut all_cards = sum.lock().unwrap();
            let mut arc_card = all_cards.iter_mut().find(|x| x.id == i).unwrap();
            
            for _ in 0..current_card.copies {
                arc_card.copies += 1;
            }
            
        }
    }
}

fn process_line(line: &str, sum: &Arc<Mutex<u64>>) {
    let divided_line = line.split(" | ").into_iter().collect::<Vec<&str>>();
    let left_side =  divided_line[0].split(": ").into_iter().collect::<Vec<&str>>();
    
    let card = left_side[0];

    let winning_numbers = left_side[1].split(" ").into_iter().collect::<Vec<&str>>();
    let picked_numbers = divided_line[1].split(" ").into_iter().collect::<Vec<&str>>();

    // println!("card {} \n winning_numbers {:?} \n picked_numbers {:?}", card, winning_numbers, picked_numbers);

    let mut found_numbers = 0;
    picked_numbers.iter().for_each(|&x| {
        if winning_numbers.contains(&x) && x != "" {
            found_numbers += 1;
        }
    });

    let mut summary = 0;
    if found_numbers > 0 {
        for i in 0..found_numbers {
            if i == 0 {
                summary += 1;
            } else {
                summary *= 2;
            }
        }
    }
    let mut sum = sum.lock().unwrap();
    *sum += summary;
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

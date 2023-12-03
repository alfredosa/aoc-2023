use std::fs;

#[derive(PartialEq, Debug)]
enum Type {
    Symbol,
    Number,
}

#[derive(Debug)]
struct Token {
    line: usize,
    pos: usize,
    value: String,
    token_type: Type,
}

fn read_input(file_path: String) -> String {
    println!("search path file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

pub fn day_result(path: String) -> u64 {
    let file_data = read_input(path);
    let file_input = file_data.split("\n").into_iter().collect::<Vec<&str>>();
    let mut tokens: Vec<Token> = Vec::new();

    for (line_numb, line) in file_input.into_iter().enumerate() {
        let all_line_tokens = all_line_tokens(line_numb, line);
        tokens.extend(all_line_tokens);
    }

    let numbers = tokens
        .iter()
        .filter(|&x| x.token_type == Type::Number)
        .map(|x| {
            let all_symbols_around = tokens
            .iter()
            .filter(|&symbols| {

                let line_above = x.line.saturating_sub(1);
                let line_below = x.line.saturating_add(1);
                let pos_left = x.pos.saturating_sub(1);
                let pos_right = x.pos + x.value.len();

                let condition = symbols.token_type == Type::Symbol
                    && (symbols.line == line_above && (symbols.pos >= pos_left && symbols.pos <= pos_right)
                        || symbols.line == line_below && (symbols.pos >= pos_left && symbols.pos <= pos_right)
                        || symbols.line == x.line && (symbols.pos >= pos_left && symbols.pos <= pos_right));
                
                condition
            })
            .collect::<Vec<&Token>>();
            

            if all_symbols_around.clone().len() > 0 {
                x.value.as_str()
            } else {
                "0"
            }
        })
        .collect::<Vec<&str>>();

    sum_all_numbers(numbers)
}


pub fn day_result2(path: String) -> u64 {
    let file_data = read_input(path);
    let file_input = file_data.split("\n").into_iter().collect::<Vec<&str>>();
    let mut tokens: Vec<Token> = Vec::new();

    for (line_numb, line) in file_input.into_iter().enumerate() {
        let all_line_tokens = all_line_tokens(line_numb, line);
        tokens.extend(all_line_tokens);
    }

    let numbers = tokens
        .iter()
        .filter(|&x| x.token_type == Type::Symbol && x.value == "*")
        .map(|x| {
            let ratio = 0;
            let all_numbers_around = tokens
            .iter()
            .filter(|&symbols| {

                let line_above = x.line.saturating_sub(1);
                let line_below = x.line.saturating_add(1);
                let pos_left = x.pos.saturating_sub(1);
                let pos_right = x.pos + x.value.len();
                let condition = symbols.token_type == Type::Number
                    && (symbols.line == line_above && (symbols.pos + symbols.value.len() -1 >= pos_left && symbols.pos <= pos_right)
                        || symbols.line == line_below && (symbols.pos + symbols.value.len() -1 >= pos_left && symbols.pos <= pos_right)
                        || symbols.line == x.line && (symbols.pos + symbols.value.len() -1 >= pos_left && symbols.pos <= pos_right));
                
                condition
            })
            .collect::<Vec<&Token>>();
            

            if all_numbers_around.clone().len() == 2 {
                let val1: u64 = all_numbers_around[0].value.parse().unwrap();
                let val2: u64 = all_numbers_around[1].value.parse().unwrap();
                val1 * val2
            } else {
                0
            }
        })
        .collect::<Vec<u64>>();

        numbers.iter().sum()
}


fn all_line_tokens(line_number: usize, line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token = Token {
        line: line_number,
        pos: 0,
        value: String::from(""),
        token_type: Type::Symbol,
    };

    for (pos, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if token.token_type == Type::Symbol {
                if token.value.len() > 0 {
                    tokens.push(token);
                }
                token = Token {
                    line: line_number,
                    pos: pos,
                    value: String::from(""),
                    token_type: Type::Number,
                };
            }
            token.value.push(c);
        } else {
            if token.token_type == Type::Number {
                if token.value.len() > 0 {
                    tokens.push(token);
                }
                token = Token {
                    line: line_number,
                    pos: pos,
                    value: String::from(""),
                    token_type: Type::Symbol,
                };
            } else if token.token_type == Type::Symbol {
                if token.value.len() > 0 && token.value.len() < 2 {
                    tokens.push(token);
                }
                token = Token {
                    line: line_number,
                    pos: pos,
                    value: String::from(""),
                    token_type: Type::Symbol,
                };
            }

            if c != '.' {
                token.value.push(c);
            }
        }
    }
    if token.value.len() > 0 {
        tokens.push(token);
    }
    tokens
}

fn sum_all_numbers(list: Vec<&str>) -> u64 {
    let mut sum = 0;
    for item in list {
        let number = item.parse::<u64>().unwrap();
        sum += number;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_result() {
        assert_eq!(4361, day_result(String::from("data/test.txt")));
    }

    #[test]
    fn test_day_result2() {
        assert_eq!(467835, day_result2(String::from("data/test.txt")));
    }

}

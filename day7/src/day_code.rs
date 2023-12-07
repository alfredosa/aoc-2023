use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Hand {
    hand_id: i32,
    cards: Vec<Card>,
    hand_type: HandType,
    bid: i32,
    rank: i32,
}

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    HighCard(CardType, Vec<Card>),
    OnePair(CardType, Vec<Card>),
    TwoPairs(CardType, CardType, Vec<Card>),
    ThreeOfAKind(CardType, Vec<Card>),
    FullHouse(CardType, CardType, Vec<Card>),
    FourOfAKind(CardType, Vec<Card>),
    FiveOfAKind(CardType, Vec<Card>),
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let (rank_self, cards_self) = self.rank();
        let (rank_other, cards_other) = other.rank();

        let result = rank_self.cmp(&rank_other)
        .then_with(|| cards_self.iter().zip(cards_other.iter()).find_map(|(a, b)| {
            if a != b {
                // println!("a {:?} b {:?} {:?}", a, b, b.cmp(a)); 
                Some(b.cmp(a))
            } else {
                None
            }
        }).unwrap_or(Ordering::Equal));

        // println!(
        //     "RESULT {:?} \n rank {:?} other {:?} \n cards self, {:?} cards other, {:?}",
        //     result, rank_self, rank_other, cards_self, cards_other
        // );

        result
    }
}

impl HandType {
    fn rank(&self) -> (u8, Vec<CardType>) {
        match self {
            HandType::HighCard(_, cards)
            | HandType::OnePair(_, cards)
            | HandType::ThreeOfAKind(_, cards)
            | HandType::FourOfAKind(_, cards)
            | HandType::FiveOfAKind(_, cards)
            | HandType::TwoPairs(_, _, cards)
            | HandType::FullHouse(_, _, cards) => {
                let mut sorted_card_types = cards
                    .iter()
                    .map(|card| card.card_type.clone())
                    .collect::<Vec<CardType>>();

                (self.get_hand_type_rank(), sorted_card_types)
            }
        }
    }

    fn get_hand_type_rank(&self) -> u8 {
        match self {
            HandType::HighCard(_, _) => 1,
            HandType::OnePair(_, _) => 2,
            HandType::TwoPairs(_, _, _) => 3,
            HandType::ThreeOfAKind(_, _) => 4,
            HandType::FullHouse(_, _, _) => 5,
            HandType::FourOfAKind(_, _) => 6,
            HandType::FiveOfAKind(_, _) => 7,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Ord, Eq)]
enum CardType {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Debug)]
struct Card {
    position: i32,
    card_type: CardType,
}

#[derive(Debug)]
struct CardCounter {
    card_type: CardType,
    count: i32,
}

impl Hand {
    fn new(line: String, line_num: i32) -> Hand {
        let splitted_line = line.split_whitespace().collect::<Vec<&str>>();
        let cards = splitted_line[0].trim();

        let mut cards_vec = Vec::new();
        let mut position = 0;

        cards.chars().for_each(|c| {
            let card = match c {
                'A' => Card {
                    position: position,
                    card_type: CardType::A,
                },
                'K' => Card {
                    position: position,
                    card_type: CardType::K,
                },
                'Q' => Card {
                    position: position,
                    card_type: CardType::Q,
                },
                'J' => Card {
                    position: position,
                    card_type: CardType::J,
                },
                'T' => Card {
                    position: position,
                    card_type: CardType::T,
                },
                '9' => Card {
                    position: position,
                    card_type: CardType::Nine,
                },
                '8' => Card {
                    position: position,
                    card_type: CardType::Eight,
                },
                '7' => Card {
                    position: position,
                    card_type: CardType::Seven,
                },
                '6' => Card {
                    position: position,
                    card_type: CardType::Six,
                },
                '5' => Card {
                    position: position,
                    card_type: CardType::Five,
                },
                '4' => Card {
                    position: position,
                    card_type: CardType::Four,
                },
                '3' => Card {
                    position: position,
                    card_type: CardType::Three,
                },
                '2' => Card {
                    position: position,
                    card_type: CardType::Two,
                },
                _ => Card {
                    position: position,
                    card_type: CardType::Two,
                },
            };

            cards_vec.push(card);
            position += 1;
        });

        let mut hand = Hand {
            hand_id: line_num,
            cards: cards_vec.clone(),
            hand_type: HandType::HighCard(CardType::A, cards_vec.clone()),
            bid: splitted_line[1].parse::<i32>().unwrap(),
            rank: 1,
        };
        hand.hand_type();
        hand
    }

    fn hand_type(&mut self) -> &Self {
        let mut card_totals: Vec<CardCounter> = Vec::new();
        for card in &self.cards {
            match card_totals
                .iter_mut()
                .find(|c| c.card_type == card.card_type)
            {
                Some(counter) => {
                    // If the CardCounter exists, increment the count
                    counter.count += 1;
                }
                None => {
                    // If the CardCounter doesn't exist, create a new one and push it to the vector
                    card_totals.push(CardCounter {
                        card_type: card.card_type.clone(),
                        count: 1,
                    });
                }
            }
        }

        // Sort the vector by count
        card_totals.sort_by(|a, b| b.count.cmp(&a.count));
        // j is now a Joker
        let mut jokers = 0;
        if card_totals.iter().find(|c| c.card_type == CardType::J).is_some() {
            jokers += card_totals.iter().find(|c| c.card_type == CardType::J).unwrap().count;
        }
        // Get highest count and card type
        let highest_count = card_totals[0].count + jokers;
        let second_highest = card_totals.get(1).map_or(0, |ct| ct.count);
        let highest_card_type = card_totals[0].card_type;
        let second_highest_card_type = card_totals.get(1).map_or(CardType::A, |ct| ct.card_type);

        match highest_count {
            1 => {
                self.hand_type = HandType::HighCard(highest_card_type, self.cards.clone());
            }
            2 => {
                if second_highest == 2 {
                    self.hand_type = HandType::TwoPairs(
                        highest_card_type,
                        second_highest_card_type,
                        self.cards.clone(),
                    );
                } else {
                    self.hand_type = HandType::OnePair(highest_card_type, self.cards.clone());
                }
            }
            3 => {
                if second_highest == 2 {
                    self.hand_type = HandType::FullHouse(
                        highest_card_type,
                        second_highest_card_type,
                        self.cards.clone(),
                    );
                } else {
                    self.hand_type = HandType::ThreeOfAKind(highest_card_type, self.cards.clone());
                }
            }
            4 => {
                self.hand_type = HandType::FourOfAKind(highest_card_type, self.cards.clone());
            }
            5 => {
                self.hand_type = HandType::FiveOfAKind(highest_card_type, self.cards.clone());
            }
            _ => {
                self.hand_type = HandType::HighCard(highest_card_type, self.cards.clone());
            }
        }

        self
    }

    fn rank_times_bid(&self) -> i64 {
        (self.rank * self.bid).into()
    }

    fn final_bids_by_rank(all_hands: Vec<Self>) -> i64 {
        let mut sorted_hands = Hand::get_rank(all_hands);
        let mut sum: i64 = 0;
        let mut rank = sorted_hands.len() as i32;
        sorted_hands.iter_mut().for_each(|hand| {
            // println!("cards {:?} >>>> RANK{:?}", hand.cards, rank);
            hand.rank = rank;
            sum += hand.rank_times_bid();
            rank -= 1;
        });

        sum
    }

    fn get_rank(all_hands: Vec<Self>) -> Vec<Self> {
        // sort hand by type and give a rank to each
        let mut sorted_hands = all_hands;
        sorted_hands.sort_by(|a, b| b.hand_type.cmp(&a.hand_type));

        sorted_hands
    }
}

pub fn day_result(path: String) -> usize {
    let mut data = read_input(path).unwrap();
    let _ = data.iter_mut().map(|h| h.hand_type());

    Hand::final_bids_by_rank(data) as usize
}

fn read_input(path: String) -> io::Result<Vec<Hand>> {
    let path = Path::new(&path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut all_hands = Vec::new();
    let mut line_counter = 0;
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let hand = Hand::new(line, line_counter);
        all_hands.push(hand);
        line_counter += 1;
    });

    Ok(all_hands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_result() {
        assert_eq!(5905, day_result(String::from("data/test.txt")));
    }

    // #[test]
    // fn test_day_result2() {
    //     assert_eq!(71503, day_result_pt2(String::from("data/test.txt")));
    // }
}

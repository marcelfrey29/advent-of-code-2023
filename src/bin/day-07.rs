use advent_of_code_2023::get_lines_from_file;
use std::{cmp::Ordering, collections::HashMap, env, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = get_lines_from_file(file_path);
    let result = get_total_winnings(lines);

    println!("Result: {}", result)
}

fn get_total_winnings(lines: Vec<String>) -> i64 {
    let mut winnings: Vec<Hand> = vec![];
    for line in lines {
        winnings.push(line.into());
    }

    winnings.sort();
    println!("Sorted Hands: {:?}", winnings);

    let mut result = 0;
    for hand_index in 0..winnings.len() {
        let hand = winnings.get(hand_index).expect("");
        result += format!("{}", (hand_index + 1)).parse::<i64>().expect("") * hand.strength;
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: String,
    strength: i64,
    type_of_hand: HandType,
}

impl From<String> for Hand {
    fn from(value: String) -> Self {
        let parts = value.split(' ').collect::<Vec<&str>>();
        let cards = parts.first().expect("").to_string();

        let mut map: HashMap<char, i32> = HashMap::new();
        let mut hand_type: HandType = HandType::HighCard(1);
        for card in cards.clone().chars() {
            if map.get(&card).is_none() {
                map.insert(card, 1);
            } else {
                map.insert(card, map.get(&card).expect("") + 1);
            }
        }
        // Only element in map: all elements are the same => Five of a Kind
        if map.len() == 1 {
            hand_type = HandType::FiveOfAKind(7)
        }
        // Two elements in map => Four of a Type OR Full House
        else if map.len() == 2 {
            if let Some((_key, value)) = map.iter().next() {
                if value == &3 || value == &2 {
                    hand_type = HandType::FullHouse(5)
                } else {
                    hand_type = HandType::FoutOfAKind(6)
                }
            } else {
                panic!("Expected elements in map.")
            }
        }
        // Three elements => Three of a Kind OR Two Pair
        else if map.len() == 3 {
            let mut has_three_same_cards = false;
            for (_key, value) in map.iter() {
                if value == &3 {
                    has_three_same_cards = true;
                }
            }

            if has_three_same_cards {
                hand_type = HandType::ThreeOfAKind(4);
            } else {
                hand_type = HandType::TwoPair(3);
            }
        }
        // Four elements: two cards must be the same => One Pair
        else if map.len() == 4 {
            hand_type = HandType::OnePair(2);
        }
        // No need for `else if map.len() == 5` or `else` because they would result
        // in `HandType::HighCard` which is already the default value for the
        // `hand_type` variable

        Hand {
            cards,
            strength: parts.get(1).expect("").parse::<i64>().expect(""),
            type_of_hand: hand_type,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rankings = vec![
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ];
        let card_ranks: HashMap<char, i32> = rankings.into_iter().collect();

        #[allow(clippy::comparison_chain)] // Allow if-else-if-else
        if self.type_of_hand < other.type_of_hand {
            Ordering::Greater
        } else if self.type_of_hand == other.type_of_hand {
            for i in 0..self.cards.len() {
                let current_char = card_ranks
                    .get(
                        &self
                            .cards
                            .chars()
                            .nth(i)
                            .expect("Expected Char at position."),
                    )
                    .expect("Expected ranking for char.");
                let other_char = card_ranks
                    .get(
                        &other
                            .cards
                            .chars()
                            .nth(i)
                            .expect("Expected Char at position."),
                    )
                    .expect("Expected ranking for char.");
                if current_char > other_char {
                    return Ordering::Greater;
                } else if current_char < other_char {
                    return Ordering::Less;
                }
                // When both chars are equal, we compare the next chars (next loop iteration)
            }
            // If all chars are the same, we return equal
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind(i32),
    FoutOfAKind(i32),
    FullHouse(i32),
    ThreeOfAKind(i32),
    TwoPair(i32),
    OnePair(i32),
    HighCard(i32),
}

#[test]
fn test_get_hand() {
    assert_eq!(
        Hand::from("11111 10".to_string()),
        Hand {
            cards: "11111".to_string(),
            strength: 10,
            type_of_hand: HandType::FiveOfAKind(7)
        }
    );
    assert_eq!(
        Hand::from("11110 10".to_string()),
        Hand {
            cards: "11110".to_string(),
            strength: 10,
            type_of_hand: HandType::FoutOfAKind(6)
        }
    );
    assert_eq!(
        Hand::from("11100 10".to_string()),
        Hand {
            cards: "11100".to_string(),
            strength: 10,
            type_of_hand: HandType::FullHouse(5)
        }
    );
    assert_eq!(
        Hand::from("11102 10".to_string()),
        Hand {
            cards: "11102".to_string(),
            strength: 10,
            type_of_hand: HandType::ThreeOfAKind(4)
        }
    );
    assert_eq!(
        Hand::from("11223 10".to_string()),
        Hand {
            cards: "11223".to_string(),
            strength: 10,
            type_of_hand: HandType::TwoPair(3)
        }
    );
    assert_eq!(
        Hand::from("11234 10".to_string()),
        Hand {
            cards: "11234".to_string(),
            strength: 10,
            type_of_hand: HandType::OnePair(2)
        }
    );
    assert_eq!(
        Hand::from("12345 10".to_string()),
        Hand {
            cards: "12345".to_string(),
            strength: 10,
            type_of_hand: HandType::HighCard(1)
        }
    );
}

#[test]
fn test_get_total_winnings() {
    assert_eq!(
        get_total_winnings(vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ]),
        6440
    )
}

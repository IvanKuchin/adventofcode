use itertools::Itertools;

// const ORDERING: &str = "AKQJT98765432";  // part 1
const ORDERING: &str = "AKQT98765432J";

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
    score: usize,
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.cards == other.cards
    }
}

impl std::cmp::Eq for Hand {}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.score < other.score {
            return Some(std::cmp::Ordering::Less);
        } else if self.score > other.score {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(compare_strings(&self.cards, &other.cards));
            // return Some(self.cards.cmp(&other.cards));
        }
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn compare_strings(a: &str, b: &str) -> std::cmp::Ordering {
    // let order = "AKQT98765432J";
    
    for i in 0..a.len() {
        let a_index = ORDERING.find(a.chars().nth(i).unwrap()).unwrap();
        let b_index = ORDERING.find(b.chars().nth(i).unwrap()).unwrap();

        if a_index < b_index {
            return std::cmp::Ordering::Less;
        } else if a_index > b_index {
            return std::cmp::Ordering::Greater;
        }
    }

    std::cmp::Ordering::Equal
}


fn freq_has_double(freq: &std::collections::HashMap<char, usize>, n: usize) -> bool {
    let mut instances = 0;

    for (_, v) in freq {
        if *v == n {
            instances += 1;
        }
    }

    if instances >= 2 {
        return true;
    }

    false
}

fn freq_has(freq: &std::collections::HashMap<char, usize>, n: usize) -> bool {
    for (_, v) in freq {
        if *v == n {
            return true;
        }
    }
    false
}

fn get_score(cards: &str) -> usize {
    // count cards frequency
    let mut freq = std::collections::HashMap::new();
    for c in cards.chars() {
        let counter = freq.entry(c).or_insert(0);
        *counter += 1;
    }

    if freq_has(&freq, 5) {
        return 1;
    } else if freq_has(&freq, 4) {
        return 2;
    } else if freq_has(&freq, 3) && freq_has(&freq, 2) {
        return 3;
    } else if freq_has(&freq, 3) {
        return 4;
    } else if freq_has_double(&freq, 2) {
        return 5;
    } else if freq_has(&freq, 2) {
        return 6;
    } else {
        return 7
    }
}

fn get_score_part2(cards: &str) -> usize {
    let j_count = cards.chars().filter(|c| *c == 'J').count();
    let cards_no_j = cards.chars().filter(|c| *c != 'J').collect::<String>();

    let cards_perm = ORDERING[..ORDERING.len() - 1].chars().collect::<Vec<_>>();
    let permutations = cards_perm
                            .iter()
                            .combinations_with_replacement(j_count)
                            .map(|f| f.iter().join(""))
                            .collect::<Vec<_>>();

    let mut min = get_score(cards);
    for perm in permutations {
        let card_to_check = cards_no_j.clone() + perm.as_str();
        let score = get_score(&card_to_check);
        if score < min {
            min = score;
        }
    }

    min
}

impl std::str::FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s[..5].to_string();
        let bid = s[6..].to_string().parse::<usize>().unwrap();
        // let score = get_score(&cards); // part 1
        let score = get_score_part2(&cards);

        Ok(Hand { cards, bid, score})
    }
}

fn part1(input: &str) {
    let content = std::fs::read_to_string(input).unwrap();
    let mut hands = content
                                    .lines()
                                    .map(|line| line.parse::<Hand>().unwrap())
                                    .collect::<Vec<_>>();
    // hands.sort_by(|a, b| a.score.cmp(&b.score).then(a.cards.cmp(&b.cards)));
    // hands.sort_by(|a, b| a.score.cmp(&b.score).then(compare_strings(&a.cards, &b.cards)));
    hands.sort_by(|a, b| a.cmp(&b));

    let mut sum = 0;
    for (i, hand) in hands.iter().rev().enumerate() {
        sum += hand.bid * (i + 1);
    }

    println!("Part 1: {}", sum);
}

// fn test_perm() {
//     let cards = "AKQT98765432J";
//     let mut cards = cards[..cards.len() - 1].chars().collect::<Vec<_>>();
//     let mut permutations = cards.iter().combinations_with_replacement(2).unique().collect::<Vec<_>>();

//     let perm_chars = permutations
//                                         .iter()
//                                         .map(|p| p.iter().join(""))
//                                         .collect::<Vec<_>>();
//     println!("permutations: {:?}", permutations);
//     println!("chars: {:?}", perm_chars);
// }

fn main() {
    part1("data/input.txt");
    // test_perm();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_1() {
        let hand1 = "23456 88".parse::<Hand>().unwrap();
        let hand2 = "23456 99".parse::<Hand>().unwrap();
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Equal);
    }
    #[test]
    fn compare_2() {
        let hand1 = "AAAAA 88".parse::<Hand>().unwrap();
        let hand2 = "AAAA6 99".parse::<Hand>().unwrap();

        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
    }
    #[test]
    fn compare_3() {
        let hand1 = "AAAAK 88".parse::<Hand>().unwrap();
        let hand2 = "AAAAA 99".parse::<Hand>().unwrap();
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Greater);
    }
    #[test]
    fn compare_4() {
        let hand1 = "2224K 88".parse::<Hand>().unwrap();
        let hand2 = "2233T 99".parse::<Hand>().unwrap();
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
    }
    #[test]
    fn compare_5() {
        let hand1 = "3344K 88".parse::<Hand>().unwrap();
        let hand2 = "2233T 99".parse::<Hand>().unwrap();
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
    }
    #[test]
    fn compare_6() {
        let hand1 = "T3456 88".parse::<Hand>().unwrap();
        let hand2 = "23456 99".parse::<Hand>().unwrap();
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
    }
    #[test]
    fn compare_7() {
        let hand1 = "T3456 88".parse::<Hand>().unwrap();
        let hand2 = "23456 99".parse::<Hand>().unwrap();
        assert_eq!(hand1 < hand2, true);
    }
    #[test]
    fn compare_8() {
        let hand1 = "23456 88".parse::<Hand>().unwrap();
        let hand2 = "23456 88".parse::<Hand>().unwrap();
        assert_eq!(hand1 == hand2, true);
    }
}
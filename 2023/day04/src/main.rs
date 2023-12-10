use std::usize;

#[derive(Debug)]
struct Card {
    id: i32,
    winning: Vec<i32>,
    my: Vec<i32>,
    winning_numbers: Vec<i32>,
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl std::str::FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let card_split = s.split(":").collect::<Vec<&str>>();
        let id = card_split[0]
                        .to_string()
                        .replace("Card", "")
                        .trim()
                        .parse::<i32>()
                        .unwrap();

        let numbers = card_split[1]
                        .split("|")
                        .collect::<Vec<&str>>();

        let winning = numbers[0]
                        .trim()
                        .replace("  ", " ")
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|x| x.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
        let my = numbers[1]
                        .trim()
                        .replace("  ", " ")
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|x| x.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

        Ok(Card { id, winning, my, winning_numbers: Vec::new() })
    }
}

impl std::str::FromStr for Deck {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = Vec::new();
        for line in s.lines() {
            let card = line.parse::<Card>().unwrap();
            cards.push(card);
        }
        Ok(Deck { cards })
    }
}

impl Card {
    fn count_winning_numbers(&mut self) {
        for number in &self.winning {
            if self.my.contains(number) {
                self.winning_numbers.push(*number);
            }
        }
    }
}

impl Deck {
    fn count_winning_cards(&mut self) {
        for i in 0..self.cards.len() {
            self.cards[i].count_winning_numbers();
        }
    }
}

fn part1(fname: &str) {
    let lines = std::fs::read_to_string(fname).expect("file not found");

    let mut deck = lines.parse::<Deck>().unwrap();
    deck.count_winning_cards();
    let sum = deck
                        .cards
                        .iter()
                        .map(|x| x.winning_numbers.len())
                        .collect::<Vec<usize>>()
                        .iter()
                        .filter(|x| **x > 0)
                        .fold(0, |acc, x| acc + (2 as i32).pow(x.clone() as u32 - 1));
    // println!("deck {:?}", deck);
    println!("sum {}", sum);
}

fn part2(fname: &str) {
    let lines = std::fs::read_to_string(fname).expect("file not found");

    let mut deck = lines.parse::<Deck>().unwrap();
    deck.count_winning_cards();
    let winning_counters = deck
                        .cards
                        .iter()
                        .map(|x| x.winning_numbers.len() as i32)
                        .collect::<Vec<i32>>();

    let mut card_stacks = winning_counters
                        .iter()
                        .map(|x| 1)
                        .collect::<Vec<i32>>();

    for idx in 1..winning_counters.len() {
        let curr_stack = card_stacks[idx-1];
        for i in idx..(idx + winning_counters[idx-1] as usize) {
            card_stacks[i] += curr_stack;
        }

    }

    let sum:i32 = card_stacks.iter().sum();
    
    println!("winning_counters {:?}", winning_counters);
    println!("card_stacks {:?}", card_stacks);
    println!("sum {}", sum);
}


fn main() {
    let fname = "data/input.txt";
    part2(fname);
}

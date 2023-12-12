use std::collections::HashMap;

use itertools::Itertools;

fn brute_force(broken: String, numbers: &Vec<usize>)  -> usize {
    let mut sum = 0;

    let question_marks = broken.chars().enumerate().filter(|(_, c)| *c == '?').map(|(i, _)| i);
    let numbers_sum = numbers.iter().sum::<usize>() as usize;
    let hash_count = broken.chars().filter(|c| *c == '#').count();
    let combinations = Itertools::combinations(question_marks.clone(), numbers_sum - hash_count);

    println!("combinations len: {}", combinations.clone().count());

    for (idx, combination) in combinations.clone().enumerate() {
        let mut new_map = broken.to_string();
        for place in combination {
            new_map.replace_range(place..place+1, "#");
        }

        let new_numbers = new_map.replace("?", ".").split(".").map(|x| x.len()).filter(|x| *x > 0).collect::<Vec<_>>();

        if new_numbers == *numbers {
            sum += 1;
        }

        if (idx + 1) % 10_000_000 == 0 {
            println!("*** iter {}M of {}", idx / 1_000_000, combinations.clone().count() / 1_000_000);
        }
    }

    println!("Finished broken map {} with total # of combinations is {}", broken, sum);
    sum
}

fn replicate(s: &str, n: usize, merge_symbol: char) -> String {
    let mut new_s = String::new();
    for i in 0..n {
        if i > 0 {
            new_s.push(merge_symbol);
        }
        new_s.push_str(s);
    }
    new_s
}

fn part1(fname: &str) {
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    let mut sum = 0;
    for line in lines {
        let split = line.split(" ").collect::<Vec<_>>();
        let (broken_map, right) = (split[0], split[1]);
        let numbers = right.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

        println!("{}: {:?}", broken_map, numbers);

        sum += brute_force(broken_map.to_string(), &numbers);
    }

    println!("Sum: {}", sum);
}

impl Cache
{

    fn dynamic_programming(&mut self, springs: &str, numbers: &[usize], group_length: i32) -> usize {
        if numbers.len() == 0 && group_length == -1 {
            let any_springs_left = springs.find("#").is_some();

            return if any_springs_left { 0 } else { 1 };
        }

        if springs.is_empty() {
            if group_length > 0 {
                return 0;
            } else {
                if numbers.is_empty() {
                    // println!("Found a match: springs {:?}, numbers {:?}", springs, numbers);
                    return 1;
                } else {
                    return 0;
                }
            }
        }

        match (&springs[0..1], group_length) {
            (".", -1) => return self.cached_dynamic_programming(&springs[1..], numbers, -1),
            (".", 0) => return self.cached_dynamic_programming(&springs[1..], numbers, -1),
            (".", _) => return 0,


            ("?", -1) => return self.cached_dynamic_programming(&springs[1..], numbers, -1) + self.cached_dynamic_programming(springs, &numbers[1..], numbers[0] as i32),
            ("?", 0) => return self.cached_dynamic_programming(&springs[1..], numbers, -1),
            ("?", _) => return self.cached_dynamic_programming(&springs[1..], numbers, group_length - 1),

            ("#", -1) => return self.cached_dynamic_programming(springs, &numbers[1..], numbers[0] as i32),
            ("#", 0) => return 0,
            ("#", _) => return self.cached_dynamic_programming(&springs[1..], numbers, group_length - 1),

            (_, _) => unimplemented!("Not implemented yet"),
        }

    }

    fn cached_dynamic_programming(&mut self, springs: &str, numbers: &[usize], group_length: i32) -> usize {
        if self.cache.contains_key(&(springs.to_string(), numbers.to_vec(), group_length)) {
            return *self.cache.get(&(springs.to_string(), numbers.to_vec(), group_length)).unwrap();
        }
        
        let result = self.dynamic_programming(springs, numbers, group_length);
        self.cache.insert((springs.to_string(), numbers.to_vec(), group_length), result);
        
        return result;
    }
}

struct Cache {
    cache: HashMap<(String, Vec<usize>, i32), usize>,
}



fn part2(fname:&str) {
    let mut cache = Cache {
        cache: HashMap::new(),
    };
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        let split = line.split(" ").collect::<Vec<_>>();
        let (broken_map, right) = (split[0], split[1]);
        let spring_map = replicate(broken_map, 5, '?');
        let right = replicate(right, 5, ',');
        let numbers = right.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

        // println!("{}: {:?}", spring_map, numbers);

        let combinations = cache.dynamic_programming(&spring_map, &numbers, -1);

        sum += combinations;
        println!("{:4}\tcombinations: {}, sum {}", i, combinations, sum);
    }

    println!("Sum: {}", sum);
}

fn main() {
    part2("data/input.prod");
}

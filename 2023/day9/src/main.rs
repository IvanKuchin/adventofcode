fn subtraction(top: Vec<i32>) -> i32 {
    if top.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut bottom = Vec::new();
    for i in 1..top.len() {
        bottom.push(top[i] - top[i-1]);
    }

    return top[top.len()-1] + subtraction(bottom);
}

fn part1(fname: &str) {
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content
                                .lines()
                                .map(|line| line
                                                    .split_whitespace()
                                                    .map(|x| x.parse::<i32>().unwrap())
                                                    .collect::<Vec<_>>()
                                )
                                .collect::<Vec<_>>();


    let mut sum = 0;

    for i in 0..lines.len() {
        sum += subtraction(lines[i].clone());
    }

    println!("Part 1: {}", sum);
}

fn part2(fname: &str) {
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content
                                .lines()
                                .map(|line| line
                                                    .split_whitespace()
                                                    .map(|x| x.parse::<i32>().unwrap())
                                                    .rev()
                                                    .collect::<Vec<_>>()
                                )
                                .collect::<Vec<_>>();

    let mut sum = 0;

    for i in 0..lines.len() {
        sum += subtraction(lines[i].clone());
    }

    println!("Part 2: {}", sum);
}

fn main() {
    part1("data/input.txt");
    part2("data/input.txt");
    // println!("{} {}", std::i32::MIN, std::i32::MAX);
}

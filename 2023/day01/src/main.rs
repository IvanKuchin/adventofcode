use std::io::{BufReader, BufRead};

fn part1() {
    let fname = "data/input.txt";
    let buffered = BufReader::new(std::fs::File::open(fname).unwrap());
    let mut numbers: Vec<i32> = Vec::new();

    buffered.lines().for_each(|line| {
        let line = line
                                .unwrap()
                                .chars()
                                .filter(|c| c.is_digit(10))
                                .collect::<String>();
        let num= format!("{}{}", &line[0..1], &line[line.len()-1..line.len()])
                        .parse::<i32>()
                        .unwrap();
        numbers.push(num);
    });

    let result = numbers.iter().sum::<i32>();
    println!("Result: {}", result);
}

fn part2() {
    let fname = "data/input.txt";
    let buffered = BufReader::new(std::fs::File::open(fname).unwrap());
    let mut numbers: Vec<i32> = Vec::new();

    buffered.lines()
            .for_each(|line| {
        let line2 = line
                                .unwrap();
        let line3 = line2
                                .replace("zero", "z0o")
                                .replace("one", "o1e")
                                .replace("two", "t2o")
                                .replace("three", "t3e")
                                .replace("four", "f4r")
                                .replace("five", "f5e")
                                .replace("six", "s6x")
                                .replace("seven", "s7n")
                                .replace("eight", "e8t")
                                .replace("nine", "n9e");
        let line4 = line3
                                .chars()
                                .filter(|c| c.is_digit(10))
                                .collect::<String>();
        let num= format!("{}{}", &line4[0..1], &line4[line4.len()-1..line4.len()])
                        .parse::<i32>()
                        .unwrap();
        numbers.push(num);

        println!("{} -> {} -> {}", line2, line3, num);
    });

    let result = numbers.iter().sum::<i32>();
    println!("Result: {}", result);
}


fn main() {
    // part1();
    part2();
}

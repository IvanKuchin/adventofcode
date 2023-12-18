use std::{collections::HashMap, vec};


const R : (i64, i64) = (0, 1);
const L : (i64, i64) = (0, -1);
const U : (i64, i64) = (-1, 0);
const D : (i64, i64) = (1, 0);

fn part1(fname: &str) -> i64 {
    let contents = std::fs::read_to_string(fname).unwrap();
    let lines = contents.lines().collect::<Vec<_>>();

    let directions: HashMap<&str, (i64, i64)> = [
        ("R", R),
        ("L", L),
        ("U", U),
        ("D", D),
    ].iter().cloned().collect();

    let mut perimeter = 0;
    let mut vertexes = vec![(0, 0)];

    for line in lines {
        let split = line.split(" ").collect::<Vec<_>>();
        let (dir, dist) = (split[0], split[1].parse::<i64>().unwrap());
        let dir = (directions[dir].0 * dist, directions[dir].1 * dist);

        vertexes.push((vertexes.last().unwrap().0 + dir.0, vertexes.last().unwrap().1 + dir.1));
        perimeter += dir.0.abs() + dir.1.abs();
    }

    let (ys, xs):(Vec<i64>, Vec<i64>) = vertexes.iter().cloned().unzip();

    let right = xs.iter().zip(ys[1..].iter()).collect::<Vec<_>>();
    let  left = ys.iter().zip(xs[1..].iter()).collect::<Vec<_>>();

    println!("len: {}, {}", right.len(), left.len());

    let right_footage: i64 = right
                                .iter()
                                .map(|(x, y)| **x * **y)
                                .sum();

    let left_footage: i64 = left
                                .iter()
                                .map(|(x, y)| **x * **y)
                                .sum(); 

    println!("{} {} {}", left_footage, right_footage, perimeter);

    (left_footage - right_footage).abs() / 2 + perimeter/2 + 1
}

fn part2(fname: &str) -> i64 {
    let contents = std::fs::read_to_string(fname).unwrap();
    let lines = contents.lines().collect::<Vec<_>>();

    let directions: HashMap<&str, (i64, i64)> = [
        ("0", R),
        ("2", L),
        ("3", U),
        ("1", D),
    ].iter().cloned().collect();

    let mut perimeter = 0;
    let mut vertexes = vec![(0, 0)];

    for line in lines {
        let split = line.split(" ").collect::<Vec<_>>();
        let hex = split[2];

        let dist = &hex[2..hex.len()-2];
        let dist = i64::from_str_radix(dist, 16).unwrap();

        let dir = &hex[hex.len()-2..hex.len()-1];
        let dir = (directions[dir].0 * dist, directions[dir].1 * dist);

        vertexes.push((vertexes.last().unwrap().0 + dir.0, vertexes.last().unwrap().1 + dir.1));
        perimeter += dir.0.abs() + dir.1.abs();
    }

    let (ys, xs):(Vec<i64>, Vec<i64>) = vertexes.iter().cloned().unzip();

    let right = xs.iter().zip(ys[1..].iter()).collect::<Vec<_>>();
    let  left = ys.iter().zip(xs[1..].iter()).collect::<Vec<_>>();

    println!("len: {}, {}", right.len(), left.len());

    let right_footage: i64 = right
                                .iter()
                                .map(|(x, y)| **x * **y)
                                .sum();

    let left_footage: i64 = left
                                .iter()
                                .map(|(x, y)| **x * **y)
                                .sum(); 

    println!("{} {} {}", left_footage, right_footage, perimeter);

    (left_footage - right_footage).abs() / 2 + perimeter/2 + 1
}

fn main() {
    println!("Part 1: {}", part1("data/prod"));
    println!("Part 2: {}", part2("data/prod"));
}

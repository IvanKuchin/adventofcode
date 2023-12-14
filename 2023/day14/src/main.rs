use std::collections::HashMap;

const FINAL: usize = 1_000_000_000;

enum Direction {
    North,
    South,
    East,
    West,
}

fn reverse_tilt_north(src: &Vec<String>) -> Vec<String> {
    transpose(src)
}

fn tilt_north(src: &Vec<String>) -> Vec<String> {
    transpose(src)
}

fn tilt_south(src: &Vec<String>) -> Vec<String> {
    let transposed = transpose(src);
    let mut dst = vec![];

    for line in transposed {
        dst.push(line.chars().rev().collect::<String>());
    }

    dst
}

fn reverse_tilt_south(src: &Vec<String>) -> Vec<String> {
    let mut dst = vec![];

    for line in src {
        dst.push(line.chars().rev().collect::<String>());
    }

    transpose(&dst)
}

fn tilt_east(src: &Vec<String>) -> Vec<String> {
    let mut dst = vec![];

    for line in src {
        dst.push(line.chars().rev().collect::<String>());
    }

    dst
}

fn reverse_tilt_east(src: &Vec<String>) -> Vec<String> {
    let mut dst = vec![];

    for line in src {
        dst.push(line.chars().rev().collect::<String>());
    }

    dst
}

fn tilt_west(src: &Vec<String>) -> Vec<String> {
    src.clone()
}

fn reverse_tilt_west(src: &Vec<String>) -> Vec<String> {
    src.clone()
}

fn tilt(src: &Vec<String>, direction: Direction) -> Vec<String> {
    let prep = match direction {
        Direction::North => tilt_north(src),
        Direction::South => tilt_south(src),
        Direction::East => tilt_east(src),
        Direction::West => tilt_west(src),
    };

    // println!("prep: {:#?}", prep);

    let mut dst = vec![];

    for line_from  in prep {
        let mut line_to = line_from.chars().map(|_| '.').collect::<String>();
        let mut place = 0;

        for (i, c) in line_from.chars().enumerate() {
            match c {
                '#' => {
                    line_to.replace_range(i..i+1, "#");
                    place = i + 1;
                },
                'O' => {
                    line_to.replace_range(place..place+1, "O");
                    place += 1;
                },
                _ => {},
            }
        }
        dst.push(line_to);
    }

    // println!("dst: {:#?}", dst);

    match direction {
        Direction::North => reverse_tilt_north(&dst),
        Direction::South => reverse_tilt_south(&dst),
        Direction::East => reverse_tilt_east(&dst),
        Direction::West => reverse_tilt_west(&dst),
    }
}

fn transpose(src: &Vec<String>) -> Vec<String> {
    if src.len() == 0 {
        return vec![];
    }

    let dst = (0..src[0].len())
                                .map(|x| {
                                    src
                                        .iter()
                                        .map(|line| line.chars().nth(x).unwrap())
                                        .collect::<String>()
                                })
                                .collect::<Vec<_>>();

    dst
}

fn put_on_scale(src: &Vec<String>) -> usize {
    let transposed = transpose(&src);
    let mut weight = 0;

    for line in transposed {
        for (i, c) in line.chars().rev().enumerate() {
            if c == 'O' {
                weight += i+1;
            }
        }
    }

    weight
}

fn part1(fname: &str) -> usize {
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content.lines().map(|x| x.to_string()).collect::<Vec<_>>();

    let tilted = tilt(&lines, Direction::North);

    println!("tilted: {:#?}", tilted);

    put_on_scale(&tilted)
}

#[derive(Debug)]
struct Metrics {
    step: usize,
    cycle: usize,
}

impl std::fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ step: {}, cycle: {} }}", self.step, self.cycle)
    }
}

fn part2(fname: &str) -> usize {
    let content = std::fs::read_to_string(fname).unwrap();
    let mut start = content.lines().map(|x| x.to_string()).collect::<Vec<_>>();
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut cache_step: HashMap<String, Metrics> = HashMap::new();

    let mut step = 1;
    loop {
        let key = start.join("|");

        if cache.contains_key(&key) {
            start = cache.get(&key).unwrap().clone();
            let metrics = cache_step.get(&key).unwrap();

            let leap = step - metrics.step;

            if step + leap > FINAL {
                cache.clear();
                cache_step.clear();
            } else {
                println!("leap is {} jump from step {} -> {}", leap, step, step + step - metrics.step);

                step += leap;
                cache_step.insert(key, Metrics { step, cycle:0 });
            }


            

            // if step - metrics.step != metrics.cycle {
            //     println!("step: {:#?}, cycle from cache: {:#?}, counted cycle: {:#?}, {:?}", step, metrics.cycle, step - metrics.step, metrics);
            // }

            // let cycle = step - metrics.step;
            // cache_step.insert(key, Metrics { step, cycle });

            // if (FINAL - step - 1) % cycle == 0 {
            //     println!("solution found on step {}", step);
            //     return put_on_scale(&start);
            // }

        } else {
            let tilted_n = tilt(&start, Direction::North);
            let tilted_w = tilt(&tilted_n, Direction::West);
            let tilted_s = tilt(&tilted_w, Direction::South);
            let tilted_e = tilt(&tilted_s, Direction::East);
    
            cache.insert(start.join("|"), tilted_e.clone());
            cache_step.insert(start.join("|"), Metrics { step, cycle: 0 });
    
            start = tilted_e.clone();
        }


        if step+1 % 10_000 == 0 {
            println!("cycle: {:#?}M, weight {}", step / 1_000_000, put_on_scale(&start));
        }
        
        if step >= FINAL - 10 {
            println!("cycle: {:#?}, weight {}", step, put_on_scale(&start));
        }

        if step >= FINAL {
            break;
        }

        step += 1;
    }

    put_on_scale(&start)
}

fn main() {
    println!("part1 {}", part1("data/dev1"));
    println!("part2 {}", part2("data/prod"));
}

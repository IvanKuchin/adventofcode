use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Walk {
    step: usize,
    pace: usize,
}

#[derive(Debug)]
struct Map {
    entry_points: Vec<String>,
    map: HashMap<String, String>,
    walking_in_a_loop: Vec<Walk>,
}

impl std::str::FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut entry_points = Vec::new();
        for line in s.lines() {

            if line.find(" = ").is_none() {
                continue;
            }

            let mut parts = line.split('=');
            let a = parts
                                .next()
                                .unwrap()
                                .trim();
            let b = parts
                                .next()
                                .unwrap()
                                .trim();
            let directions = b[1..b.len()-1].split(',').collect::<Vec<&str>>();

            map.insert(a.to_string() + "_L", directions[0].trim().to_string());
            map.insert(a.to_string() + "_R", directions[1].trim().to_string());

            entry_points.push(a.to_string());
        }
        Ok(Map { map, entry_points, walking_in_a_loop: Vec::new() })
    }
}

impl Map {
    fn walk(&self, moves: &Vec<char>) -> Vec<String> {
        let mut steps = Vec::new();
        let mut current_node = "AAA".to_string();
        let mut i = 0;
        while &current_node != "ZZZ" {
            let l_or_r = moves.get(i % moves.len()).unwrap();
            let step = {
                if l_or_r == &'L' {
                    current_node.to_string() + "_L"
                } else {
                    current_node.to_string() + "_R"
                }
            };
            let next_node = self.map.get(&step).unwrap();
            steps.push(next_node.clone());
            current_node = next_node.clone();
            i += 1;
        }
        steps
    }

    fn find_start_nodes(& mut self) -> Vec<String> {
        let mut start_nodes = Vec::new();
        // for ep in self.entry_points.iter() {
        for i in 0..self.entry_points.len() {
            if &self.entry_points[i][2..3] == "A" {
                start_nodes.push(self.entry_points[i].clone());
                self.walking_in_a_loop.push(Walk { step: 0, pace: 0 });
            }
        }
        start_nodes
    }
    
    fn make_a_step(&self, step_number:usize, current_nodes: &Vec<String>, moves: &Vec<char>) -> Vec<String> {
        let mut steps_to_record = Vec::new();

        for i in 0..current_nodes.len() {
            let l_or_r = moves.get(step_number % moves.len()).unwrap();
            let step;
            if l_or_r == &'L' {
                step = current_nodes[i].to_string() + "_L";
            } else {
                step = current_nodes[i].to_string() + "_R";
            }
            let next_node = self.map.get(&step).unwrap();
            steps_to_record.push(next_node.clone());
        }

        steps_to_record
    }

    fn parallel_walk(& mut self, moves: &Vec<char>) -> Vec<Walk> {
        let mut steps: Vec<Vec<String>> = Vec::new();

        let mut current_nodes = self.find_start_nodes();
        steps.push(current_nodes.clone());

        let mut step: usize = 0;

        while self.keep_walking(&current_nodes, step) && !self.walking_in_a_circle(){
            current_nodes = self.make_a_step(step, &current_nodes, moves);
            // steps.push(current_nodes.clone());
            step += 1;

            if step % 100_000_000 == 0 {
                println!("Step: {}M", step/1_000_000);
            }
        }

        self.walking_in_a_loop.clone()
    }

    fn walking_in_a_circle(&self) -> bool {
        for i in 0..self.walking_in_a_loop.len() {
            if self.walking_in_a_loop[i].pace == 0 {
                return false;
            }
        }
        true
    }

    fn keep_walking(&mut self, current_nodes: &Vec<String>, step: usize) -> bool {
        let mut result = false;

        for i in 0..current_nodes.len() {
            if &current_nodes[i][2..3] == "Z" {
                let pace = step - self.walking_in_a_loop[i].step;

                self.walking_in_a_loop[i].pace = pace;
                self.walking_in_a_loop[i].step = step;

                if  pace != self.walking_in_a_loop[i].pace && self.walking_in_a_loop[i].pace != 0{
                    panic!("At step {} pace changed from {} to {}", step, self.walking_in_a_loop[i].pace, pace)
                }
                continue;

            } else {
                result = true;
            }
        }
        result
    }
}

fn part1(fname: &str) {
    let content = std::fs::read_to_string(fname).expect("Could not read file");
    let lines = content.lines().collect::<Vec<&str>>();

    let moves = lines[0].chars().collect::<Vec<char>>();
    let map = content.parse::<Map>().expect("Could not parse map");
    let steps = map.walk(&moves);

    println!("Moves: {:?}", moves);
    // println!("Steps: {:?}", steps);
    println!("Steps: {}", steps.len());
}

fn part2 (fname: &str) {
    let content = std::fs::read_to_string(fname).expect("Could not read file");
    let lines = content.lines().collect::<Vec<&str>>();

    let moves = lines[0].chars().collect::<Vec<char>>();
    let mut map = content.parse::<Map>().expect("Could not parse map");
    let steps = map.parallel_walk(&moves);

    let multipliers = steps.iter().map(|x| x.pace).collect::<Vec<usize>>();

    println!("Moves: {:?}", moves);
    // println!("Steps: {:?}", steps);
    println!("Steps till finish: {:?}", steps);
    println!("Multipliers: {:?}", multipliers);
    println!("Least common multiple: {:?}", least_comon_multiple(multipliers));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn least_comon_multiple(v: Vec<usize>) -> usize {
    let mut lcm = v[0];
    for i in 1..v.len() {
        lcm = lcm * v[i] / gcd(lcm, v[i]);
    }
    lcm
}

fn main() {
    let fname = "data/input.txt";

    // part1(fname);
    part2(fname);

}

use std::{collections::{HashMap, VecDeque, HashSet}, cmp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dir(i32, i32);

const N: Dir = Dir(-1, 0);
const S: Dir = Dir(1, 0);
const E: Dir = Dir(0, 1);
const W: Dir = Dir(0, -1);

fn bfs(maze: HashMap<Pos, char>, start: Pos, start_dir: Dir) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, start_dir));

    let mut visited = HashMap::new();

    while !queue.is_empty() {
        let (pos, dir) = queue.pop_front().unwrap();
        
        if maze.get(&pos).is_none() { continue; }
        if visited.get(&(pos, dir)).is_some() { continue; }
        
        visited.insert((pos, dir), 1);
        let &curr_cell = maze.get(&pos).unwrap();

        if curr_cell == '/' {
            let new_dir = Dir(-dir.1, -dir.0);
            queue.push_back((Pos(pos.0 + new_dir.0, pos.1 + new_dir.1), new_dir));
        } else if curr_cell == '\\' {
            let new_dir = Dir(dir.1, dir.0);
            queue.push_back((Pos(pos.0 + new_dir.0, pos.1 + new_dir.1), new_dir));
        } else if curr_cell == '|' && (dir == E || dir == W) {
            queue.push_back((Pos(pos.0 + 1, pos.1), Dir(1, 0)));
            queue.push_back((Pos(pos.0 - 1, pos.1), Dir(-1, 0)));
        } else if curr_cell == '-' && (dir == N || dir == S) {
            queue.push_back((Pos(pos.0, pos.1 + 1), Dir(0, 1)));
            queue.push_back((Pos(pos.0, pos.1 - 1), Dir(0, -1)));
        } else {
            queue.push_back((Pos(pos.0 + dir.0, pos.1 + dir.1), dir));
        }
    }
    
    let visited_pos = visited
                                                    .keys()
                                                    .map(|(pos, _)| pos)
                                                    .collect::<HashSet<_>>();

    visited_pos.len()
}

fn part1(_filename: &str) -> usize {
    let contet = std::fs::read_to_string(_filename).unwrap();
    let lines = contet.lines();

    let mut maze = HashMap::new();
    for (i, line) in lines.enumerate() {
        for (j, char) in line.chars().enumerate() {
            maze.insert(Pos(i as i32, j as i32), char);
        }
    }
    
    bfs(maze, Pos(0, 0), E)
}

fn part2(_filename: &str) -> usize {
    let contet = std::fs::read_to_string(_filename).unwrap();
    let lines = contet.lines();

    let cols = lines.clone().next().unwrap().len();
    let rows = lines.clone().count();

    let mut maze = HashMap::new();
    for (i, line) in lines.enumerate() {
        for (j, char) in line.chars().enumerate() {
            maze.insert(Pos(i as i32, j as i32), char);
        }
    }
 
    let mut max = 0;

    for i in 0..rows as i32 {
        max = cmp::max(max, bfs(maze.clone(), Pos(i as i32, 0), E));
        max = cmp::max(max, bfs(maze.clone(), Pos(i as i32, (cols - 1) as i32), W));
    }

    for i in 0..cols as i32 {
        max = cmp::max(max, bfs(maze.clone(), Pos(0, i as i32), S));
        max = cmp::max(max, bfs(maze.clone(), Pos((rows - 1) as i32, i as i32), N));
    }  

    max
}

fn main() {
    // println!("Part1 {}", part1("data/prod"));
    println!("Part2 {}", part2("data/prod"));
}

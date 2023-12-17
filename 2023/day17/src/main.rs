use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Pos (i32, i32);

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Run(i32, i32);

fn get_neighbors1(pos: Pos, run: Run) -> Vec<(Pos, Run)> {
    let mut result = vec![];

    if run.0 == 0 {
        result.push((Pos(pos.0 - 1, pos.1), Run(-1, 0)));
        result.push((Pos(pos.0 + 1, pos.1), Run(1, 0)));
    }
    if run.1 == 0 {
        result.push((Pos(pos.0, pos.1 - 1), Run(0, -1)));
        result.push((Pos(pos.0, pos.1 + 1), Run(0, 1)));
    }
    if run.0 > 0 {
        result.push((Pos(pos.0 + 1, pos.1), Run(run.0 + 1, 0)));
    }
    if run.0 < 0 {
        result.push((Pos(pos.0 - 1, pos.1), Run(run.0 - 1, 0)));
    }
    if run.1 > 0 {
        result.push((Pos(pos.0, pos.1 + 1), Run(0, run.1 + 1)));
    }
    if run.1 < 0 {
        result.push((Pos(pos.0, pos.1 - 1), Run(0, run.1 - 1)));
    }

    result
}

fn print_maze(maze: &HashMap<Pos, char>, prev: &HashMap<(Pos, Run), ((Pos, Run), usize)>, end: &(Pos, Run)) {
    let mut pos = end.0;
    let mut run = end.1;
    let mut path = HashMap::new();
    let mut cost = vec![];

    while let Some(((prev_pos, prev_run), local_dist)) = prev.get(&(pos, run)) {
        path.insert(pos, run);
        cost.push(local_dist);

        pos = *prev_pos;
        run = *prev_run;
    }

    cost.reverse();

    println!("Cost: {:?}", cost);

    for i in 0..6 {
        for j in 0..15 {
            let run = path.get(&Pos(i as i32, j as i32));
            if run.is_some() {
                let run = run.unwrap();
                if run.0 < 0 {
                    print!("^");
                } else if run.0 > 0 {
                    print!("v");
                } else if run.1 < 0 {
                    print!("<");
                } else if run.1 > 0 {
                    print!(">");
                } else {
                    print!("?");
                }

            } else if maze.contains_key(&Pos(i as i32, j as i32)) {
                print!("{}", maze.get(&Pos(i as i32, j as i32)).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn dijkstra1(maze: &HashMap<Pos, char>, start: Pos, end: Pos) -> usize {
    let mut dist = HashMap::new();
    let mut pq = std::collections::BTreeSet::new();
    let mut prev = HashMap::new();

    dist.insert((start, Run(0,0)), 0);
    pq.insert((0, (start, Run(0,0))));

    while let Some((d, (pos, run))) = pq.pop_first() {
        if run.0 < -3 || 3 < run.0 { continue; }
        if run.1 < -3 || 3 < run.1 { continue; }

        if pos == end {
            // print_maze(&maze, &prev, &(end, run)); 
            return d; 
        }

        let neighbors = get_neighbors1(pos, run);

        for (nei_pos, new_run) in neighbors {
            if !maze.contains_key(&nei_pos) { continue; }
            let new_dist = d + maze.get(&nei_pos).unwrap().to_digit(10).unwrap() as usize;
            let new_key = (nei_pos, new_run);
            if !dist.contains_key(&new_key) || new_dist < *dist.get(&new_key).unwrap() {
                dist.insert(new_key, new_dist);
                pq.insert((new_dist, new_key));
                prev.insert(new_key, ((pos, run), new_dist));
            }
        }
    }

    0
}

fn part1(fname: &str) -> usize {
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    let mut maze = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            maze.insert(Pos(i as i32, j as i32), c);
        }
    }

    dijkstra1(&maze, Pos(0, 0), Pos((lines.len() - 1) as i32, (lines[0].len() - 1) as i32))
}

fn get_neighbors2(pos: Pos, run: Run) -> Vec<(Pos, Run)> {
    let mut result = vec![];

    if 0 < run.0 && run.0 < 4 {
        result.push((Pos(pos.0 + 1, pos.1), Run(run.0 + 1, 0)));
    } else if -4 < run.0 && run.0 < 0 {
        result.push((Pos(pos.0 - 1, pos.1), Run(run.0 - 1, 0)));
    } else if 0 < run.1 && run.1 < 4  {
        result.push((Pos(pos.0, pos.1 + 1), Run(0, run.1 + 1)));
    } else if -4 < run.1 && run.1 < 0 {
        result.push((Pos(pos.0, pos.1 - 1), Run(0, run.1 - 1)));
    } else {
        result = get_neighbors1(pos, run);
    }

    result
}

fn dijkstra2(maze: &HashMap<Pos, char>, start: Pos, end: Pos) -> usize {
    let mut dist = HashMap::new();
    let mut pq = std::collections::BTreeSet::new();

    dist.insert((start, Run(0,0)), 0);
    pq.insert((0, (start, Run(0,0))));

    while let Some((d, (pos, run))) = pq.pop_first() {
        if run.0 < -10 || 10 < run.0 { continue; }
        if run.1 < -10 || 10 < run.1 { continue; }

        if pos == end && (4 <= run.0 || 4 <= run.1 || run.0 <= -4 || run.1 <= -4) { return d; }

        let neighbors = get_neighbors2(pos, run);

        for (nei_pos, new_run) in neighbors {
            if !maze.contains_key(&nei_pos) { continue; }
            let new_dist = d + maze.get(&nei_pos).unwrap().to_digit(10).unwrap() as usize;
            let new_key = (nei_pos, new_run);
            if !dist.contains_key(&new_key) || new_dist < *dist.get(&new_key).unwrap() {
                dist.insert(new_key, new_dist);
                pq.insert((new_dist, new_key));
            }
        }
    }

    0
}

fn part2(fname: &str) -> usize {
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    let mut maze = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            maze.insert(Pos(i as i32, j as i32), c);
        }
    }

    dijkstra2(&maze, Pos(0, 0), Pos((lines.len() - 1) as i32, (lines[0].len() - 1) as i32))
}

fn main() {
    println!("Part1: {}", part1("data/prod"));
    println!("Part2: {}", part2("data/prod"));
}

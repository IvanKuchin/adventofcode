use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct Direction(i32, i32);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TileType {
    Empty,
    Start,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl TileType {
    fn directions(&self) -> Vec<Direction> {
        match self {
            TileType::NorthSouth    => vec![Direction(-1,  0), Direction( 1,  0)],
            TileType::EastWest      => vec![Direction( 0, -1), Direction( 0,  1)],
            TileType::NorthEast     => vec![Direction(-1,  0), Direction( 0,  1)],
            TileType::NorthWest     => vec![Direction(-1,  0), Direction( 0, -1)],
            TileType::SouthEast     => vec![Direction( 1,  0), Direction( 0,  1)],
            TileType::SouthWest     => vec![Direction( 1,  0), Direction( 0, -1)],
            TileType::Empty         => vec![],
            TileType::Start         => vec![Direction(-1, -1), Direction(-1,  0),Direction(-1,  1), Direction(0,  -1),Direction(0,  1),Direction(1, -1), Direction(1,  0),Direction(1,  1)],
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    y: i32,
    x: i32,
    tile_type: TileType,
}

impl Tile {
    fn directions(&self) -> Vec<Direction> {
        self.tile_type.directions()
    }

    fn add(&self, directions: &Vec<Direction>, tiles: &HashMap<String, Tile>) -> VecDeque<Tile> {
        let mut result = VecDeque::new();
        for direction in directions {
            let new_y = self.y + direction.0;
            let new_x = self.x + direction.1;

            let new_tile = tiles.get(&format!("{}:{}", new_y, new_x));
            if new_tile.is_none() {
                continue;
            }

            result.push_back(*new_tile.unwrap());

        }
        result
    }
}

fn tiles_from_maze(lines: &Vec<String>) -> Vec<Vec<TileType>> {
    let mut tiles = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '|' => TileType::NorthSouth,
                '-' => TileType::EastWest,
                'L' => TileType::NorthEast,
                'J' => TileType::NorthWest,
                '7' => TileType::SouthWest,
                'F' => TileType::SouthEast,
                'S' => TileType::Start,
                '.' => TileType::Empty,
                _ => panic!("Unknown tile ASCII symbol: {}", c),
            });
        }
        tiles.push(row);
    }
    tiles
}

fn coords_from_maze(tiles: &Vec<String>, tile_types:&Vec<Vec<TileType>>) -> HashMap<String, Tile> {
    let mut coords = HashMap::new();
    for (y, row) in tiles.iter().enumerate() {
        for (x, _) in row.chars().enumerate() {
            coords.insert(format!("{}:{}", y, x), Tile { x:x as i32, y: y as i32, tile_type: tile_types[y][x] });
        }
    }
    coords
}

fn find_start_tile(tiles: &Vec<Vec<TileType>>) -> Tile {
    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == TileType::Start {
                return Tile {
                    x: x as i32,
                    y: y as i32,
                    tile_type: TileType::Start,
                };
            }
        }
    }
    panic!("No start tile found");
}

fn are_tiles_adjacent(tile1: &Tile, tile2: &Tile) -> bool {
    let directions = tile1.directions();
    for direction in directions {
        let adj_x = tile1.x + direction.1;
        let adj_y = tile1.y + direction.0;

        if adj_x == tile2.x && adj_y == tile2.y {
            return true;
        }
    }
    false
}

fn are_tiles_mutually_adjacent(tile1: &Tile, tile2: &Tile) -> bool {
    are_tiles_adjacent(tile1, tile2) && are_tiles_adjacent(tile2, tile1)
}

fn get_directions_from_start(start_tile: &Tile, tile_types: &Vec<Vec<TileType>>, tiles: &HashMap<String, Tile>) -> Vec<Direction> {
    let mut result = Vec::new();
    for direction in &start_tile.directions() {
        let adj_x = start_tile.x + direction.1;
        let adj_y = start_tile.y + direction.0;

        let adj_tile = tiles.get(&format!("{}:{}", adj_y, adj_x)).copied();
        if adj_tile.is_none() {
            continue;
        }

        let adj_tile = adj_tile.unwrap();

        if are_tiles_mutually_adjacent(&start_tile, &adj_tile) {
            // println!("Tile {:?} is adjacent to start tile {:?}", adj_tile, start_tile);
            result.push(direction.clone());
        }
    }

    result
}

#[derive(Debug)]
struct Route (VecDeque<Tile>);
impl Route {
    fn new() -> Route {
        Route(VecDeque::new())
    }

    fn add_moves(&mut self, src: &Tile, field: &HashMap<String, Tile>) {
        let directions = src.directions();
        for direction in directions {
            let adj_y = src.y + direction.0;
            let adj_x = src.x + direction.1;

            let adj_tile = field.get(&format!("{}:{}", adj_y, adj_x)).copied();
            if adj_tile.is_none() {
                continue;
            }

            let adj_tile = adj_tile.unwrap();

            if are_tiles_mutually_adjacent(&src, &adj_tile) {
                self.0.push_back(adj_tile);
            } else {
                panic!("Tile {:?} is not adjacent to start tile {:?}", adj_tile, src)
            }
        }
    }
}

fn part1(fname: &str) {
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content
                                    .lines()
                                    .map(|line| line.parse::<String>().unwrap())
                                    .collect::<Vec<_>>();

    let tile_types = tiles_from_maze(&lines);
    let field = coords_from_maze(&lines, &tile_types);

    let start_tile = find_start_tile(&tile_types);
    // println!("Start tile: {:?}", start_tile);

    let start_directions = get_directions_from_start(&start_tile, &tile_types, &field);
    // println!("Start directions: {:?}", start_directions);

    let tiles_from_start = start_tile.add(&start_directions, &field);
    let mut to_visit = Route(tiles_from_start);
    // println!("To visit: {:?}", to_visit);

    let mut distance = 0;
    let mut visited: HashMap<String, i32> = HashMap::new();
    visited.insert(format!("{}:{}", start_tile.y, start_tile.x), distance);

    while(to_visit.0.len() > 0) {

        distance += 1;

        for i in 0..to_visit.0.len() {
            let tile = to_visit.0.pop_front().unwrap();
            // println!("Tile: {:?}", tile);

            let coords = format!("{}:{}", tile.y, tile.x);
            if visited.contains_key(&coords) {
                continue;
            }

            visited.insert(coords.clone(), distance);

            to_visit.add_moves(&field.get(&coords).unwrap(), &field);
        }
    }

    // println!("Visited: {:?}", visited);

    let max = visited.values().max().unwrap();
    println!("Max distance: {}", max);
}

fn part2(fname: &str) {
    let content = std::fs::read_to_string(fname).unwrap();
    let lines = content
                                    .lines()
                                    .map(|line| line.parse::<String>().unwrap())
                                    .collect::<Vec<_>>();

    let tile_types = tiles_from_maze(&lines);
    let field = coords_from_maze(&lines, &tile_types);

    let start_tile = find_start_tile(&tile_types);
    // println!("Start tile: {:?}", start_tile);

    let mut start_directions = get_directions_from_start(&start_tile, &tile_types, &field);
    start_directions.pop(); // force to go to one direction only

    let tiles_from_start = start_tile.add(&start_directions, &field);
    let mut to_visit = Route(tiles_from_start);
    // println!("To visit: {:?}", to_visit);

    let mut distance = 0;
    let mut visited: HashMap<String, i32> = HashMap::new();
    visited.insert(format!("{}:{}", start_tile.y, start_tile.x), distance);

    while(to_visit.0.len() > 0) {

        distance += 1;

        for i in 0..to_visit.0.len() {
            let tile = to_visit.0.pop_front().unwrap();
            // println!("Tile: {:?}", tile);

            let coords = format!("{}:{}", tile.y, tile.x);
            if visited.contains_key(&coords) {
                continue;
            }

            visited.insert(coords.clone(), distance);

            to_visit.add_moves(&field.get(&coords).unwrap(), &field);
        }
    }

    let max_steps = visited.values().max().unwrap();
    println!("Max distance: {}", max_steps);

    println!("Visited: {:?}", visited);
    // --------------------------

    let mut visited_lines = visited
                                                    .keys()
                                                    .map(|key| key
                                                                            .split(":")
                                                                            .into_iter()
                                                                            .map(|s| s.parse::<i32>().unwrap())
                                                                            .collect::<Vec<_>>()
                                                    )
                                                    .collect::<Vec<_>>();
    visited_lines.sort();
    println!("Visited lines: {:?}", visited_lines);

    let mut inside_tiles = 0;
    let mut direction = 0;

    for i in 0..visited_lines.len() - 1 {

        let _curr_tile = &visited_lines[i];
        let mut curr_tile_step = visited[&format!("{}:{}", visited_lines[i][0], visited_lines[i][1])];
        if curr_tile_step == 0 {
            curr_tile_step = max_steps + 1;
        }
        let below_tile_step = visited.get(&format!("{}:{}", visited_lines[i][0] + 1, visited_lines[i][1]));
        if let None = below_tile_step {
            continue;
        }
        let below_tile_step = below_tile_step.unwrap();

        let modulo = curr_tile_step - below_tile_step;

        if modulo.abs() == 1 {
            direction += modulo;
        }

        if direction != 0 {
            let tiles_to_add = visited_lines[i + 1][1] - visited_lines[i][1] - 1;
            inside_tiles += tiles_to_add;

            if tiles_to_add != 0 {
                println!("add {} tiles when step from {:?} to {:?}", tiles_to_add, visited_lines[i], visited_lines[i + 1]);
            }
        }
    }

    println!("Inside tiles: {}", inside_tiles);
}

fn main() {
    // part1("data/input.txt");
    part2("data/input.txt");
}

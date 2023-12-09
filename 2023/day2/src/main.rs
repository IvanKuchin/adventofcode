use std::{fs::File, io::BufReader, io::BufRead, vec};

struct Bag(i32, i32, i32);

#[derive(Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
struct Games {
    games: vec::Vec<Game>,
}

#[derive(Debug)]
struct GameRecord {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
    games: Games,
}

impl GameRecord {
    fn count_max(&mut self) {
        for game in &self.games.games {
            if self.red < game.red {
                self.red = game.red;
            }
            if self.green < game.green {
                self.green = game.green;
            }
            if self.blue < game.blue {
                self.blue = game.blue;
            }
        }
    }
}

impl std::str::FromStr for GameRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let id = parts.next().unwrap();
        let games = parts.next().unwrap();

        let id = id.replace("Game ", "").parse::<i32>().unwrap();
        let games = games.parse::<Games>().unwrap();
        
        // println!("id = {}\n games = {:?}", id, games);

        Ok(GameRecord { 
            id, 
            red: 0, 
            green: 0, 
            blue: 0, 
            games: games })
    }
}

fn parse_color(s: &str, game: String) -> Option<i32> {

    // println!("parse_color: s = {}, game = {}", s, game);

    let balls = game
                        .split(",")
                        .filter(|c| c.contains(s))
                        .next()?;

    let balls = balls.replace(s, "").trim().parse::<i32>().unwrap();
    Some(balls)
}

impl std::str::FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let green = parse_color("green", s.to_string()).unwrap_or(0);
        let red = parse_color("red", s.to_string()).unwrap_or(0);
        let blue = parse_color("blue", s.to_string()).unwrap_or(0);

        Ok(Game { red, green, blue })
    }
}

impl std::str::FromStr for Games {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v: vec::Vec<Game> = vec::Vec::new();
        let games = s.split(";");
        for game in games {
            // println!("- game {}", game);
            let game = game.parse::<Game>().unwrap();
            v.push(game);
        }

        Ok(Games { games: v })
    }
}

fn part1(bag: Bag, fname: String) {
    let file = File::open(fname).expect("can't open file");
    let reader = BufReader::new(file);

    let mut game_records: vec::Vec<GameRecord> = vec::Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();

        let mut game_record = l.parse::<GameRecord>().unwrap();
        game_record.count_max();

        game_records.push(game_record);
    }

    // for game_record in game_records {
    //     println!("id = {}, red = {}, green = {}, blue = {}", 
    //         game_record.id, game_record.red, game_record.green, game_record.blue);
    // }

    let game_ids = game_records
                        .iter()
                        .filter(|g| g.red <= bag.0 && g.green <= bag.1 && g.blue <= bag.2)
                        .map(|g| g.id)
                        .collect::<vec::Vec<i32>>();

    let sum = game_ids.iter().sum::<i32>();

    println!("sum = {}", sum);
}

fn part2(fname: String) {
    let file = File::open(fname).expect("can't open file");
    let reader = BufReader::new(file);

    let mut game_records: vec::Vec<GameRecord> = vec::Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();

        let mut game_record = l.parse::<GameRecord>().unwrap();
        game_record.count_max();

        game_records.push(game_record);
    }

    // for game_record in game_records {
    //     println!("id = {}, red = {}, green = {}, blue = {}", 
    //         game_record.id, game_record.red, game_record.green, game_record.blue);
    // }

    let game_powers = game_records
                        .iter()
                        .map(|g| g.red * g.green * g.blue)
                        .collect::<vec::Vec<i32>>();

    let sum = game_powers.iter().sum::<i32>();

    println!("sum = {}", sum);
}

fn main() {
    let  fname = "data/input.txt";

    part1(Bag(12, 13, 14), fname.to_string());
    part2(fname.to_string());
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

#[derive(Debug)]
struct Records {
    records: Vec<Race>,
}

impl std::str::FromStr for Records {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .trim()
            .split('\n')
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let times = lines[0]
                                .replace("Time:", "")
                                .replace(" ", "") // comment it for part 1
                                .trim()
                                .split_whitespace()
                                .map(|x| x.trim().parse::<usize>().unwrap())
                                .collect::<Vec<usize>>();
                            
        let distances = lines[1]
                                .replace("Distance:", "")
                                .replace(" ", "") // comment it for part 1
                                .trim()
                                .split_whitespace()
                                .map(|x| x.trim().parse::<usize>().unwrap())
                                .collect::<Vec<usize>>();

                            
        let  mut records = Vec::new();
        // join times and distances into Vec<Race>

        for i in 0..times.len() {
            records.push(Race {time: times[i], distance: distances[i]});
        }

        
        Ok(Records { records})
    }
}

impl Race {
    fn get_number_of_success_ways(&self) -> usize {
        let mut success = 0;
        let mut time = 0;

        while time < self.time {
            let distance = (self.time - time) * time;
            if distance > self.distance {
                success += 1;
            }
            time += 1;

            // if time % 10_000_000 == 0 {
            //     println!("time: {}M / {}M", time / 1_000_000, self.time / 1_000_000);
            // }

        }
        success
    }
}

fn part1(fname: &str) {
    let content = std::fs::read_to_string(fname).unwrap();
    let records = content.parse::<Records>().unwrap();

    println!("{:?}", records);

    let accumulator = records.records
                                    .iter()
                                    .fold(1, |acc, x| acc * x.get_number_of_success_ways());
    println!("Part 1: {}", accumulator);
}

fn main() {
    let fname = "data/input.txt";
    part1(fname);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_single_race() {
        let fname = "data/input.txt";
        let content = std::fs::read_to_string(fname).unwrap();
        let records = content.parse::<Records>().unwrap();

        assert_eq!(records.records[0].get_number_of_success_ways(), 4);
        assert_eq!(records.records[1].get_number_of_success_ways(), 8);
        assert_eq!(records.records[2].get_number_of_success_ways(), 9);
       }
}

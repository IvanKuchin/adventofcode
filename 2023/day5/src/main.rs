#[derive(Debug)]
struct Range {
    destination: usize,
    source: usize,
    length: usize,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl std::str::FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        let mut ranges = lines.iter()
                          .map(|line| {
                              let mut parts = line.split_whitespace();
                              let destination = parts.nth(0).unwrap().parse::<usize>().expect("can't parse destination");
                              let source = parts.nth(0).unwrap().parse::<usize>().expect("can't parse source");
                              let length = parts.nth(0).unwrap().parse::<usize>().expect("can't parse length");
                              Range { destination, source, length }
                          })
                          .collect::<Vec<Range>>();
        
        ranges.sort_by(|a, b| a.source.cmp(&b.source));
        
        Ok(Map { ranges })
    }
}

impl Map {
    fn get_destination(&self, source: usize) -> usize {
        for range in &self.ranges {
            if source < range.source {
                return source
            }
            if range.source <= source && source < range.source + range.length {
                return source + range.destination - range.source
            }
        }
        source
    }
}

impl std::str::FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps: Vec<Map> = Vec::new();
        let lines = s.lines().collect::<Vec<&str>>();
        let seeds = lines[0]
                                    .to_string()
                                    .replace("seeds: ", "")
                                    .trim()
                                    .split_whitespace()
                                    .map(|x| x.parse::<usize>().expect("can't parse seed"))
                                    .collect::<Vec<usize>>();

        // cretae vec<usize> from seeds containing line numbers that conains map-word
        let mut map_lines = lines.iter()
                             .enumerate()
                             .filter(|(_, line)| line.contains("map:"))
                             .map(|(i, _)| i)
                             .collect::<Vec<usize>>();
        map_lines.push(lines.len() );

        for i in 0..map_lines.len()-1 {
            let map = lines[map_lines[i]+1..map_lines[i+1]]
                            .join("\n")
                            .parse::<Map>()
                            .expect("can't parse map");

            maps.push(map);
        } 

        // let map = lines[map_lines[0]+1..map_lines[1]]
        //                     .join("\n")
        //                     .parse::<Map>()
        //                     .expect("can't parse map");

        // maps.push(map);

        Ok(Almanac { seeds, maps })
    }
}

impl Almanac {
    fn get_destination(&self, source: usize) -> usize {
        let mut destination = source;
        for map in &self.maps {
            destination = map.get_destination(destination);
            // print!("{} ", destination);
        }
        // println!("");
        destination
    }
}

fn part1(fname: &str) {
    let content = std::fs::read_to_string(fname).expect("can't read file");
    let mut almanac = content.parse::<Almanac>().expect("can't parse content");

    // println!("{}", almanac.get_destination(79));
    // println!("{}", almanac.maps[0].get_destination(100));
    // println!("{:#?}", almanac);

    let mut lowest_location = std::usize::MAX;
    for seed in &almanac.seeds {
        let curr_location = almanac.get_destination(*seed);
        if curr_location < lowest_location {
            lowest_location = curr_location;
            println!("seed {}, location {}", seed, lowest_location);
        }
    }
}

fn part2(fname: &str) {

    let content = std::fs::read_to_string(fname).expect("can't read file");
    let almanac = content.parse::<Almanac>().expect("can't parse content");

    let seed_pairs = almanac.seeds
                                            .chunks(2)
                                            .map(|chunk| (chunk[0], chunk[1]))
                                            .collect::<Vec<(usize, usize)>>();


    seed_pairs.iter().for_each(|&(seed, length)| {
            let mut lowest_location = std::usize::MAX;

            for i in seed..seed + length {
                let curr_location = almanac.get_destination(i);
                if curr_location < lowest_location {
                    lowest_location = curr_location;
                    println!("seed {}, location {}", i, lowest_location);
                }
            }
        }
    );

    // let mut lowest_location = std::usize::MAX;
    // for i in 0..almanac.seeds.len() / 2 {
    //     let mut step = 0;
    //     for j in almanac.seeds[i * 2]..almanac.seeds[i * 2] + almanac.seeds[i * 2 + 1] {
    //         let curr_location = almanac.get_destination(j);
    //         if curr_location < lowest_location {
    //             lowest_location = curr_location;
    //             println!("seed {}, location {}", j, lowest_location);
    //         }

    //         step += 1;
    //         if step % 100_000_000 == 0 {
    //             println!("{}M steps ", step/1_000_000);
    //         }
    //     }
    //     println!("{} range completed ({}M steps)", i, step/1_000_000);
    // }    
}

fn part3(fname: &str) {
    use rayon::prelude::*;

    let content = std::fs::read_to_string(fname).expect("can't read file");
    let almanac = content.parse::<Almanac>().expect("can't parse content");

    let seed_pairs = almanac.seeds
                                            .chunks(2)
                                            .map(|chunk| (chunk[0], chunk[1]))
                                            .collect::<Vec<(usize, usize)>>();


    seed_pairs.par_iter().for_each(|&(seed, length)| {
            let mut step = 0;
            let mut lowest_location = std::usize::MAX;

            for i in seed..seed + length {
                let curr_location = almanac.get_destination(i);
                if curr_location < lowest_location {
                    lowest_location = curr_location;
                    println!("seed {}, location {}", i, lowest_location);
                }
            }
        }
    );
}


fn main() {
    let fname = "data/input.txt";
    // part1(fname);
    // part2(fname);
    part3(fname);
}

mod test {
    #[test]
    fn part1_first_map() {
        let fname = "data/input.txt";
        let content = std::fs::read_to_string(fname).expect("can't read file");
        let almanac = content.parse::<super::Almanac>().expect("can't parse content");

        assert_eq!(almanac.maps[0].get_destination(97), 99);
        assert_eq!(almanac.maps[0].get_destination(98), 50);
        assert_eq!(almanac.maps[0].get_destination(99), 51);
        assert_eq!(almanac.maps[0].get_destination(100), 100);
    }

    #[test]
    fn part1_all_maps() {
        let fname = "data/input.txt";
        let content = std::fs::read_to_string(fname).expect("can't read file");
        let almanac = content.parse::<super::Almanac>().expect("can't parse content");

        assert_eq!(almanac.get_destination(79), 82);
        assert_eq!(almanac.get_destination(14), 43);
        assert_eq!(almanac.get_destination(55), 86);
        assert_eq!(almanac.get_destination(13), 35);
        assert_eq!(almanac.get_destination(82), 46);
    }
}
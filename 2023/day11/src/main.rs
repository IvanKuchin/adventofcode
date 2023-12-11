use std::cmp;
use std::collections::HashSet;
use itertools::Itertools;

fn part1(input: &str) {
    let content = std::fs::read_to_string(input).unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    let expand_rows = lines
                            .iter()
                            .enumerate()
                            .filter(|(_, line)| !line.contains("#"))
                            .map(|(i, _)| (i))
                            .collect::<HashSet<_>>();


    let expand_cols = (0..lines[0].len())
                                        .collect::<Vec<_>>()
                                        .iter()
                                        .filter(|col| {
                                            lines
                                                .iter()
                                                .map(|line| line.chars().nth(**col).unwrap() == '#')
                                                .all(|galaxy_here| !galaxy_here)
                                        })
                                        .copied()
                                        .collect::<HashSet<_>>();

    let mut galaxies = lines
                            .iter()
                            .enumerate()
                            .map(|(row, line)| { 
                                line
                                    .chars()
                                    .enumerate()
                                    .map(move |(col, galaxy)| {
                                        if galaxy == '#' {
                                            (row as i64, col as i64)
                                        } else {
                                            (-1, -1)
                                        }
                                    })
                            })
                            .flatten()
                            .collect::<HashSet<_>>();

    galaxies.remove(&(-1, -1));

    // println!("rows to expand {:?}", expand_rows);
    // println!("columns to expand {:?}", expand_cols);
    // println!("galaxies {:?}", galaxies);

    let galaxies_pairs = galaxies.iter().copied().combinations(2).unique().collect::<Vec<_>>();

    let mut sum = 0;
    for (idx, pair) in galaxies_pairs.iter().enumerate() {
        let (g1, g2) = (pair[0], pair[1]);
        let mut dist = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();

        let row_range = (cmp::min(g1.0, g2.0) as usize .. cmp::max(g1.0, g2.0) as usize).collect::<HashSet<_>>();
        let col_range = (cmp::min(g1.1, g2.1) as usize .. cmp::max(g1.1, g2.1) as usize).collect::<HashSet<_>>();

        let row_intersection = expand_rows.intersection(&row_range).collect::<Vec<_>>();
        let col_intersection = expand_cols.intersection(&col_range).collect::<Vec<_>>();

        // print!("g1 {:?} g2 {:?} : {} + {} + {} ", g1, g2, dist, row_intersection.len(), col_intersection.len());

        dist += (row_intersection.len() as i64 + col_intersection.len() as i64) * (1_000_000-1);

        // println!(" = {}", dist);
        
        sum += dist;
    }

    println!("sum {}", sum);
}
                            

fn main() {
    part1("data/input.prod");
}

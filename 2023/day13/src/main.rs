use std::cmp;

// make a function that counts number of diffeeent symbols between two strings
fn count_diff(s1: &str, s2: &str) -> usize {
    let mut count = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            count += 1;
        }
    }
 
    count
}

fn find_symmetry(matrix: &Vec<String>, skip: Option<usize>) -> usize {
    for cut in 1..matrix.len() {
        
        let cut_len = cmp::min(cut, matrix.len() - cut);
        let top = matrix[cut-cut_len..cut].join("|");
        let bottom = matrix[cut..cut + cut_len].iter().rev().map(|x| x.to_string()).collect::<Vec<_>>().join("|");

        if top == bottom {
            match skip {
                None => return cut,
                Some(skip) => if cut != skip { return cut; }
            }
        }
    }
    
    0
}


fn part1(fname: &str) {
    let content = std::fs::read_to_string(fname).unwrap();
    let mut sum: usize = 0;

    for block in content.split("\r\n\r\n") {
        if block.len() == 0 {
            continue;
        }

        let matrix = block
                                    .lines()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>();


        let transposed = (0..matrix[0].len())
                                        .collect::<Vec<_>>()
                                        .iter()
                                        .map(|&i| matrix
                                                                .iter()
                                                                .map(|row| {
                                                                    row.chars().nth(i).unwrap()
                                                                })
                                                                .collect::<String>())
                                        .collect::<Vec<_>>();

        let h_idx_original = find_symmetry(&matrix, None);
        let v_idx_original = find_symmetry(&transposed, None);

        sum += h_idx_original * 100 + v_idx_original;
    }

    println!("sum = {}", sum);

}

fn permute_matrix(matrix: &Vec<String>, h_idx_original: usize, v_idx_original: usize) -> (usize, usize) {

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            let new_symbol = if &matrix[y][x..x+1] == "#" { "." } else { "#" };
            let mut new_matrix = matrix.clone();
            new_matrix[y].replace_range(x..x+1, new_symbol);

            let transposed = (0..matrix[0].len())
                                                        .collect::<Vec<_>>()
                                                        .iter()
                                                        .map(|&i| new_matrix
                                                                                .iter()
                                                                                .map(|row| {
                                                                                    row.chars().nth(i).unwrap()
                                                                                })
                                                                                .collect::<String>())
                                                        .collect::<Vec<_>>();

            let h_idx = find_symmetry(&new_matrix, Some(h_idx_original));
            let v_idx = find_symmetry(&transposed, Some(v_idx_original));

            if h_idx > 0 || v_idx > 0 {
                return (h_idx, v_idx);
            }
        }
    }

    (0,0)
}

fn part2(fname: &str) {
    let content = std::fs::read_to_string(fname).unwrap();
    let mut sum: usize = 0;

    for block in content.split("\r\n\r\n") {
        if block.len() == 0 {
            continue;
        }

        let matrix = block
                                    .lines()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>();


        let transposed = (0..matrix[0].len())
                                        .collect::<Vec<_>>()
                                        .iter()
                                        .map(|&i| matrix
                                                                .iter()
                                                                .map(|row| {
                                                                    row.chars().nth(i).unwrap()
                                                                })
                                                                .collect::<String>())
                                        .collect::<Vec<_>>();

        let h_idx = find_symmetry(&matrix, None);
        let v_idx = find_symmetry(&transposed, None);

        let (h_idx_new, v_idx_new) = permute_matrix(&matrix, h_idx, v_idx);

        if h_idx_new == 0 && v_idx_new == 0 {
            println!("No solution found. Replace with {}", h_idx * 100 + v_idx);
        }

        sum += h_idx_new * 100 + v_idx_new;
    }
    
    println!("Sum = {}", sum);

}


fn main() {
    part1("data/prod");
    part2("data/prod");
}

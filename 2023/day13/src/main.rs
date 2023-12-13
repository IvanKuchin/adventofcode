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

fn find_symmetry(matrix: &Vec<String>, diff: usize) -> usize {
    for cut in 1..matrix.len() {
        
        let cut_len = cmp::min(cut, matrix.len() - cut);
        let top = matrix[cut-cut_len..cut].join("|");
        let bottom = matrix[cut..cut + cut_len].iter().rev().map(|x| x.to_string()).collect::<Vec<_>>().join("|");

        if count_diff(&top, &bottom) == diff {
            return cut;
        }
    }
    
    0
}


fn part_common(fname: &str, number_of_diff: usize) {
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

        let h_idx_original = find_symmetry(&matrix, number_of_diff);
        let v_idx_original = find_symmetry(&transposed, number_of_diff);

        sum += h_idx_original * 100 + v_idx_original;
    }

    println!("sum = {}", sum);

}


fn main() {
    part_common("data/prod", 0);
    part_common("data/prod", 1);
}

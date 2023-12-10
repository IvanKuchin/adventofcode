use std::{io::BufRead, fmt, vec};

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for c in row {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_lines_into_matrix(lines: std::str::Lines) -> Matrix {
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    for line in lines {
        cols = 0;
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            if c.is_digit(10) {
                row.push(c);
            } else if c == '*' {
                row.push('#');
            } else {
                row.push(' ');
            }
            cols += 1;
        }
        data.push(row);
        rows += 1;
    }
    Matrix { data, rows, cols }
}


impl Matrix {
    fn get_part_number_list(&self) -> Vec<i32> {
        let mut part_numbers: Vec<i32> = Vec::new();
        let mut mx_temp: Matrix = Matrix { data: self.data.clone(), rows: self.rows, cols: self.cols };
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row as usize][col as usize] == '#' {
                    // clone self to mx_temp
                    let mut gears: vec::Vec<i32> = Vec::new();

                    let pn = mx_temp.get_part_number(row - 1, col - 1);
                    if pn.is_some() { gears.push(pn.unwrap()) }
                    let pn = mx_temp.get_part_number(row - 1, col);
                    if pn.is_some() { gears.push(pn.unwrap()) }
                    let pn = mx_temp.get_part_number(row - 1, col + 1);
                    if pn.is_some() { gears.push(pn.unwrap()) }
                    let pn = mx_temp.get_part_number(row , col - 1);
                    if pn.is_some() { gears.push(pn.unwrap()) }
                    let pn = mx_temp.get_part_number(row , col + 1);
                    if pn.is_some() { gears.push(pn.unwrap()) }
                    let pn = mx_temp.get_part_number(row + 1, col - 1);
                    if pn.is_some() { gears.push(pn.unwrap()) }
                    let pn = mx_temp.get_part_number(row + 1, col);
                    if pn.is_some() { gears.push(pn.unwrap()) }
                    let pn = mx_temp.get_part_number(row + 1, col + 1);
                    if pn.is_some() { gears.push(pn.unwrap()) }

                    if(gears.len() == 2) {
                        let gear_ratio = gears.iter().product::<i32>();
                        println!("gear_ration: {}", gear_ratio);
                        part_numbers.push(gear_ratio);
                    }

                    // println!("{}", mx_temp);
                    // println!("----------------")
                }
            }
        }

        println!("{:?}", part_numbers);

        part_numbers
    }

    fn is_number_around(&self, row: usize, col: usize) -> bool {
        self.data[row][col].is_digit(10)
    }

    fn extract_number(&self, row: usize, col: usize) -> i32 {
        let mut start = col;
        loop {
            if start == 0 {
                break;
            }

            let curr_char = self.data[row][start-1];

            if curr_char.is_digit(10) {
                ()
            } else {
                break;
            }

            start -= 1;
        }

        let mut part_number = 0;
        for i in start..self.cols as usize {
            if self.data[row][i].is_digit(10) {
                part_number = part_number * 10 + self.data[row][i].to_digit(10).unwrap() as i32;
            } else {
                break;
            }
        }

        part_number
    }

    fn get_part_number(&mut self, row: i32, col: i32) -> Option<i32> {
        if row < 0 || col < 0 || row >= self.rows || col >= self.cols {
            return None;
        }

        let row = row as usize;
        let col = col as usize;

        if !self.is_number_around(row, col) {
            return None;
        }

        let part_number = self.extract_number(row, col);
        if part_number == 0 {
            return None;
        }

        self.remove_number(row, col);
        Some(part_number)
    }

    fn remove_number(&mut self, row: usize, col: usize) -> i32 {
        let mut start = col;
        loop {
            if start == 0 {
                break;
            }

            let curr_char = self.data[row][start-1];

            if curr_char.is_digit(10) {
                ()
            } else {
                break;
            }

            start -= 1;
        }

        let mut part_number = 0;
        for i in start..self.cols as usize {
            if self.data[row][i].is_digit(10) {
                self.data[row][i] = '-';
            } else {
                break;
            }
        }

        part_number
    }

}


fn part1(fname: &str) {
    // let mut reader = std::io::BufReader::new(std::fs::File::open(fname).unwrap());
    // let mut line = String::new();
    // reader.read_line(&mut line).unwrap();

    let f_content = std::fs::read_to_string(fname)
                                .unwrap();
    let lines = f_content.lines();

    let mx = read_lines_into_matrix(lines);

    let part_numbers = mx.get_part_number_list(); 

    // println!("{}", mx);

    let sum = part_numbers.iter().sum::<i32>();

    print!("Sum: {}", sum);
}

fn main() {
    let fname = "data/input.txt";
    part1(fname);
}

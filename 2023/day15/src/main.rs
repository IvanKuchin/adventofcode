use std::collections::VecDeque;

fn hash(val: &str) -> usize {
    let chunk = val.as_bytes();
    let mut curr = 0;

    for i in 0..chunk.len() {
        curr += chunk[i] as usize;
        curr *= 17;
        curr %= 256;
    }

    curr
}

fn part1(fname: &str) -> usize {
    let content = std::fs::read_to_string(fname).unwrap();
    let chunks = content.split(",").collect::<Vec::<_>>();

    let mut sum = 0;
    for chunk in chunks {
        sum += hash(chunk);
    }

    sum
}

#[derive(Clone, Debug)]
struct Lens(String, usize);

#[derive(Clone)]
struct OpticalBox {
    lenses: VecDeque<Lens>,
}

fn add_lens(lenses: &mut VecDeque<Lens>, new_lens: Lens) {
    // let mut new_lenses = VecDeque::new();
    let mut added = false;

    for lens_id in 0..lenses.len() {
        if lenses[lens_id].0 == new_lens.0 {
            lenses[lens_id].1 = new_lens.1;
            added = true;
        }
    }
    if !added {
        lenses.push_back(new_lens.clone());
    }
}

fn remove_lens(lenses: &VecDeque<Lens>, id: &str) -> VecDeque<Lens> {
    let mut new_lenses = VecDeque::new();

    for lens in lenses {
        if lens.0 != id {
            new_lenses.push_back(lens.clone());
        }
    }

    new_lenses
}

fn part2(fname: &str) -> usize {
    let content = std::fs::read_to_string(fname).unwrap();
    let chunks = content.split(",").collect::<Vec::<_>>();

    let mut boxes: Vec<OpticalBox> = Vec::new();

    for _ in 0..256 {
        boxes.push( OpticalBox { lenses: VecDeque::new() });
    }

    for chunk in chunks {
        // println!("chunk {:?}", chunk);
        if chunk.contains("=") {
            let parts = chunk.split("=").collect::<Vec::<_>>();
            let (box_hash, focal) = (parts[0], parts[1].parse::<usize>().unwrap());
            let box_id = hash(box_hash);

            add_lens(&mut boxes[box_id].lenses, Lens(box_hash.to_string(), focal));
            
            // println!("add to box #{:?} -> {:?}", box_id, boxes[box_id].lenses);
        }
        if chunk.contains("-") {
            let box_hash = &chunk[0..chunk.len() - 1];
            let box_id = hash(box_hash);

            boxes[box_id].lenses = remove_lens(&boxes[box_id].lenses, box_hash);

            // println!("remove from box #{:?} -> {:?}", box_id, boxes[box_id].lenses);
        }

        // println!("");
    }

    let mut sum = 0;
    for box_id in 0..256 {
        for (lens_id, lens) in boxes[box_id].lenses.iter().enumerate() {
            sum += (box_id + 1) * (lens_id + 1) * lens.1;
        }
    }
    sum
}

fn main() {
    println!("Part1 {}", part1("data/prod"));
    println!("Part2 {}", part2("data/prod"));

    let arr: [String; 5] = [String::new(); 5];

}

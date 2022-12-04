use std::fs;

fn get_priority(char: char) -> u8 {
    match char {
        'a'..='z' => char as u8 - b'a' + 1,
        'A'..='Z' => char as u8 - b'A' + 27,
        _ => 0,
    }
}

fn part_one(input: String) {
    let rucksacks = input.split('\n');
    let mut sum_of_priorities: i32 = 0;

    'outer: for rucksack in rucksacks {
        let size = rucksack.len();
        let (first_compartment, second_compartment) = rucksack.split_at(size / 2);
        for c in first_compartment.chars() {
            if second_compartment.contains(c) {
                sum_of_priorities += get_priority(c) as i32;
                continue 'outer;
            }
        }
    }

    println!("Sum of priorities: {:?}", sum_of_priorities);
}

fn part_two(input: String) {
    let rucksacks = input.split('\n');
    let groups = rucksacks.step_by(3);

    for group in groups {
        println!("{:?}", group);
    }
}

fn main() {
    let file_path = "src/input.txt";
    let input = fs::read_to_string(file_path).expect("File not found");

    part_one(input.clone());
    part_two(input);
}

use std::fs;

#[derive(Copy, Clone)]
struct Assignment {
    low: i32,
    high: i32,
}

fn is_contained(assignment1: Assignment, assignment2: Assignment) -> bool {
        let mut struct_is_contained: bool = false;

        if assignment1.low <= assignment2.low && assignment1.high >= assignment2.high {
            struct_is_contained = true;
        }
        if assignment2.low <= assignment1.low && assignment2.high >= assignment1.high {
            struct_is_contained = true;
        }

        struct_is_contained
}

fn does_overlap(assignment1: Assignment, assignment2: Assignment) -> bool {
    let mut overlap: bool = false;

    if assignment1.low <= assignment2.low && assignment1.high >= assignment2.low {
        overlap = true;
    }
    if assignment2.low <= assignment1.low && assignment2.high >= assignment1.low {
        overlap = true;
    }

    overlap
}

fn main() {
    let file_path = "src/input.txt";
    let input = fs::read_to_string(file_path).expect("File not found");
    let pairs = input.lines();

    let mut fully_contained: i32 = 0;
    let mut overlapping: i32 = 0;

    for pair in pairs {
        let mut assignment_vec: Vec<Assignment> = Vec::new();
        let assignments = pair.split(',');
        for assignment_str in assignments {
            let high_low: Vec<&str> = assignment_str.split("-").collect();
            assignment_vec.push(Assignment { low: high_low[0].parse().unwrap(), high: high_low[1].parse().unwrap() });
        }

        if is_contained(assignment_vec[0], assignment_vec[1]) {
            fully_contained += 1;
        }
        if does_overlap(assignment_vec[0], assignment_vec[1]) {
            overlapping += 1;
        }

    }

    println!("Fully contained: {fully_contained}");
    println!("Overlapping: {overlapping}");
}

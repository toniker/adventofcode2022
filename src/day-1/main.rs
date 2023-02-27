use std::fs;

fn main() {
    let file_path = "src/input.txt";

    let input = fs::read_to_string(file_path).expect("File not found");

    let elves = input.split("\n\n");

    let mut elf_calories: Vec<i32> = Vec::new();

    for elf in elves {
        let mut count: i32 = 0;
        let items = elf.split('\n');
        for item in items {
            count += match item.parse::<i32>() {
                Ok(n) => { n }
                Err(_) => { 0 }
            }
        }
        elf_calories.push(count);
    }

    elf_calories.sort();
    let len: usize = elf_calories.len();

    println!("Elf with most calories: {:?}", elf_calories[len - 1]);

    let sum_of_top_3: i32 = elf_calories[len - 1] + elf_calories[len - 2] + elf_calories[len - 3];

    println!("Sum of top 3 elves with most calories: {:?}", sum_of_top_3);
}
fn main() {

    // Read input from file and put it into a Vector
    let calories_input: Vec<&str> = include_str!("../input.txt").split("\n").collect();
    
    let mut single_elf_calories: u32 = 0;
    let mut calorie_per_elf: Vec<u32> = Vec::new();

    // Add calories for each elves
    for calorie in calories_input.iter() {
        if calorie.is_empty() || *calorie == calories_input[calories_input.len()-1] {
            calorie_per_elf.push(single_elf_calories);
            single_elf_calories = 0;
            continue;
        }
        single_elf_calories = single_elf_calories + calorie.parse::<u32>().unwrap();
    }

    // Sort ascending then revert to get highest calorie first
    calorie_per_elf.sort();
    calorie_per_elf.reverse();

    println!("{}", calorie_per_elf[0]);
    
}
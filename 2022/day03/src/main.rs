use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ALPHABET: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

fn calcul_priority (mut character: char) -> usize {

    let mut additive: usize = 1;

    match character.is_lowercase() {
        true => (),
        false => {
            additive = 1 + 26;
            character.make_ascii_lowercase();
        }
    }

    return ALPHABET.iter().position(|&r| r == character).unwrap() + additive
}

fn find_chars(first_slice: &[char], second_slice: &[char]) -> Vec<char> {

    let mut chars: Vec<char> = Vec::new();

    for element in first_slice {
        match second_slice.iter().position(|&r| r == *element) {
            Some(i) => {
                chars.push(second_slice.get(i).unwrap().clone());
            },
            None => continue
        }
    }
    chars.sort();
    chars.dedup();
    return chars;
}

fn main() {

    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut sum_priorities_p1: usize = 0;
    let mut sum_priorities_p2: usize = 0;
    let mut rucksack_group: Vec<Vec<char>> = Vec::new();

    for line in f.lines() {
        // Parse the line and store characters in a Vector
        let line = line.expect("Unable to read line");
        let rucksack: Vec<char> = line.chars().collect();

        // [PART-1]

        // Split rucksacks into two compartment
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len()/2);

        // Get shared char between compartments
        let shared_items: Vec<char> = find_chars(first_compartment, second_compartment);
        // Ensure a single item is found otherwise panic
        assert!(shared_items.len() == 1);
        // Calculate items priority
        sum_priorities_p1 = sum_priorities_p1 + calcul_priority(shared_items[0]);

        // [PART-2]

        // Add current rucksack to the group
        rucksack_group.push(rucksack);
        
        // If the group contain 3 rucksack then find badge
        if rucksack_group.len() == 3 {

            let shared_items_r1r2: Vec<char> = find_chars(rucksack_group[0].as_slice(), rucksack_group[1].as_slice());
            let shared_items_r2r3: Vec<char> = find_chars(rucksack_group[1].as_slice(), rucksack_group[2].as_slice());
            let badge: Vec<char> = find_chars(shared_items_r1r2.as_slice(), shared_items_r2r3.as_slice());

            // Ensure a single badge is found otherwise panic
            assert!(badge.len() == 1);

            // Calculate badges priority
            sum_priorities_p2 = sum_priorities_p2 + calcul_priority(badge[0]);

            // Empty vector for next group or rucksack
            rucksack_group.clear();
        }
    }

    println!("[PART-1] Sum of items priority: {}\n[PART-2] Sum of badges priority: {}", sum_priorities_p1, sum_priorities_p2);
}

use std::{collections::HashSet, fs, hash::Hash};

fn main() {
    let input = read_input();
    let mut list1 : Vec<i32> = Vec::new();
    let mut list2 : Vec<i32> = Vec::new();

    for line in input.split("\n") {
        let mut split = line.split("   ");
        list1.push(split.next().unwrap().parse::<i32>().unwrap());
        list2.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    // Part 1
    let mut sum = 0;

    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }

    println!("Sum of differences: {}", sum);

    // Part 2
    let mut similarity_score = 0;
    for item in list1.iter() {
        similarity_score += list2.iter().filter(|x| *x == item).count() as i32 * item;
    }

    println!("Similarity score: {}", similarity_score);
}

fn read_input() -> String {
    return fs::read_to_string("input").expect("Failed to read file");
}
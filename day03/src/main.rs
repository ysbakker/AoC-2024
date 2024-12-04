use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap().replace("\n", "");
    let sum = get_sum_of_multiples(&input);
    let mut sum_disabled = sum;

    let disabled_pattern = Regex::new(r"don't\(\)(.*?)(?:do\(\)|$)").unwrap();
    for (_, [disabled_part]) in disabled_pattern.captures_iter(&input).map(|x| x.extract()) {
        sum_disabled -= get_sum_of_multiples(disabled_part);
    }

    println!("Sum: {}", sum);
    println!("Sum without disabled parts: {}", sum_disabled);
}

fn get_sum_of_multiples(instructions: &str) -> u32 {
    let mut sum: u32 = 0;
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for (_, [v1, v2]) in mul_pattern.captures_iter(&instructions).map(|x| x.extract()) {
        let i1 = v1.parse::<u32>().unwrap();
        let i2 = v2.parse::<u32>().unwrap();
        sum += i1 * i2;
    }

    sum
}

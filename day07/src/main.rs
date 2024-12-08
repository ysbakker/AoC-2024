use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut sum_of_test_values = 0;

    for equation in input.lines() {
        let mut split = equation.split(':');
        let test_value: usize = split.next().unwrap().parse().unwrap();
        let equation: Vec<usize> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let amount_of_values = equation.len() as u32;
        let permutations = (2 as i32).pow(amount_of_values - 1);
        let mut result = equation[0] as usize;
        let mut valid = false;

        for p in 0..permutations {
            for n in 0..amount_of_values - 1 {
                // Get nth bit
                let b = (p >> n) & 1;
                if b == 1 {
                    result *= equation[(n as usize) + 1];
                } else {
                    result += equation[(n as usize) + 1];
                }
            }

            if result == test_value {
                valid = true;
                break;
            }
            result = equation[0];
        }
        if valid {
            sum_of_test_values += test_value;
        }
    }

    println!("Sum of test values: {}", sum_of_test_values);
}

// 000
// 001
// 010
// 011
// 100
// 101
// 110
// 111

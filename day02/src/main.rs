use std::fs;

enum Motion {
    Decreasing,
    Increasing,
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let reports = input.split("\n");

    let mut safe_reports: u8 = 0;
    let mut dampened_safe_reports: u16 = 0;

    for report in reports {
        let levels = report
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        if is_safe(&levels) {
            safe_reports += 1;
            dampened_safe_reports += 1;
        } else {
            for i in 0..levels.len() {
                let mut dampened_levels = levels.clone();
                dampened_levels.remove(i);
                if is_safe(&dampened_levels) {
                    dampened_safe_reports += 1;
                    break;
                }
            }
        }
    }

    println!("Safe reports: {}", safe_reports);
    println!("Dampened safe reports: {}", dampened_safe_reports);
}

fn is_safe(levels: &Vec<u8>) -> bool {
    let mut prev: Option<u8> = None;
    let mut motion: Option<Motion> = None;

    for level in levels {
        let l = *level;
        if prev.is_none() {
            prev = Some(l);
            continue;
        }

        if motion.as_ref().is_none() {
            if prev.unwrap() < l {
                motion = Some(Motion::Increasing);
            } else {
                motion = Some(Motion::Decreasing);
            }
        }

        if (matches!(motion.as_ref().unwrap(), Motion::Increasing) && prev.unwrap() > l)
            || (matches!(motion.as_ref().unwrap(), Motion::Decreasing) && prev.unwrap() < l)
            || !check_diff(prev.unwrap(), l)
        {
            return false;
        }

        prev = Some(l);
    }

    return true;
}

fn check_diff(val1: u8, val2: u8) -> bool {
    return diff(val1, val2) >= 1 && diff(val1, val2) <= 3;
}

fn diff(val1: u8, val2: u8) -> u8 {
    if val1 > val2 {
        val1 - val2
    } else {
        val2 - val1
    }
}

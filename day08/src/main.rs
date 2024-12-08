use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let antennas = parse_antennas(&input);
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let mut antinodes_with_harmonics: HashSet<(isize, isize)> = HashSet::new();

    for positions in antennas.values() {
        for (x1, y1) in positions {
            for (x2, y2) in positions {
                let dx = x2 - x1;
                let dy = y2 - y1;

                if dx == 0 && dy == 0 {
                    continue;
                }

                let mut antinode = (x1 - dx, y1 - dy);

                if is_within_bounds(&input, antinode) {
                    antinodes.insert(antinode);
                }

                antinodes_with_harmonics.insert((*x1, *y1));
                let mut iters = 1;
                while is_within_bounds(&input, antinode) {
                    antinodes_with_harmonics.insert(antinode);
                    iters += 1;
                    antinode = (x1 - (dx * iters), y1 - (dy * iters));
                }
            }
        }
    }

    println!("Amount of antinodes: {}", antinodes.len());
    println!(
        "Amount of antinodes with resonant harmonics: {}",
        antinodes_with_harmonics.len()
    );
}

fn parse_antennas(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            match antennas.get_mut(&c) {
                Some(antenna) => {
                    antenna.push((x.try_into().unwrap(), y.try_into().unwrap()));
                }
                None => {
                    antennas.insert(c, vec![(x.try_into().unwrap(), y.try_into().unwrap())]);
                }
            }
        }
    }

    return antennas;
}

fn is_within_bounds(input: &str, (x, y): (isize, isize)) -> bool {
    let lines = input.lines().count();
    let chars = input.lines().next().unwrap().len();
    return x >= 0 && y >= 0 && y < lines.try_into().unwrap() && x < chars.try_into().unwrap();
}

/*
- Get all unique frequencies
- Put them in a dictionary
- Iterate over the antennas
- For each antenna, find the distance (dx, dy) to each other antenna of the same frequency
- Place the antinode, at (-dx, -dy) from the origin in a hashset
- Count the hashset entries

Part 2:
- Include antenna origins
- Loop over (dx * i, dy * i) until out of bounds and add all as antinodes to a hashset
- Count the hashset entries
 */

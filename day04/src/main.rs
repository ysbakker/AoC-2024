use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut times_xmas = 0;
    let mut times_x_mas = 0;
    let dir: Vec<(i16, i16)> = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let corners: Vec<(i16, i16)> = vec![(-1, -1), (1, -1), (1, 1), (-1, 1)];

    // Part 1
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != 'X' {
                continue;
            }

            for (dx, dy) in dir.iter() {
                let mut cx = x as i16;
                let mut cy = y as i16;
                let mut i = 1;
                let word = "XMAS";

                loop {
                    if i > 3 {
                        times_xmas += 1;
                        break;
                    }

                    cx = cx + dx;
                    cy = cy + dy;

                    if cx >= lines[0].len() as i16 || cy >= lines.len() as i16 || cx < 0 || cy < 0 {
                        break;
                    }

                    if lines[cy as usize].chars().nth(cx as usize).unwrap()
                        != word.chars().nth(i).unwrap()
                    {
                        break;
                    }

                    i += 1;
                }
            }
        }
    }

    // Part 2
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != 'A' {
                continue;
            }
            let mut corner_letters = String::new();

            for (dx, dy) in corners.iter() {
                let mut cx = x as i16;
                let mut cy = y as i16;

                cx = cx + dx;
                cy = cy + dy;

                if cx >= lines[0].len() as i16 || cy >= lines.len() as i16 || cx < 0 || cy < 0 {
                    break;
                }
                corner_letters.push(lines[cy as usize].chars().nth(cx as usize).unwrap());
            }

            if corner_letters == "MSSM"
                || corner_letters == "SSMM"
                || corner_letters == "SMMS"
                || corner_letters == "MMSS"
            {
                times_x_mas += 1;
            }
        }
    }

    println!("XMAS: {} times", times_xmas);
    println!("X-MAS: {} times", times_x_mas);
}

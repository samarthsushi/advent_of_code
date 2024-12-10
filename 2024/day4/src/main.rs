use std::fs;

fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_xmas(grid: &[Vec<char>]) -> usize {
    let directions = [
        ((-1, -1), (1, 1)), 
        ((-1, 1), (1, -1)),
    ];

    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] == 'A' {
                let mut valid = true;

                for &((dx1, dy1), (dx2, dy2)) in &directions {
                    let mut has_m = false;
                    let mut has_s = false;

                    for &(dx, dy) in &[(dx1, dy1), (dx2, dy2)] {
                        let x = i + dx;
                        let y = j + dy;

                        if x >= 0 && x < rows && y >= 0 && y < cols {
                            let c = grid[x as usize][y as usize];
                            if c == 'M' {
                                has_m = true;
                            } else if c == 'S' {
                                has_s = true;
                            }
                        }
                    }

                    if !(has_m && has_s) {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let file_path = "data/data.txt";
    let s = fs::read_to_string(file_path).unwrap();
    let g = string_to_grid(&s);
    let x = find_xmas(&g);
    println!("{x}");
}



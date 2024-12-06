use std::fs;
use std::collections::HashMap;

fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_xmas(grid: &[Vec<char>]) -> usize {
    let directions = [
        (0, 1),   
        (0, -1),  
        (1, 0),
        (-1, 0),  
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut match_found = true;

                for k in 0..4 {
                    let x = i as isize + k * dx;
                    let y = j as isize + k * dy;

                    if x < 0 || x >= rows as isize || y < 0 || y >= cols as isize {
                        match_found = false;
                        break;
                    }

                    if grid[x as usize][y as usize] != XMAS[k as usize] {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
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



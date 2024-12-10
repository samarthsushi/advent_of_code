use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "data/data.txt";
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut safe_count = 0;
    for line in reader.lines() {
        let line = line?;
        let parts = line.split_whitespace();
        
        let seq = parts.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let increasing = seq[0] < seq[1];
        let mut safe = true;
        let mut dampener_used = false;

        for pair in seq.windows(2) {
            let diff = (pair[0] - pair[1]).abs();
            let condition_violated = if increasing {
                pair[0] > pair[1]
            } else {
                pair[0] < pair[1]
            };

            if condition_violated || !(1..=3).contains(&diff) {
                if dampener_used {
                    safe = false;
                    break;
                } else {
                    dampener_used = true;
                }
            }
        }

        if safe {
            safe_count += 1;
        }
    }
    println!("{safe_count}");
    Ok(())    
}
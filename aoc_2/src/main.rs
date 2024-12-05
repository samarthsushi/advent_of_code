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
        
        let partx = parts.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let increasing = partx[0] < partx[1];
        let mut safe = true;
        let mut dampener_limit = false;

        for i in 0..partx.len() - 1 {
            if increasing { 
                if partx[i] > partx[i+1] {
                    if !dampener_limit {
                        dampener_limit = true;
                        continue;
                    }
                    safe = false;
                    break; 
                } 
            }
            else { 
                if partx[i] < partx[i+1] { 
                    if !dampener_limit {
                        dampener_limit = true;
                        continue;
                    }
                    safe = false;
                    break; 
                } 
            }
            let diff = (partx[i] - partx[i+1]).abs();
            if diff < 1 || diff > 3 {
                if !dampener_limit {
                    dampener_limit = true;
                    continue;
                }
                safe = false;
                break;
            }
        }

        if safe {
            safe_count += 1;
        }
    }
    println!("{safe_count}");
    Ok(())    
}
use std::fs;
use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/data.txt";
    let hay = fs::read_to_string(file_path)?;

    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)");
    for caps in re?.captures_iter(&hay) {
        let num1: i32 = caps[1].parse().unwrap();
        let num2: i32 = caps[2].parse().unwrap();
        
        sum += num1 * num2;
    }

    println!("{sum}");

    Ok(())
}

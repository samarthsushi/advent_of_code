use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn sum_distances(a: &[i32], b: &[i32]) -> i32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn build_occurrence_map(vec: &Vec<i32>) -> HashMap<i32, u32> {
    let mut map = HashMap::new();

    for &value in vec {
        *map.entry(value).or_insert(0) += 1;
    }

    map
}

fn similarity_score(a: &HashMap<i32, u32>, b: &HashMap<i32, u32>) -> u32 {
    let mut similarity_score = 0;

    for k in a.keys() {
        similarity_score += b.get(k).unwrap_or(&0) * *k as u32;
    }

    similarity_score
}

fn main() -> io::Result<()>{
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    let file_path = "data/data.txt";
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {

            if let (Ok(val_a), Ok(val_b)) = (first.parse::<i32>(), second.parse::<i32>()) {
                a.push(val_a);
                b.push(val_b);
            } else {}
        } else {}
    }
    
    a.sort();
    b.sort();
    let sum = sum_distances(&a,&b);
    println!("{sum}");

    let a_map = build_occurrence_map(&a);
    let b_map = build_occurrence_map(&b);

    let sscor = similarity_score(&a_map, &b_map);
    println!("{}", sscor);
    
    Ok(())
}

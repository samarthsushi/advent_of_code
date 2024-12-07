use std::fs::{self, File};
use std::io::{self, BufRead, Read};
use std::collections::HashMap;

fn main() {
    let file_path = "data/data.txt";
    let file = File::open(file_path).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut s1 = String::new();
    let mut s2 = String::new();

    for line in reader.by_ref().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        s1.push_str(&line);
        s1.push('\n');
    }
    reader.read_to_string(&mut s2).unwrap();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in s1.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let a: String = parts[0].to_string();
        let b: String = parts[1].to_string();

        map.entry(a).or_insert_with(Vec::new).push(b);
    }
    let mut sum = 0;
    for line in s2.lines() {
        let mut is_valid = true;
        let pages: Vec<String> = line
            .split(',')
            .map(|s| s.to_string())
            .collect();
        for i in 0..pages.len()-1 {
            for j in i + 1..pages.len() {
                if let Some(values) = map.get(&pages[i]) {
                    if !values.contains(&pages[j]) {
                        is_valid = false;
                        break;
                    }
                } else {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
        }  
        if !is_valid {
            continue;
        }
        let middle_index = pages.len()/2;
        let middle = pages[middle_index].parse::<u32>().unwrap();
        sum+=middle;
    }
    println!("{}", sum);
}
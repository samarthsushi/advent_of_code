use regex::Regex;
use std::collections::HashMap;

fn count_ways<'a>(design: &'a str, towel_patterns: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = cache.get(design) {
        return result;
    }

    let mut count = 0;
    for &pattern in towel_patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            count += count_ways(remaining, towel_patterns, cache);
        }
    }

    cache.insert(design, count);

    count
}

fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();

    let (a,b) = s.split_once("\r\n\r\n").unwrap();
    let patterns = a.split(',').map(|p| p.trim()).collect::<Vec<&str>>();
    let strings = b.lines().collect::<Vec<&str>>();

    let re_raw = format!("^({})*$", patterns.join("|"));
    let re = Regex::new(&re_raw).unwrap();

    let p1 = strings
        .iter()
        .filter(|design| re.is_match(design))
        .count();

    println!("{p1}");

    let mut p2 = 0;
    for string in strings {
        let mut cache = HashMap::new();
        p2+=count_ways(string, &patterns, &mut cache);
    }

    println!("{p2}");
}

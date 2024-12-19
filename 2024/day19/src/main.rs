use regex::Regex;

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
}

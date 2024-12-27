fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();
    let mut prices = Vec::new();
    for l in s.as_str().lines() {
        prices.push(l.parse::<usize>().unwrap());
    }

    for _ in 0..2000 {
        for x in &mut prices {
            let mut dx = *x << 6;
            *x ^= dx;
            *x %= 16777216;

            dx = *x >> 5;
            *x ^= dx;
            *x %= 16777216;

            dx = *x << 11;
            *x ^= dx;
            *x %= 16777216;
        }
    }

    let p1: usize = prices.iter().sum();
    println!("{p1}");    
}

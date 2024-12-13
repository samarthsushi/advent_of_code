use std::collections::HashMap;

fn get_num_digits(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![0];
    }

    let mut digits: Vec<u64> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    digits
}

fn merge_digits(digits: &[u64]) -> u64 {
    let result: u64 = digits
        .iter()
        .enumerate()
        .rev()
        .map(|(k, v)| {
            let d = 10_u64.pow((digits.len() - k - 1) as u32);
            d * v
        })
        .sum();
    result
}

fn split_digits(digits: Vec<u64>) -> (u64, u64) {
    let (left, right) = digits.split_at(digits.len() / 2);
    (merge_digits(left), merge_digits(right))
}

fn blink_util(num: u64, blinks: u64) -> Vec<u64> {
    let mut digits: Vec<u64> = vec![num];
    for _ in 0..blinks {
        let mut current: Vec<u64> = Vec::new();
        for num in digits.iter() {
            if *num == 0 {
                current.push(1);
            } else {
                let digits = get_num_digits(*num);
                if digits.len() % 2 == 0 {
                    let (left, right) = split_digits(digits);
                    current.push(left);
                    current.push(right);
                } else {
                    current.push(*num * 2024);
                }
            }
        }
        digits = current;
    }
    digits
}

fn blinking(num: u64, blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(entry) = cache.get(&(num, blinks)) {
        return *entry as u64;
    }

    let mut total: u64 = 0;
    let stones = blink_util(num, 1);
    for v in stones.iter() {
        total += blinking(*v, blinks - 1, cache);
    }

    cache.insert((num, blinks), total as u64);
    total
}

fn solver(data: &str, blinks: u64) -> u64 {
    let stones = data.split_whitespace().map(|x| x.parse::<u64>().expect("NaN")).collect::<Vec<u64>>();
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    let mut total: u64 = 0;
    for num in stones.iter() {
        total += blinking(*num, blinks, &mut cache);
    }
    total
}

fn main() {
    let file_path = "data/data.txt";
    let s = std::fs::read_to_string(file_path).unwrap();

    let p1 = solver(&s, 25);
    let p2 = solver(&s, 75);
    println!("{p1}\n{p2}");
}

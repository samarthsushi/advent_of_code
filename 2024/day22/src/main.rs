use std::collections::{HashSet, HashMap};

fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();
    let mut secrets = Vec::new();
    for l in s.as_str().lines() {
        secrets.push(l.parse::<usize>().unwrap());
    }

    let secrets_with_t: Vec<Vec<usize>> = secrets
        .iter()
        .map(|&initial| generate_secrets(initial, 2000))
        .collect();
    let p1: usize = secrets_with_t
        .iter()
        .map(|secrets| *secrets.last().unwrap())
        .sum();
    println!("{p1}");
    let prices_with_t: Vec<Vec<isize>> = (0..2000)
        .map(|t| secrets_with_t.iter().map(|secrets| (secrets[t] % 10) as isize).collect())
        .collect();
    let dprices_dt: Vec<Vec<isize>> = prices_with_t
        .windows(2)
        .map(|w| {
            w[1].iter()
                .zip(w[0].iter())
                .map(|(curr, prev)| curr - prev)
                .collect()
        })
        .collect();

    let (best_sequence, max_bananas) = best_window(&dprices_dt, &prices_with_t);

    println!("best sequence: {:?}\nmax bananas: {}", best_sequence, max_bananas);
}

fn generate_secrets(initial: usize, count: usize) -> Vec<usize> {
    let mut secrets = Vec::new();
    secrets.push(initial);
    for _ in 0..count {
        let mut x = *secrets.last().unwrap();
        let mut dx = x << 6;
        x ^= dx;
        x %= 16777216;

        dx = x >> 5;
        x ^= dx;
        x %= 16777216;

        dx = x << 11;
        x ^= dx;
        x %= 16777216;

        secrets.push(x);
    }
    secrets
}

fn best_window(
    dprices_dt: &[Vec<isize>],
    prices_with_t: &[Vec<isize>],
) -> (Vec<isize>, isize) {
    let mut sequence_bananas: HashMap<Vec<isize>, isize> = HashMap::new();

    for vendor in 0..dprices_dt[0].len() {
        let mut seen_sequences: HashSet<Vec<isize>> = HashSet::new();
        for (t, window) in dprices_dt.windows(4).enumerate() {
            let seq: Vec<isize> = window.iter().map(|changes| changes[vendor]).collect();
            if !seen_sequences.insert(seq.clone()) {
                continue;
            }
            let bananas = prices_with_t[t + 4][vendor];
            *sequence_bananas.entry(seq).or_insert(0) += bananas;
        }
    }

    let mut max_bananas = 0;
    let mut best_sequence = Vec::new();
    for (seq, bananas) in sequence_bananas {
        if bananas > max_bananas {
            max_bananas = bananas;
            best_sequence = seq;
        }
    }
    
    (best_sequence, max_bananas)
}
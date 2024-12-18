fn get_combo(oprnd: i64, registers: &[i64]) -> i64 {
    match oprnd {
        0..=3 => oprnd,
        4     => registers[0],
        5     => registers[1],
        6     => registers[2],
        _     => unreachable!()
    }
}

fn run(program: &[i64], a: i64) -> Vec<i64> {
    let mut registers = [a, 0, 0];
    let mut output = Vec::new();
    let mut ptr = 0;

    while ptr < program.len() {
        let oprnd = program[ptr+1];
        match program[ptr] {
            0 => registers[0] >>= get_combo(oprnd, &registers) as u32,
            1 => registers[1] ^= oprnd,
            2 => registers[1] = get_combo(oprnd, &registers) & 7,
            3 => {
                if registers[0] != 0 {
                    ptr = oprnd.try_into().unwrap();
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => {
                let o = get_combo(oprnd, &registers) & 7;
                output.push(o);
            }
            6 => registers[1] = registers[0] >> get_combo(oprnd, &registers) as u32,
            7 => registers[2] = registers[0] >> get_combo(oprnd, &registers) as u32,
            _ => unreachable!(),
        }
        ptr+=2;
    }
    output
}

fn find_a(prog: &[i64], target: &[i64], mut a: i64, depth: usize) -> Option<i64> {
    if depth == target.len() {
        return Some(a);
    }

    for i in 0..8 {
        let candidate_a = a * 8 + i;
        let output = run(prog, candidate_a);
        if let Some(&out_digit) = output.get(0) {
            if out_digit == target[depth] {
                if let Some(result) = find_a(prog, target, candidate_a, depth + 1) {
                    return Some(result);
                }
            }
        }
    }

    None
}

fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();
    let mut a = 0;
    for line in s.as_str().lines() {
        if line.starts_with("Register A:") {
            a = line[11..].trim().parse::<i64>().unwrap();
        }
    }
    let program = s
        .as_str()
        .lines()
        .find(|line| line.starts_with("Program:"))
        .unwrap()[9..]
        .trim()
        .split(',')
        .flat_map(|x| x.chars())
        .map(|x| x.to_digit(10).unwrap())
        .map(|x| x.try_into().unwrap())
        .collect::<Vec<i64>>();

    let output = run(&program, a);
    println!("{:?}", output);
    let target: Vec<i64> = program.iter().rev().cloned().collect();
    if let Some(result) = find_a(&program, &target, 0, 0) {
        println!("part 2: {}", result);
    } else {
        println!("part2: no valid solution");
    }
}

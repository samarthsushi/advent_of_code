fn get_combo(oprnd: i64, registers: &[i64]) -> i64 {
    match oprnd {
        0..=3 => oprnd,
        4     => registers[0],
        5     => registers[1],
        6     => registers[2],
        _     => unreachable!()
    }
}

fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();

    let mut registers = [0; 3];
    for line in s.as_str().lines() {
        if line.starts_with("Register A:") {
            registers[0] = line[11..].trim().parse::<i64>().unwrap();
        } else if line.starts_with("Register B:") {
            registers[1] = line[11..].trim().parse::<i64>().unwrap();
        } else if line.starts_with("Register C:") {
            registers[2] = line[11..].trim().parse::<i64>().unwrap();
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
    let mut ptr = 0;
    let mut output = Vec::new();
    while ptr < program.len() {
        let oprnd = program[ptr+1];
        match program[ptr] {
            0 => registers[0] /= 2_i64.pow(get_combo(oprnd, &registers) as u32),
            1 => registers[1] ^= oprnd,
            2 => registers[1] = get_combo(oprnd, &registers) % 8,
            3 => {
                if registers[0] != 0 {
                    ptr = oprnd.try_into().unwrap();
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => {
                let o = get_combo(oprnd, &registers) % 8;
                output.push(o);
            }
            6 => registers[1] = registers[0] / 2_i64.pow(get_combo(oprnd, &registers) as u32),
            7 => registers[2] = registers[0] / 2_i64.pow(get_combo(oprnd, &registers) as u32),
            _ => unreachable!(),
        }
        ptr+=2;
    }

    println!("{:?}", output);
}

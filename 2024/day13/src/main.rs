struct ClawMachine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn solve_machine(machine: &ClawMachine) -> i64 {
    let det = machine.ax * machine.by - machine.ay * machine.bx;
    if det == 0 {
        return 0;
    }

    let num_a = machine.px * machine.by - machine.py * machine.bx;
    let num_b = machine.py * machine.ax - machine.px * machine.ay;

    if num_a % det != 0 || num_b % det != 0 {
        return 0;
    }

    let a = num_a / det;
    let b = num_b / det;

    if a >= 0 && b >= 0 {
        return 3 * a + b;
    }
    0
}

fn partone(machines: &[ClawMachine]) -> i64 {
    let mut total_cost = 0;

    for machine in machines {
        total_cost += solve_machine(machine);
    }

    total_cost
}

fn parttwo(machines: &mut Vec<ClawMachine>) -> i64 {
    let mut total_cost = 0;

    for machine in machines {
        machine.px += 10000000000000;
        machine.py += 10000000000000;
        total_cost += solve_machine(machine);
    }

    total_cost
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();

    let lines: Vec<&str> = input.lines().map(|line| line.trim()).filter(|line| !line.is_empty()).collect();

    for chunk in lines.chunks(3) {
        let a_parts: Vec<&str> = chunk[0].split(|c| c == 'X' || c == 'Y' || c == ',' || c == ':').collect();
        let ax = a_parts[2].trim().trim_start_matches('+').parse::<i64>().unwrap();
        let ay = a_parts[4].trim().trim_start_matches('+').parse::<i64>().unwrap();

        let b_parts: Vec<&str> = chunk[1].split(|c| c == 'X' || c == 'Y' || c == ',' || c == ':').collect();
        let bx = b_parts[2].trim().trim_start_matches('+').parse::<i64>().unwrap();
        let by = b_parts[4].trim().trim_start_matches('+').parse::<i64>().unwrap();

        let p_parts: Vec<&str> = chunk[2].split(|c| c == 'X' || c == 'Y' || c == '=' || c == ',').collect();
        let px = p_parts[2].trim().parse::<i64>().unwrap();
        let py = p_parts[5].trim().parse::<i64>().unwrap();

        machines.push(ClawMachine {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        });
    }

    machines
}

fn main() {
    let file_path = "data/data.txt";
    let s = std::fs::read_to_string(file_path).unwrap();
    let mut machines = parse_input(&s);
    let p1 = partone(&machines);
    let p2 = parttwo(&mut machines);

    println!("{p1}\n{p2}");
}

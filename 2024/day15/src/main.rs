#[derive(Copy,Clone,Debug)]
enum MoveT {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug,PartialEq,Clone,Copy)]
enum CellStatus {
    Wall,
    Box,
    Empty
}

struct Warehouse {
    grid: [[CellStatus;50];50],
    moves: Vec<MoveT>,
    robot_loc: (usize, usize)
}

impl Warehouse {
    pub fn new(input: String) -> Self {
        let (grid_raw,moves_raw) = input.split_once("\r\n\r\n").unwrap();
        let mut moves = Vec::new();
        for c in moves_raw.chars() {
            match c {
                '^' => moves.push(MoveT::Up),
                '>' => moves.push(MoveT::Right),
                '<' => moves.push(MoveT::Left),
                'v' => moves.push(MoveT::Down),
                _ => {}
            }
        }
        let mut grid = [[CellStatus::Empty; 50]; 50];
        let mut robot_loc = (0,0);
        for (row_idx, line) in grid_raw.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                if row_idx < 50 && col_idx < 50 {
                    match c {
                        '#' => grid[row_idx][col_idx] = CellStatus::Wall,
                        '.' => grid[row_idx][col_idx] = CellStatus::Empty,
                        'O' => grid[row_idx][col_idx] = CellStatus::Box,
                        '@' => robot_loc = (row_idx, col_idx),
                        _ => continue,
                    };
                }
            }
        }
        Self { grid, moves, robot_loc }
    }

    pub fn print_grid(&self) {
        let mut output = String::new();
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if (row_idx, col_idx) == self.robot_loc {
                    output.push('@');
                } else {
                    match cell {
                        CellStatus::Wall => output.push('#'),
                        CellStatus::Empty => output.push('.'),
                        CellStatus::Box => output.push('O'),
                    }
                }
            }
            output.push('\n');
        }
        output.push('\n');
        print!("{}", output);
    }

    fn evaluate_move(&mut self, move_: MoveT) {        
        match move_ {
            MoveT::Up => {
                let (robot_row, robot_col) = self.robot_loc;
                if robot_row > 0 && self.grid[robot_row - 1][robot_col] == CellStatus::Empty {
                    self.robot_loc.0-=1;
                    return;
                }
                if robot_row > 0 && self.grid[robot_row - 1][robot_col] == CellStatus::Wall {
                    return;
                }
                if robot_row > 0 && self.grid[robot_row - 1][robot_col] == CellStatus::Box {
                    let mut last_box_idx = robot_row - 1;

                    while last_box_idx > 0 && self.grid[last_box_idx-1][robot_col] == CellStatus::Box {
                        last_box_idx -= 1;
                    }

                    if self.grid[last_box_idx-1][robot_col] == CellStatus::Empty {
                        self.grid[last_box_idx - 1][robot_col] = CellStatus::Box;
                        self.grid[robot_row - 1][robot_col] = CellStatus::Empty;
                        self.robot_loc.0 = robot_row-1;
                    }
                }
            }
            MoveT::Down => {
                let (robot_row, robot_col) = self.robot_loc;
                if robot_row < 49 && self.grid[robot_row + 1][robot_col] == CellStatus::Empty {
                    self.robot_loc.0+=1;
                    return;
                }
                if robot_row < 49 && self.grid[robot_row + 1][robot_col] == CellStatus::Wall {
                    return;
                }
                if robot_row < 49 && self.grid[robot_row + 1][robot_col] == CellStatus::Box {
                    let mut last_box_idx = robot_row + 1;

                    while last_box_idx < 49 && self.grid[last_box_idx+1][robot_col] == CellStatus::Box {
                        last_box_idx += 1;
                    }

                    if self.grid[last_box_idx+1][robot_col] == CellStatus::Empty {
                        self.grid[last_box_idx + 1][robot_col] = CellStatus::Box;
                        self.grid[robot_row + 1][robot_col] = CellStatus::Empty;
                        self.robot_loc.0 = robot_row+1;
                    }
                }
            }
            MoveT::Left => {
                let (robot_row, robot_col) = self.robot_loc;
                if robot_col > 0 && self.grid[robot_row][robot_col-1] == CellStatus::Empty {
                    self.robot_loc.1-=1;
                    return;
                }
                if robot_col > 0 && self.grid[robot_row][robot_col-1] == CellStatus::Wall {
                    return;
                }
                if robot_col > 0 && self.grid[robot_row][robot_col-1] == CellStatus::Box {
                    let mut last_box_idx = robot_col - 1;

                    while last_box_idx > 0 && self.grid[robot_row][last_box_idx-1] == CellStatus::Box {
                        last_box_idx -= 1;
                    }

                    if self.grid[robot_row][last_box_idx-1] == CellStatus::Empty {
                        self.grid[robot_row][last_box_idx-1] = CellStatus::Box;
                        self.grid[robot_row][robot_col-1] = CellStatus::Empty;
                        self.robot_loc.1 = robot_col-1;
                    }
                }
            }
            MoveT::Right => {
                let (robot_row, robot_col) = self.robot_loc;
                if robot_col < 49 && self.grid[robot_row][robot_col+1] == CellStatus::Empty {
                    self.robot_loc.1+=1;
                    return;
                }
                if robot_col < 49 && self.grid[robot_row][robot_col+1] == CellStatus::Wall {
                    return;
                }
                if robot_col < 49 && self.grid[robot_row][robot_col+1] == CellStatus::Box {
                    let mut last_box_idx = robot_col + 1;

                    while last_box_idx < 49 && self.grid[robot_row][last_box_idx+1] == CellStatus::Box {
                        last_box_idx += 1;
                    }

                    if self.grid[robot_row][last_box_idx+1] == CellStatus::Empty {
                        self.grid[robot_row][last_box_idx+1] = CellStatus::Box;
                        self.grid[robot_row][robot_col+1] = CellStatus::Empty;
                        self.robot_loc.1 = robot_col+1;
                    }
                }
            }
        }
    }

    pub fn partone(&mut self) -> usize {
        let moves = self.moves.clone();
        for move_ in moves {
            self.evaluate_move(move_);
        }

        let mut total_score = 0;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == CellStatus::Box {
                    let score = 100 * row + col;
                    total_score += score;
                }
            }
        }

        total_score
    }
}

fn main() {
    let fp = "data/data.txt";
    let s = std::fs::read_to_string(fp).unwrap();
    let mut wrhs = Warehouse::new(s);
    let p1 = wrhs.partone();
    println!("{p1}");
}

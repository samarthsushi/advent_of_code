const WRHSD: usize = 7;

#[derive(Copy,Clone,Debug,PartialEq)]
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
    Empty,
    BoxL,
    BoxR
}

struct Warehouse1 {
    grid: [[CellStatus;WRHSD];WRHSD],
    moves: Vec<MoveT>,
    robot_loc: (usize, usize)
}

impl Warehouse1 {
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
        let mut grid = [[CellStatus::Empty; WRHSD]; WRHSD];
        let mut robot_loc = (0,0);
        for (row_idx, line) in grid_raw.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                if row_idx < WRHSD && col_idx < WRHSD {
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
                        CellStatus::BoxL => output.push('['),
                        CellStatus::BoxR => output.push(']'),
                    }
                }
            }
            output.push('\n');
        }
        output.push('\n');
        print!("{}", output);
    }

    fn is_in_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < WRHSD as isize && col >= 0 && col < WRHSD as isize
    }

    fn eval(&mut self, move_: MoveT) {
        let (dr, dc) = match move_ {
            MoveT::Up => (-1, 0),
            MoveT::Down => (1, 0),
            MoveT::Left => (0, -1),
            MoveT::Right => (0, 1),
        };
        let (mut new_row, mut new_col) = (
            (self.robot_loc.0 as isize + dr) as usize,
            (self.robot_loc.1 as isize + dc) as usize,
        );
        if self.is_in_bounds(new_row as isize, new_col as isize) {
            match self.grid[new_row][new_col] {
                CellStatus::Empty => {
                    self.robot_loc = (new_row, new_col);
                }
                CellStatus::Wall => {
                    return;
                }
                CellStatus::Box => {
                    let mut last_box_row = new_row as isize;
                    let mut last_box_col = new_col as isize;

                    while self.is_in_bounds(last_box_row + dr, last_box_col + dc)
                        && self.grid[(last_box_row + dr) as usize][(last_box_col + dc) as usize]
                            == CellStatus::Box
                    {
                        last_box_row += dr;
                        last_box_col += dc;
                    }

                    if self.is_in_bounds(last_box_row + dr, last_box_col + dc)
                        && self.grid[(last_box_row + dr) as usize][(last_box_col + dc) as usize]
                            == CellStatus::Empty
                    {
                        self.grid[(last_box_row + dr) as usize][(last_box_col + dc) as usize] =
                            CellStatus::Box;
                        self.grid[new_row][new_col] = CellStatus::Empty;
                        self.robot_loc = (new_row, new_col);
                    }
                }
                _ => unreachable!()
            }
        }
    }

    pub fn exec(&mut self) -> usize {
        let moves = self.moves.clone();
        for move_ in moves {
            self.eval(move_);
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

struct Warehouse2 {
    grid: [[CellStatus;2*WRHSD];WRHSD],
    moves: Vec<MoveT>,
    robot_loc: (usize, usize)
}

impl Warehouse2 {
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
        let mut grid = [[CellStatus::Empty; WRHSD*2]; WRHSD];
        let mut robot_loc = (0,0);
        for (row_idx, line) in grid_raw.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                if row_idx < WRHSD && col_idx < WRHSD {
                    match c {
                        '#' => {
                            grid[row_idx][2*col_idx] = CellStatus::Wall;
                            grid[row_idx][2*col_idx+1] = CellStatus::Wall;
                        }
                        '.' => {
                            grid[row_idx][2*col_idx] = CellStatus::Empty;
                            grid[row_idx][2*col_idx+1] = CellStatus::Empty;
                        }
                        'O' => {
                            grid[row_idx][2*col_idx] = CellStatus::BoxL;
                            grid[row_idx][2*col_idx+1] = CellStatus::BoxR;
                        }
                        '@' => robot_loc = (row_idx, 2*col_idx),
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
                        CellStatus::BoxL => output.push('['),
                        CellStatus::BoxR => output.push(']'),
                    }
                }
            }
            output.push('\n');
        }
        output.push('\n');
        print!("{}", output);
    }

    fn is_in_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < WRHSD as isize && col >= 0 && col < (2*WRHSD) as isize
    }

    fn eval(&mut self, move_: MoveT) {
        match move_ {
            MoveT::Left | MoveT::Right => {
                let dc: isize = if move_ == MoveT::Left { -1 } else { 1 };
                let (robot_row, robot_col) = self.robot_loc;
                let mut current_col: isize = robot_col as isize + dc;
                let mut chain_start = robot_col;
                let mut chain_end = robot_col;
                let mut can_move = false;

                while current_col > 0 {
                    println!("{:?}", self.grid[robot_row][current_col as usize]);
                    match self.grid[robot_row][current_col as usize] {
                        CellStatus::BoxR => {
                            current_col += dc;
                        }
                        CellStatus::BoxL => {
                            current_col += dc;
                        }
                        CellStatus::Empty => {
                            chain_end = (current_col-dc) as usize;
                            can_move = true;
                            break;
                        }
                        _ => return,
                    }
                }
                if can_move {
                    for col in chain_end..=chain_start {
                        if self.grid[robot_row][col] == CellStatus::BoxL {
                            self.grid[robot_row][(col as isize + dc) as usize] = CellStatus::BoxL;
                            self.grid[robot_row][col] = CellStatus::Empty;
                        } else if self.grid[robot_row][col] == CellStatus::BoxR {
                            self.grid[robot_row][(col as isize + dc) as usize] = CellStatus::BoxR;
                            self.grid[robot_row][col] = CellStatus::Empty;
                        }
                    }
                    self.robot_loc = (robot_row, (robot_col as isize + dc) as usize);
                }
            }
            MoveT::Up => {
                todo!();
            }
            MoveT::Down => {
                todo!();
            }
        }    
    }

    pub fn exec(&mut self) -> usize {
        println!("init");
        self.print_grid();
        let moves = self.moves.clone();
        for move_ in moves {
            println!("{:?}", move_);
            self.eval(move_);
            self.print_grid();
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
    // let mut wrhs1 = Warehouse1::new(s.clone());
    // let p1 = wrhs1.exec();
    // println!("{p1}");
    let mut wrhs2 = Warehouse2::new(s);
    let p2 = wrhs2.exec();
    println!("{p2}");
}

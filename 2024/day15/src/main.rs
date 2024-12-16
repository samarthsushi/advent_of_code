const WRHSD: usize = 7;

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
                CellStatus::BoxL | CellStatus::BoxR => {
                    let affected_boxes_ = self.find_affected_boxes(new_row, new_col, dr, dc);
                    let mut set = std::collections::HashSet::new();
                    let affected_boxes = affected_boxes_.into_iter()
                        .filter(|item| set.insert(item.clone()))
                        .collect::<Vec<(usize,usize)>>();
                    println!("a_b:{:?}", affected_boxes);
                    let mut updates = vec![];
                    for (row, col) in affected_boxes.iter().rev() {
                        // Calculate new positions for BoxL and BoxR
                        let new_row = (row.to_owned() as isize + dr) as usize;
                        let new_col = (col.to_owned() as isize + dc) as usize;
    
                        updates.push((CellStatus::BoxL, (*row, *col), (new_row, new_col)));
                        updates.push((
                            CellStatus::BoxR,
                            (*row, col + 1),      // BoxR is always to the right of BoxL
                            (new_row, new_col + 1), // Move BoxR accordingly
                        ));
                    }

                    println!("{:?}", updates);
                
                    // Apply all updates
                    for (cell_type, (old_row, old_col), (new_row, new_col)) in updates {
                        self.grid[new_row][new_col] = cell_type; // Move the cell
                        self.grid[old_row][old_col] = CellStatus::Empty; // Clear the old position
                    }
                

                    self.robot_loc = (
                        (self.robot_loc.0 as isize + dr) as usize,
                        (self.robot_loc.1 as isize + dc) as usize,
                    );
                }
                _ => unreachable!()
            }
        }

    }

    fn find_affected_boxes(
        &self,
        start_row: usize,
        start_col: usize,
        row_delta: isize,
        col_delta: isize,
    ) -> Vec<(usize, usize)> {
        let mut affected_boxes = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        queue.push_back((start_row, start_col));
        println!("q:{:?}",queue);

        while let Some((current_row, current_col)) = queue.pop_front() {
            let box_left_col = if self.grid[current_row][current_col] == CellStatus::BoxR {
                current_col - 1
            } else {
                current_col
            };

            affected_boxes.push((current_row, box_left_col));

            let next_row = (current_row as isize + row_delta) as usize;
            let next_col = (current_col as isize + col_delta) as usize;

            if self.is_in_bounds(next_row as isize, next_col as isize)
                && (self.grid[next_row][next_col] == CellStatus::BoxL
                    || self.grid[next_row][next_col] == CellStatus::BoxR)
            {
                queue.push_back((next_row, next_col));
                println!("q:{:?}", queue);
            }
        }

        affected_boxes
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

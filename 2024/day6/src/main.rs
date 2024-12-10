use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellStatus {
    Free,
    Obstructed,
    Visited,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Guard {
    x: isize,
    y: isize,
    facing: Direction
}

impl Guard {
    pub fn new() -> Self {
        Self { x: 0, y: 0, facing: Direction::Up }
    }

    pub fn move_peek(&mut self) -> (isize, isize) {
        match self.facing {
            Direction::Up => (self.x, self.y-1),
            Direction::Down => (self.x, self.y+1),
            Direction::Left => (self.x-1, self.y),
            Direction::Right => (self.x+1, self.y),
        }
    }

    pub fn rotate_right(&mut self) {
        match self.facing {
            Direction::Up => self.facing = Direction::Right,
            Direction::Right => self.facing = Direction::Down,
            Direction::Down => self.facing = Direction::Left,
            Direction::Left => self.facing = Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Lab {
    grid: [[CellStatus; 130]; 130],
    guard: Guard
}

impl Lab {
    pub fn new(input: String) -> Self {
        let mut grid = [[CellStatus::Free; 130]; 130];
        let mut guard = Guard::new();
        let mut y = 0;
        for line in input.lines() {
            let mut x = 0;
            for c in line.chars() {
                match c {
                    '.' => {},
                    '#' => grid[x][y] = CellStatus::Obstructed,
                    '^' => {
                        guard.x = x as isize;
                        guard.y = y as isize;
                        grid[x][y] = CellStatus::Visited;
                    }
                    _ => panic!("invalid char encountered while parsing")
                }
                x+=1;
            }
            y+=1;
        }
        Self { grid, guard }
    }

    pub fn partone(&mut self) -> usize {
        loop {
            let (nx, ny) = self.guard.move_peek();
            if nx < 0 || nx > 129 || ny < 0 || ny > 129 {
                break;
            }
            if self.grid[nx as usize][ny as usize] == CellStatus::Obstructed {
                self.guard.rotate_right();
                continue;
            }
            self.guard.x = nx;
            self.guard.y = ny;
            self.grid[nx as usize][ny as usize] = CellStatus::Visited;
        }
        let mut visited = 0;
        for row in self.grid {
            for cell in row {
                if cell == CellStatus::Visited { visited+=1 };
            }
        }

        visited
    }

    pub fn parttwo(&self) -> usize {
        let mut ox: isize = 0;
        let mut oy: isize = 0;
        let mut valid = 0;
        while oy < 129 || ox < 129 {
            let mut guard = self.guard.clone();
            if self.grid[ox as usize][oy as usize] == CellStatus::Obstructed {
                if ox == 129 {
                    ox=0;
                    oy+=1;
                } else {
                    ox+=1;
                }
                continue;
            }
            if ox == guard.x && oy == guard.y {
                if ox == 129 {
                    ox=0;
                    oy+=1;
                } else {
                    ox+=1;
                }
                continue;
            }
            
            let mut visited_nodes = std::collections::HashSet::new();
            visited_nodes.insert(guard.clone());

            loop {
                let (nx, ny) = guard.move_peek();
                if nx < 0 || nx > 129 || ny < 0 || ny > 129 {
                    break;
                }
                if self.grid[nx as usize][ny as usize] == CellStatus::Obstructed || (nx, ny) == (ox, oy) {
                    guard.rotate_right();
                    visited_nodes.insert(guard.clone());
                    continue;
                }
                guard.x = nx;
                guard.y = ny;
                if visited_nodes.contains(&guard) {
                    valid += 1;
                    break;
                }
                visited_nodes.insert(guard.clone());
            }
            if ox == 129 {
                ox=0;
                oy+=1;
            } else {
                ox+=1;
            }
        }
        valid
    }
}

fn main() {
    let file_path = "data/data.txt";
    let s = fs::read_to_string(file_path).unwrap();

    let mut lab = Lab::new(s);
    let p2 = lab.parttwo();
    let p1 = lab.partone();
    println!("{}\n{}", p1, p2);
}

use std::collections::HashSet;
use std::collections::VecDeque;

struct TopographicalMap {
    grid: [[isize; 52]; 52],
}

impl TopographicalMap {
    pub fn new(input: String) -> Self {
        let mut row = 0;
        
        let mut grid = [[0; 52]; 52];
        for l in input.lines() {
            let l = l.trim();
            let mut col = 0;
            for c in l.chars() {
                grid[row][col] = c.to_digit(10).unwrap().try_into().unwrap();
                col+=1;
            }
            row+=1;
        }

        Self { grid }
    }

    pub fn count_trails(
        &self,
        row: isize,
        col: isize,
        prev: isize,
    ) -> usize {
        if !(0..52).contains(&row) || !(0..52).contains(&col) {
            return 0;
        }

        let prev_peek = self.grid[row as usize][col as usize];

        if prev_peek != prev + 1 {
            return 0;
        }

        if prev_peek == 9 {
            return 1;
        }

        let total_trails = self.count_trails(row, col - 1, prev_peek)
            + self.count_trails(row, col + 1, prev_peek)
            + self.count_trails(row - 1, col, prev_peek)
            + self.count_trails(row + 1, col, prev_peek);

        total_trails
    }

    pub fn parttwo(&self) -> usize {
        let mut total_score = 0;

        for row in 0..52 {
            for col in 0..52 {
                if self.grid[row][col] == 0 {
                    total_score += self.count_trails(row as isize, col as isize, -1);
                }
            }
        }

        total_score
    }

    pub fn find_reachable_nines(&self, start: (isize, isize)) -> usize {
        let mut q = VecDeque::from([start]);
        let mut seen = HashSet::new();
        let mut count = 0;

        while let Some((r, c)) = q.pop_front() {
            if !seen.insert((r, c)) {
                continue;
            }

            let curr_height = self.grid[r as usize][c as usize];
            if curr_height == 9 {
                count += 1;
                continue;
            }

            let next = curr_height + 1;
            let neighbours = [
                (r + 1, c),
                (r-1, c),
                (r, c + 1),
                (r, c-1),
            ];

            for (rr, cc) in neighbours {
                if let Some(&val) = self.grid.get(rr as usize).and_then(|row: &[isize; 52]| row.get(cc as usize)) {
                    if val == next {
                        q.push_back((rr, cc));
                    }
                }
            }
        }

        count
    }

    pub fn partone(&self) -> usize {
        let mut total_score = 0;

        for row in 0..52 {
            for col in 0..52 {
                if self.grid[row][col] == 0 {
                    let score = self.find_reachable_nines((row as isize, col as isize));
                    total_score += score;
                }
            }
        }

        total_score
    }
}

fn main() {
    let file_path = "data/data.txt";
    let s = std::fs::read_to_string(file_path).unwrap();

    let map = TopographicalMap::new(s);
    let p2 = map.parttwo();
    let p1 = map.partone();
    println!("{p1}\n{p2}");
}

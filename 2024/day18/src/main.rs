use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &[[usize; 71]; 71]) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut distances = vec![vec![usize::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();

    distances[0][0] = 0;
    heap.push(State { cost: 0, position: (0, 0) });

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(State { cost, position }) = heap.pop() {
        let (x, y) = position;

        if x == rows - 1 && y == cols - 1 {
            return Some(cost);
        }
        if cost > distances[x][y] {
            continue;
        }
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] == 1 {
                    continue;
                }

                let next_cost = cost + 1;

                if next_cost < distances[nx][ny] {
                    distances[nx][ny] = next_cost;
                    heap.push(State { cost: next_cost, position: (nx, ny) });
                }
            }
        }
    }

    None
}

fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();
    let mut grid = [[0; 71]; 71];
    for line in s.as_str().lines().take(1024) {
        let (x,y) = line.split_once(',').unwrap();
        let x: usize = x.trim().parse().unwrap();
        let y: usize = y.trim().parse().unwrap();
        grid[y][x] = 1;
    }

    if let Some(min_cost) = dijkstra(&grid) {
        println!("{}", min_cost);
    } else {
        println!("no path");
    }
    

}

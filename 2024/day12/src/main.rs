use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

type Grid = Vec<Vec<char>>;
type Regions = Vec<Vec<(usize,usize)>>;

fn flood_fill(grid: &Grid, x: usize, y: usize, marker: usize, visited: &mut HashSet<(usize, usize)>, regions: &mut Regions) {
    let rows = grid.len();
    let cols = grid[0].len();
    let original = grid[x][y];

    let mut stack = vec![(x, y)];
    while let Some((cx, cy)) = stack.pop() {
        if visited.contains(&(cx, cy)) {
            continue;
        }
        visited.insert((cx, cy));
        regions[marker].push((cx,cy));

        for (dx, dy) in &DIRECTIONS {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;

            if nx >= 0 && ny >= 0 && nx < rows as isize && ny < cols as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == original && !visited.contains(&(nx, ny)) {
                    stack.push((nx, ny));
                }
            }
        }
    }
}

fn find_regions(grid: &Grid) -> Regions {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if !visited.contains(&(x, y)) {
                regions.push(Vec::new());
                let marker = regions.len()-1;
                flood_fill(grid, x, y, marker, &mut visited, &mut regions);
            }
        }
    }
    regions
}

fn calc_perimeter(region: &Vec<(usize, usize)>, grid: &Grid) -> usize {
    let mut perimeter = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for &(x, y) in region {
        for (dx, dy) in &DIRECTIONS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                perimeter += 1;
            } else {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] != grid[x][y] {
                    perimeter += 1;
                }
            }
        }
    }

    perimeter
}

fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();
    let mut grid = Vec::new();
    for line in s.as_str().lines() {
        let line = line.trim();
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let regions = find_regions(&grid);
    let mut p1 = 0;
    for region in regions.iter() {
        let perimeter = calc_perimeter(region, &grid);
        let area = region.len();
        p1+=area*perimeter;
    }
    println!("{p1}");
}
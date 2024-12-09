/*
external_antinode_pair_locs = (antenna_1 + slope, antenna_2 - slope) where,
                                                                slope = |antenna_1 - antenna_2|
                                                              & antenna_1.y > antenna_2.y
                                                                (i.e. antenna_1 is below antenna_2 in the 2d map)

internal_antinode_pair_locs = if slope % 3 { return (antenna_2 + slope/3, antenna_2 + 2*slope/3) }
*/
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellStatus {
	Antinode,
	Nothing
}

#[derive(Debug)]
struct Map {
	grid: [[CellStatus; 50]; 50],
	antenna_clusters: HashMap<char, Vec<(isize, isize)>>
}

impl Map {
	pub fn new(input: String) -> Self {
		let grid = [[CellStatus::Nothing; 50]; 50];
		let mut antenna_clusters: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
		let mut y = 0;
		for l in input.as_str().lines() {
			let mut x = 0;
			for c in l.chars() {
				match c {
					'.' => {},
					ant_ty => antenna_clusters.entry(ant_ty).or_insert(Vec::new()).push((y,x)),
				}
				x+=1;
			}
			y+=1;
		}
		Self { grid, antenna_clusters }
	}

	pub fn partone(&mut self) -> usize {
		for (_, loc_t) in &self.antenna_clusters {
			for i in 0..loc_t.len() {
				for j in 0..loc_t.len() {
					if i==j { continue; }
					let dy = loc_t[i].0 - loc_t[j].0;
					let dx = loc_t[i].1 - loc_t[j].1;

					if dx % 3 == 0 && dy % 3 == 0 {
						let internal_a1y = loc_t[i].0 + (dy/3);
						let internal_a1x = loc_t[i].1 + (dx/3);
						self.grid[internal_a1y as usize][internal_a1x as usize] = CellStatus::Antinode;

						let internal_a2y = loc_t[i].0 + (2*dy/3);
						let internal_a2x = loc_t[i].1 + (2*dx/3);
						self.grid[internal_a2y as usize][internal_a2x as usize] = CellStatus::Antinode;
					}

					let antinode1y = loc_t[i].0 + dy;
					let antinode1x = loc_t[i].1 + dx;
					if (0..50).contains(&antinode1x) && (0..50).contains(&antinode1y) {
						self.grid[antinode1y as usize][antinode1x as usize] = CellStatus::Antinode;
					}
					let antinode2y = loc_t[j].0 - dy;
					let antinode2x = loc_t[j].1 - dx;
					if (0..50).contains(&antinode2x) && (0..50).contains(&antinode2y) {
						self.grid[antinode2y as usize][antinode2x as usize] = CellStatus::Antinode;
					}
				}
			}
		}
		let mut antinodes = 0;
		for row in self.grid {
			for col in row {
				if CellStatus::Antinode == col { antinodes+=1; }
			}
		}
		antinodes
	}

	pub fn parttwo(&mut self) -> usize {
		for (_, loc_t) in &self.antenna_clusters {
			for i in 0..loc_t.len() {
				for j in 0..loc_t.len() {
					if i==j { continue; }
					self.grid[loc_t[i].0 as usize][loc_t[i].1 as usize] = CellStatus::Antinode;
					self.grid[loc_t[j].0 as usize][loc_t[j].1 as usize] = CellStatus::Antinode;
					let dy = loc_t[i].0 - loc_t[j].0;
					let dx = loc_t[i].1 - loc_t[j].1;

					if dx % 3 == 0 && dy % 3 == 0 {
						let internal_a1y = loc_t[i].0 + (dy/3);
						let internal_a1x = loc_t[i].1 + (dx/3);
						self.grid[internal_a1y as usize][internal_a1x as usize] = CellStatus::Antinode;

						let internal_a2y = loc_t[i].0 + (2*dy/3);
						let internal_a2x = loc_t[i].1 + (2*dx/3);
						self.grid[internal_a2y as usize][internal_a2x as usize] = CellStatus::Antinode;
					}

					let mut antinode1y = loc_t[i].0 + dy;
					let mut antinode1x = loc_t[i].1 + dx;
					while (0..50).contains(&antinode1x) && (0..50).contains(&antinode1y) {
						self.grid[antinode1y as usize][antinode1x as usize] = CellStatus::Antinode;
						antinode1y+=dy;
						antinode1x+=dx;
					}
					let mut antinode2y = loc_t[j].0 - dy;
					let mut antinode2x = loc_t[j].1 - dx;
					while (0..50).contains(&antinode2x) && (0..50).contains(&antinode2y) {
						self.grid[antinode2y as usize][antinode2x as usize] = CellStatus::Antinode;
						antinode2x+=dx;
						antinode2y+=dy;
					}
				}
			}
		}
		let mut antinodes = 0;
		for row in self.grid {
			for col in row {
				if CellStatus::Antinode == col { antinodes+=1; }
			}
		}
		antinodes
	}
}

fn main() {
	let file_path = "data/data.txt";
	let s = std::fs::read_to_string(file_path).unwrap();

	let mut map = Map::new(s);
	let p1 = map.partone();
	let p2 = map.parttwo();
	println!("{p1}\n{p2}");
}
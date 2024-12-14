use std::cmp::Ordering::{Less, Greater, Equal};
use std::io::Write;
use image::{ImageBuffer, Rgb};

const GRIDLENGTH: isize = 101;
const GRIDHEIGHT: isize = 103;

struct Guard {
    vx: isize,
    vy: isize,
    px: isize,
    py: isize
}

struct Floor {
    guards: Vec<Guard>,
}

impl Floor {
    pub fn new(input: String) -> Self {
        let mut guards = Vec::new();
        for line in input.as_str().lines() {
            let parts: Vec<&str> = line.split(|c| c == 'p' || c == 'v' || c == '=' || c == ',').collect();
            let px = parts[2].parse::<isize>().unwrap();
            let py = parts[3].trim().parse::<isize>().unwrap();
            let vx = parts[5].parse::<isize>().unwrap();
            let vy = parts[6].parse::<isize>().unwrap();
            guards.push(Guard{vx,vy,px,py});
        }
        Self {guards}
    }

    pub fn partone(&mut self) -> usize {
        let mut counts = [0; 4];
        for guard in &mut self.guards {
            guard.px = ((guard.px + 100*guard.vx) % GRIDLENGTH + GRIDLENGTH) % GRIDLENGTH;
            guard.py = ((guard.py + 100*guard.vy) % GRIDHEIGHT + GRIDHEIGHT) % GRIDHEIGHT;

            let index = match (guard.px.cmp(&(GRIDLENGTH / 2)), guard.py.cmp(&(GRIDHEIGHT / 2))) {
                (Equal, _) | (_, Equal) => continue,
                (Less, Less) => 0,
                (Less, Greater) => 1,
                (Greater, Less) => 2,
                (Greater, Greater) => 3,
            };
            counts[index] += 1;
        }
        counts.iter().product()
    }

    pub fn parttwo(&mut self) {
        for i in 0..10000 {
            let mut img = ImageBuffer::new(GRIDLENGTH as u32, GRIDHEIGHT as u32);
            for pixel in img.pixels_mut() {
                *pixel = Rgb([0u8, 0u8, 0u8]);
            }
            for guard in &mut self.guards {
                guard.px = ((guard.px + guard.vx) % GRIDLENGTH + GRIDLENGTH) % GRIDLENGTH;
                guard.py = ((guard.py + guard.vy) % GRIDHEIGHT + GRIDHEIGHT) % GRIDHEIGHT;
                img.put_pixel(guard.px as u32, guard.py as u32, Rgb([255u8, 255u8, 255u8]));
            }
            let filename = format!("data/{}.png", i);
            img.save(&filename).unwrap();
        }
    }
}
fn main() {
    let file_path = "data/data.txt";
    let s = std::fs::read_to_string(file_path).unwrap();
    let mut floor = Floor::new(s);
    let p2 = floor.parttwo();
}

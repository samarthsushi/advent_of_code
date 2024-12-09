#[derive(Debug, Clone, PartialEq)]
enum BlockT {
    Free,
    File(usize)
}

struct DiskMap {
    disk: Vec<BlockT>,
}

impl DiskMap {
    pub fn new(input: String) ->  Self {
        let mut is_free_block = false;
        let mut fileid = 0;
        let mut disk = Vec::new();
        for c in input.chars() {
            if is_free_block {
                disk.extend(std::iter::repeat(BlockT::Free).take(c.to_digit(10).unwrap().try_into().unwrap()));
                is_free_block = false;
            } else {
                disk.extend(std::iter::repeat(BlockT::File(fileid)).take(c.to_digit(10).unwrap().try_into().unwrap()));
                is_free_block = true;
                fileid+=1;
            }
        }
        Self { disk }
    }

    fn checksum(&self) -> u64 {
        let mut checksum: u64 = 0;
        for (i,block) in self.disk.iter().enumerate() {
            match block {
                BlockT::File(x) => {
                    checksum += (i * x) as u64;
                }
                BlockT::Free => {}
            }
        }
        checksum
    }

    pub fn partone(&mut self) -> u64 {
        let mut reverse_iter = self.disk.len() - 1;
        let mut forward_iter = 0;
        while forward_iter < reverse_iter {
            while matches!(self.disk[forward_iter], BlockT::File(_)) {
                forward_iter+=1;
            }
            while matches!(self.disk[reverse_iter], BlockT::Free) {
                reverse_iter-=1;
            }
            if forward_iter >= reverse_iter {
                break;
            }
            self.disk.swap(forward_iter, reverse_iter);
        }
        self.checksum()
    }

    pub fn parttwo(&mut self) -> u64 {
        self.checksum()
    }
}

fn main() {
    let file_path = "data/data.txt";
    let s = std::fs::read_to_string(file_path).unwrap();

    let mut diskmap = DiskMap::new(s);
    // let p1 = diskmap.partone();
    let p2 = diskmap.parttwo();
    println!("{}", p2);
}

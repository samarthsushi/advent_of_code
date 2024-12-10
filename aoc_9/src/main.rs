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
                let size = c.to_digit(10).unwrap().try_into().unwrap();
                disk.extend(std::iter::repeat(BlockT::Free).take(size));
                is_free_block = false;
            } else {
                let size = c.to_digit(10).unwrap().try_into().unwrap();
                disk.extend(std::iter::repeat(BlockT::File(fileid)).take(size));
                fileid+=1;
                is_free_block = true;
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
        let mut free_spaces = Vec::new();
        let mut file_spaces = Vec::new();
        let mut iter = self.disk.iter().enumerate().peekable();

        while let Some((start, block)) = iter.next() {
            match block {
                BlockT::Free => {
                    let mut size = 1;
                    while let Some((_, next_block)) = iter.peek() {
                        if matches!(next_block, BlockT::Free) {
                            size += 1;
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    free_spaces.push((start, size));
                }
                BlockT::File(file_id) => {
                    let mut size = 1;
                    while let Some((_, next_block)) = iter.peek() {
                        if let BlockT::File(next_id) = next_block {
                            if next_id == file_id {
                                size += 1;
                                iter.next();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    file_spaces.push((start, size));
                }
            }
        }
        for (start, size) in file_spaces.iter().rev() {
            if let Some((free_start, free_size)) = free_spaces
                .iter_mut()
                .find(|(free_start, free_size)| *free_size >= *size && *free_start < *start)
            {
                for offset in 0..*size {
                    self.disk.swap(*free_start + offset, *start + offset);
                }
                *free_start += *size;
                *free_size -= *size;
            }
        }
        self.checksum()
    }
}

fn main() {
    let file_path = "data/data.txt";
    let s = std::fs::read_to_string(file_path).unwrap();

    let mut diskmap = DiskMap::new(s);
    let p2 = diskmap.parttwo();
    println!("{p2}")
}

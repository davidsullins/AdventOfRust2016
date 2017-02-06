// advent16.rs
// dragon curve

use std::io;

type BV = Vec<bool>;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut rd = RandomData::create_from_str(&input);
    // part 1
    rd.grow(272);
    let checksum1 = string_from_bv(&rd.calc_checksum());
    println!("part 1 checksum: {}", checksum1);

    // part 2
    rd.grow(35651584);
    let checksum2 = string_from_bv(&rd.calc_checksum());
    println!("part 2 checksum: {}", checksum2);

}

// ///////
// Part 1

struct RandomData {
    bv: BV,
    pos: usize,
}

impl RandomData {
    fn create_from_str(s: &str) -> RandomData {
        RandomData {
            pos: 0,
            bv: s.chars()
                .filter_map(|c| match c {
                    '0' => Some(false),
                    '1' => Some(true),
                    _ => None,
                })
                .collect(),
        }
    }

    fn grow(&mut self, new_len: usize) {
        let old_len = self.bv.len();
        if new_len > old_len {
            self.bv.reserve(new_len - old_len);
            for _ in old_len..new_len {
                if self.pos == 0 {
                    self.pos = self.bv.len();
                    self.bv.push(false);
                } else {
                    self.pos -= 1;
                    let new_val = !self.bv[self.pos];
                    self.bv.push(new_val);
                }
            }
        }
    }

    fn calc_checksum(&self) -> BV {
        let disk_len = self.bv.len();
        let trailing = count_trailing_zeros(disk_len);

        if trailing == 0 {
            self.bv.clone()
        } else {
            let chunk_size = 1 << trailing;

            // checksum of each chunk is true iff the number of true elements is even
            self.bv
                .chunks(chunk_size)
                .map(|chunk| chunk.iter().filter(|&&x| x).count() % 2 == 0)
                .collect()
        }
    }
}

fn count_trailing_zeros(x: usize) -> u32 {
    // Convert trailing 0's to 1's and set everything else to 0. Then just count 1's
    // http://graphics.stanford.edu/~seander/bithacks.html#ZerosOnRightLinear
    ((x ^ (x - 1)) >> 1).count_ones()
}

#[allow(unknown_lints)]
#[allow(ptr_arg)]
fn string_from_bv(bv: &BV) -> String {
    bv.iter().map(|&x| if x { '1' } else { '0' }).collect()
}

// //////
// Tests
#[cfg(test)]
mod tests {
    use super::RandomData;
    use super::count_trailing_zeros;
    use super::string_from_bv;

    #[test]
    fn test_rd_create_from_str() {
        let bv = RandomData::create_from_str("100").bv;
        assert_eq!(3, bv.len());
        assert_eq!(true, bv[0]);
        assert_eq!(false, bv[1]);
        assert_eq!(false, bv[2]);
    }

    #[test]
    fn test_rd_grow() {
        does_rd_grow_match("1", "100");
        does_rd_grow_match("0", "001");
        does_rd_grow_match("11111", "11111000000");
        does_rd_grow_match("111100001010", "1111000010100101011110000");
        // check a few more iterations too
        does_rd_grow_match("0", "001001100011011");
    }

    fn does_rd_grow_match(input: &str, expect: &str) {
        let len = expect.len();
        let mut rd_in = RandomData::create_from_str(input);
        rd_in.grow(len);
        assert_eq!(&string_from_bv(&rd_in.bv), expect);
    }

    #[test]
    fn test_count_trailing_zeros() {
        assert_eq!(0, count_trailing_zeros(0b1));
        assert_eq!(1, count_trailing_zeros(0b1110));
        assert_eq!(3, count_trailing_zeros(0b10101000));
    }

    #[test]
    fn test_rd_checksum() {
        does_rd_checksum_match("110010110100", 12, "100");
        does_rd_checksum_match("10000", 20, "01100");
    }

    fn does_rd_checksum_match(input: &str, len: usize, expect: &str) {
        let mut rd_in = RandomData::create_from_str(input);
        rd_in.grow(len);
        assert_eq!(&string_from_bv(&rd_in.calc_checksum()), expect);
    }
}

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let blocks = parse_input(&input);
        let compacted = compact_blocks(blocks);
        calculate_checksum(&compacted).to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let blocks = parse_input(&input);
        let compacted = compact_blocks_2(blocks);
        log::info!("{:?}", compacted);
        calculate_checksum(&compacted).to_string()
    }
}

#[derive(Copy, Clone, Debug)]
enum Block {
    Blank { size: u32 },
    File { size: u32, id: usize },
}

fn parse_input(input: &str) -> Vec<Block> {
    input
        .trim()
        .char_indices()
        .map(|(i, c)| {
            if i % 2 == 0 {
                Block::File {
                    size: c.to_digit(10).unwrap(),
                    id: i / 2,
                }
            } else {
                Block::Blank {
                    size: c.to_digit(10).unwrap(),
                }
            }
        })
        .collect()
}

fn compact_blocks(mut blocks: Vec<Block>) -> Vec<Block> {
    let mut i = 1usize;
    while i < blocks.len() {
        let last_i = blocks.len() - 1;
        let Block::File { size, id } = blocks[last_i] else {
            panic!("woops")
        };
        let Block::Blank { size: blank_size } = blocks[i] else {
            panic!("woops {i} was file")
        };
        if blank_size == 0 {
            i += 2;
        } else if size <= blank_size {
            blocks.pop(); // the last file
            blocks.pop(); // the blank before the last file
            blocks.insert(i, Block::File { size, id });
            if i + 1 < blocks.len() {
                if size == blank_size {
                    blocks.remove(i + 1);
                    i += 2;
                } else {
                    blocks[i + 1] = Block::Blank {
                        size: blank_size - size,
                    };
                    i += 1;
                }
            } else {
                i += 1;
            }
        } else {
            blocks[last_i] = Block::File {
                size: size - blank_size,
                id,
            };
            blocks[i] = Block::File {
                size: blank_size,
                id,
            };
            i += 2;
        }
    }
    blocks
}

fn compact_blocks_2(mut blocks: Vec<Block>) -> Vec<Block> {
    let mut i = blocks.len() - 1;
    while i > 0 {
        let Block::File { size, id } = blocks[i] else {
            i -= 1;
            continue;
        };
        let blank_i = blocks[0..i].iter().position(|block| {
            if let Block::Blank { size: blank_size } = block {
                *blank_size >= size
            } else {
                false
            }
        });
        if let Some(blank_i) = blank_i {
            let Block::Blank { size: blank_size } = blocks[blank_i] else {
                panic!("{blank_i} was not a blank")
            };
            log::trace!("moving block({size}, {id}) to blank({blank_size}) at {blank_i}");
            blocks[i] = Block::Blank { size };
            blocks.insert(blank_i, Block::File { size, id });
            blocks[blank_i + 1] = Block::Blank {
                size: blank_size - size,
            };
        } else {
            i -= 1;
        }
    }
    blocks
}

fn calculate_checksum(blocks: &[Block]) -> usize {
    let mut i = 0;
    let mut res = 0;
    for block in blocks {
        match block {
            Block::File { size, id } => {
                let s = *size as usize;
                res += id * (s * i + s * (s - 1) / 2);
                i += s;
            }
            Block::Blank { size } => {
                i += *size as usize;
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::{init_test_logging, Solution};

    #[test]
    fn part_1() {
        init_test_logging();

        let input = "2333133121414131402".to_string();
        let answer = Solution.solve_1(input);
        assert_eq!("1928", answer);
    }

    #[test]
    fn part_2() {
        init_test_logging();

        let input = "2333133121414131402".to_string();
        let answer = Solution.solve_2(input);
        assert_eq!("2858", answer);
    }
}

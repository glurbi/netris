extern crate rand;


mod tetris {

    use std::fmt;
    use rand::prelude::*;
    use rand::seq::SliceRandom;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Block {
        width: usize,
        height: usize,
        cells: String,
    }

    #[derive(Debug)]
    pub struct Board {
        width: usize,
        height: usize,
        cells: String,
        score: usize,
        block_pos: (usize, usize),
        block: Block,
        blocks: Vec<Block>,
        rng: ThreadRng,
    }

    impl Block {

        fn rotate(&self) -> Block {
            let mut cells = String::new();
            let bytes = self.cells.as_bytes();
            for i in (0..self.width).rev() {
                for j in 0..self.height {
                    cells.push(bytes[j*self.width+i] as char);
                }
            }
            Block { width: self.height, height: self.width, cells }
        }

        fn cell(&self, i: usize, j: usize) -> char {
            self.cells.as_bytes()[j*self.width + i] as char
        }
    }

    fn none() -> Block {
        Block { width: 0, height: 0, cells: "".to_string() }
    }

    fn o() -> Block {
        Block { width: 2, height: 2, cells: "####".to_string() }
    }

    fn i() -> Block {
        Block { width: 1, height: 4, cells: "####".to_string() }
    }

    fn s() -> Block {
        Block { width: 3, height: 2, cells: ".####.".to_string() }
    }

    fn z() -> Block {
        Block { width: 3, height: 2, cells: "##..##".to_string() }
    }

    fn l() -> Block {
        Block { width: 3, height: 2, cells: "####..".to_string() }
    }

    fn j() -> Block {
        Block { width: 3, height: 2, cells: "###..#".to_string() }
    }

    fn t() -> Block {
        Block { width: 3, height: 2, cells: "###.#.".to_string() }
    }

    fn default_blocks() -> Vec<Block> {
        vec![o(), i(), s(), z(), l(), j(), t()]
    }

    impl Board {

        pub fn new(width: usize, height: usize) -> Board {
            let cells = ".".repeat(width*height);
            let rng = rand::thread_rng();
            Board {
                width,
                height,
                cells,
                score: 0,
                block: none(),
                block_pos: (0, 0),
                blocks: default_blocks(),
                rng,
            }
        }

        pub fn step(&mut self) {
            
        }

        fn pick_block(&mut self) {
            self.block = self.blocks.choose(&mut self.rng).unwrap().clone();
            self.block_pos = (self.width/2, 0);
        }

        pub fn move_right(&mut self) -> bool {
            let new_block_pos = (self.block_pos.0 + 1, self.block_pos.1);

            if new_block_pos.0 + self.block.width > self.width {
                return false;
            }

            if self.collide(&self.block, new_block_pos) {
                return false;
            }

            self.block_pos = new_block_pos;
            return true;
        }

        fn settle_block(&mut self) {
            for j in 0..self.block.height {
                for i in 0..self.block.width {
                    if self.block.cell(i,j) == '#' {
                        let offset = j*self.width + i + self.block_pos.0;
                        unsafe {
                            self.cells.as_bytes_mut()[offset] = '#' as u8;
                        }
                    }
                }
            }
        }

        fn cell(&self, (i, j): (usize, usize)) -> char {
            self.cells.as_bytes()[j*self.width + i] as char
        }

        fn collide(&self, block: &Block, pos: (usize,usize)) -> bool {
            for j in 0..block.height {
                for i in 0..block.width {
                    if block.cell(i,j) == '#' && self.cell((i+pos.0, j+pos.1)) == '#' {
                        return true;
                    }
                }
            }
            false
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let cells = self.cells.clone();
            let mut s = String::with_capacity((self.width+1)*self.height);
            for j in 0..self.height {
                s.push_str(&cells[j*self.width..j*self.width+self.width]);
                s.push('\n');
            }
            write!(f, "{}", s)
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_move_blocks() {
            let mut board = Board::new(3, 20);
            assert_eq!(board.cells.len(), 3 * 20);
            board.block = i();
            assert!(board.move_right());
            assert!(board.move_right());
            assert!(!board.move_right());
            board.settle_block();
            println!("{}", board);
            println!("{:?}", board);
        }

        #[test]
        fn test_rotate_block() {
            assert_eq!(o().rotate(), o());
            assert_eq!(i().rotate(), Block { width: 4, height: 1, cells: "####".to_string() });
            assert_eq!(i().rotate().rotate(), i());
            assert_eq!(s().rotate(), Block { width: 2, height: 3, cells: "#.##.#".to_string() });
            assert_eq!(s().rotate().rotate(), s());
            assert_eq!(z().rotate(), Block { width: 2, height: 3, cells: ".####.".to_string() });
            assert_eq!(z().rotate().rotate(), z());
            assert_eq!(l().rotate(), Block { width: 2, height: 3, cells: "#.#.##".to_string() });
            assert_eq!(l().rotate().rotate(), Block { width: 3, height: 2, cells: "..####".to_string() });
            assert_eq!(l().rotate().rotate().rotate(), Block { width: 2, height: 3, cells: "##.#.#".to_string() });
            assert_eq!(l().rotate().rotate().rotate().rotate(), l());
            assert_eq!(j().rotate(), Block { width: 2, height: 3, cells: "###.#.".to_string() });
            assert_eq!(j().rotate().rotate(), Block { width: 3, height: 2, cells: "#..###".to_string() });
            assert_eq!(j().rotate().rotate().rotate(), Block { width: 2, height: 3, cells: ".#.###".to_string() });
            assert_eq!(j().rotate().rotate().rotate().rotate(), j());
            assert_eq!(t().rotate(), Block { width: 2, height: 3, cells: "#.###.".to_string() });
            assert_eq!(t().rotate().rotate(), Block { width: 3, height: 2, cells: ".#.###".to_string() });
            assert_eq!(t().rotate().rotate().rotate(), Block { width: 2, height: 3, cells: ".###.#".to_string() });
            assert_eq!(t().rotate().rotate().rotate().rotate(), t());
        }

    }

}

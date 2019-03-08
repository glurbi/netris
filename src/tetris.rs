extern crate rand;

mod tetris {

    use std::fmt;
    use rand::prelude::*;
    use rand::seq::SliceRandom;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Block {
        width: usize,
        height: usize,
        cells: Vec<u8>,
    }

    #[derive(Debug)]
    pub struct Board {
        width: usize,
        height: usize,
        cells: Vec<u8>,
        score: usize,
        block_pos: (usize, usize),
        block: Block,
        blocks: Vec<Block>,
        rng: ThreadRng,
    }

    impl Block {

        fn rotate(&self) -> Block {
            let mut cells = Vec::with_capacity(self.width*self.height);
            for i in (0..self.width).rev() {
                for j in 0..self.height {
                    cells.push(self.cells[j*self.width+i]);
                }
            }
            Block { width: self.height, height: self.width, cells }
        }

        fn cell(&self, i: usize, j: usize) -> char {
            self.cells[j*self.width + i] as char
        }
    }

    fn none() -> Block {
        Block { width: 0, height: 0, cells: "".as_bytes().to_vec() }
    }

    fn o() -> Block {
        Block { width: 2, height: 2, cells: "####".as_bytes().to_vec() }
    }

    fn i() -> Block {
        Block { width: 1, height: 4, cells: "####".as_bytes().to_vec() }
    }

    fn s() -> Block {
        Block { width: 3, height: 2, cells: ".####.".as_bytes().to_vec() }
    }

    fn z() -> Block {
        Block { width: 3, height: 2, cells: "##..##".as_bytes().to_vec() }
    }

    fn l() -> Block {
        Block { width: 3, height: 2, cells: "####..".as_bytes().to_vec() }
    }

    fn j() -> Block {
        Block { width: 3, height: 2, cells: "###..#".as_bytes().to_vec() }
    }

    fn t() -> Block {
        Block { width: 3, height: 2, cells: "###.#.".as_bytes().to_vec() }
    }

    fn default_blocks() -> Vec<Block> {
        vec![o(), i(), s(), z(), l(), j(), t()]
    }

    impl Board {

        pub fn new(width: usize, height: usize) -> Board {
            let cells = vec!['.' as u8; width*height];
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

        pub fn move_right(&mut self) -> bool {
            let new_block_pos = (self.block_pos.0 + 1, self.block_pos.1);
            if self.collide(&self.block, new_block_pos) {
                return false;
            }
            self.block_pos = new_block_pos;
            true
        }

        pub fn move_left(&mut self) -> bool {
            if self.block_pos.0 == 0 {
                return false;
            }
            let new_block_pos = (self.block_pos.0 - 1, self.block_pos.1);
            if self.collide(&self.block, new_block_pos) {
                return false;
            }
            self.block_pos = new_block_pos;
            true
        }

        pub fn move_down(&mut self) -> bool {
            let new_block_pos = (self.block_pos.0, self.block_pos.1 + 1);
            if self.collide(&self.block, new_block_pos) {
                return false;
            }
            self.block_pos = new_block_pos;
            true
        }

        pub fn rotate(&mut self) -> bool {
            let new_block = self.block.rotate();
            if self.collide(&new_block, self.block_pos) {
                return false;
            }
            self.block = new_block;
            true
        } 

        fn spawn_block(&mut self) {
            self.block = self.blocks.choose(&mut self.rng).unwrap().clone();
            self.block_pos = (self.width/2, 0);
        }

        fn settle_block(&mut self) {
            for j in 0..self.block.height {
                for i in 0..self.block.width {
                    if self.block.cell(i,j) == '#' {
                        let offset = (j+self.block_pos.1)*self.width + (i + self.block_pos.0);
                        self.cells[offset] = '#' as u8;
                    }
                }
            }
        }

        fn cell(&self, (i, j): (usize, usize)) -> char {
            self.cells[j*self.width + i] as char
        }

        fn collide(&self, block: &Block, pos: (usize,usize)) -> bool {
            if pos.0 + block.width > self.width {
                return true;
            }
            if pos.1 + block.height > self.height {
                return true;
            }
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
            let mut s = String::with_capacity((self.width+1)*self.height);
            for j in 0..self.height {
                let slice = &self.cells[j*self.width..j*self.width+self.width];
                s.push_str(std::str::from_utf8(slice).unwrap());
                s.push('\n');
            }
            write!(f, "{}", s)
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_move_block_i() {
            let mut board = Board::new(3, 6);
            assert_eq!(board.cells.len(), 3 * 6);
            board.block = i();
            assert!(board.move_right());
            assert!(board.move_right());
            assert!(!board.move_right());
            assert!(board.move_left());
            assert!(board.move_left());
            assert!(!board.move_left());
            assert!(board.move_down());
            assert!(board.move_down());
            assert!(!board.move_down());
        }

        #[test]
        fn test_move_block_o() {
            let mut board = Board::new(3, 6);
            assert_eq!(board.cells.len(), 3 * 6);
            board.block = o();
            assert!(board.move_right());
            assert!(!board.move_right());
            assert!(board.move_left());
            assert!(!board.move_left());
            assert!(board.move_down());
            assert!(board.move_down());
            assert!(board.move_down());
            assert!(board.move_down());
            assert!(!board.move_down());
        }

        #[test]
        fn test_move_block_t() {
            let mut board = Board::new(3, 6);
            assert_eq!(board.cells.len(), 3 * 6);
            board.block = t();
            assert!(!board.move_right());
            assert!(!board.move_left());
            assert!(board.move_down());
            assert!(board.move_down());
            assert!(board.move_down());
            assert!(board.move_down());
            assert!(!board.move_down());
        }

        #[test]
        fn test_rotate_t() {
            let mut board = Board::new(5, 5);
            board.block = t();
            assert!(board.move_right());
            assert!(board.move_down());
            assert!(board.rotate());
            assert!(board.move_right());
            assert!(board.move_right());
            assert!(!board.rotate());
            assert!(board.move_left());
            assert!(board.rotate());
            assert!(board.move_down());
            assert!(board.move_down());
            assert!(!board.rotate());
            board.settle_block();
            println!("{}", board);
        }

        #[test]
        fn test_rotate_block() {
            assert_eq!(o().rotate(), o());
            assert_eq!(i().rotate(), Block { width: 4, height: 1, cells: "####".as_bytes().to_vec() });
            assert_eq!(i().rotate().rotate(), i());
            assert_eq!(s().rotate(), Block { width: 2, height: 3, cells: "#.##.#".as_bytes().to_vec() });
            assert_eq!(s().rotate().rotate(), s());
            assert_eq!(z().rotate(), Block { width: 2, height: 3, cells: ".####.".as_bytes().to_vec() });
            assert_eq!(z().rotate().rotate(), z());
            assert_eq!(l().rotate(), Block { width: 2, height: 3, cells: "#.#.##".as_bytes().to_vec() });
            assert_eq!(l().rotate().rotate(), Block { width: 3, height: 2, cells: "..####".as_bytes().to_vec() });
            assert_eq!(l().rotate().rotate().rotate(), Block { width: 2, height: 3, cells: "##.#.#".as_bytes().to_vec() });
            assert_eq!(l().rotate().rotate().rotate().rotate(), l());
            assert_eq!(j().rotate(), Block { width: 2, height: 3, cells: "###.#.".as_bytes().to_vec() });
            assert_eq!(j().rotate().rotate(), Block { width: 3, height: 2, cells: "#..###".as_bytes().to_vec() });
            assert_eq!(j().rotate().rotate().rotate(), Block { width: 2, height: 3, cells: ".#.###".as_bytes().to_vec() });
            assert_eq!(j().rotate().rotate().rotate().rotate(), j());
            assert_eq!(t().rotate(), Block { width: 2, height: 3, cells: "#.###.".as_bytes().to_vec() });
            assert_eq!(t().rotate().rotate(), Block { width: 3, height: 2, cells: ".#.###".as_bytes().to_vec() });
            assert_eq!(t().rotate().rotate().rotate(), Block { width: 2, height: 3, cells: ".###.#".as_bytes().to_vec() });
            assert_eq!(t().rotate().rotate().rotate().rotate(), t());
        }

    }

}

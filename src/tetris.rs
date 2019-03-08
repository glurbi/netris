extern crate rand;

mod tetris {

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
        cells: Vec<bool>,
        score: usize,
        block: Option<Block>,
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
            let cells = vec![false; width * height];
            let rng = rand::thread_rng();
            Board {
                width,
                height,
                cells,
                score: 0,
                block: None,
                blocks: default_blocks(),
                rng,
            }
        }

        pub fn step(&mut self) {

        }

        pub fn current_block(&self) -> Option<&Block> {
            self.block.as_ref()
        }

        fn pick_block(&mut self) -> Option<Block> {
            Some(self.blocks.choose(&mut self.rng).unwrap().clone())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_board() {
            let mut board = Board::new(10, 20);
            assert_eq!(board.cells.len(), 200);
            assert_eq!(board.current_block(), None);
            board.block = board.pick_block();
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

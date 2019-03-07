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

    fn O() -> Block {
        Block { width: 2, height: 2, cells: "####".to_string() }
    }

    fn I() -> Block {
        Block { width: 1, height: 4, cells: "####".to_string() }
    }

    fn S() -> Block {
        Block { width: 3, height: 2, cells: ".####.".to_string() }
    }

    fn Z() -> Block {
        Block { width: 3, height: 2, cells: "##..##.".to_string() }
    }

    fn L() -> Block {
        Block { width: 3, height: 2, cells: "####..".to_string() }
    }

    fn default_blocks() -> Vec<Block> {
        vec![O(), I(), S(), Z()]
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
            assert_eq!(O().rotate(), O());
            assert_eq!(I().rotate(), Block { width: 4, height: 1, cells: "####".to_string() });
            assert_eq!(I().rotate().rotate(), I());
            assert_eq!(S().rotate(), Block { width: 2, height: 3, cells: "#.##.#".to_string() });
            assert_eq!(S().rotate().rotate(), S());
            assert_eq!(Z().rotate(), Block { width: 2, height: 3, cells: ".####.".to_string() });
            assert_eq!(Z().rotate().rotate(), Z());
            assert_eq!(L().rotate(), Block { width: 2, height: 3, cells: "#.#.##".to_string() });
        }

    }

}

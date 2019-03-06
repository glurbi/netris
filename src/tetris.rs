extern crate rand;

mod tetris {

    use rand::prelude::*;
    use rand::seq::SliceRandom;

    #[derive(Debug)]
    pub enum RotateDir { Left, Right }
    use RotateDir::*;

    #[derive(Debug)]
    pub struct Block {
        width: usize,
        height: usize,
        cells: String,
    }

    #[derive(Debug)]
    pub struct Board<'a> {
        width: usize,
        height: usize,
        cells: Vec<bool>,
        score: usize,
        block: Option<&'a Block>,
        blocks: Vec<Block>,
        rng: ThreadRng,
    }

    impl<'a> Block {
        fn rotate(&self, dir: RotateDir) -> Block {
            Block { width: 2, height: 2, cells: "####".to_string() }
        }
    }

    fn default_blocks<'a>() -> Vec<Block> {
        let O = Block { width: 2, height: 2, cells: "####".to_string() };
        vec![O]
    }

    impl<'a> Board<'a> {

        pub fn new(width: usize, height: usize) -> Board<'a> {
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

        fn pick_block(&mut self) {
            self.block = Some(self.blocks.choose(&mut self.rng).unwrap());
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_init() {
            let board = Board::new(10, 20);
            assert_eq!(board.cells.len(), 200);
        }
    }

}

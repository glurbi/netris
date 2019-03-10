use std::thread;
use std::sync::mpsc::{channel,Sender};
use rand::prelude::{ThreadRng,thread_rng};
use crate::board::Board;
use std::sync::Arc;

#[derive(Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveDown,
    Land,
    Rotate,
}

pub struct Game {
    board: Board,
}

pub struct GameMediator {
    tx: Sender<Action>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> GameMediator {
        let (tx, rx) = channel();
        let board = Board::new(width, height);

        let mut game = Game {
            board,
        };

        thread::spawn(move || {
            let rng = thread_rng();
            let mut iter = rx.iter();
            for a in rx {
                game.handle_action(&a);
            }
        });

        GameMediator {
            tx,
        }
    }

    fn handle_action(&mut self, a: &Action) {
        use Action::*;
        match a {
            MoveLeft => self.board.move_left(),
            MoveRight => self.board.move_right(),
            MoveDown => self.board.move_down(),
            Land => self.board.land(),
            Rotate => self.board.rotate(),
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_game() {
        let mut game = Game::new(10, 20);
        game.tx.send(Action::MoveLeft);
    }
}

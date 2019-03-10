use std::thread;
use std::sync::mpsc::{channel,Sender};
use crate::board::Board;
use Action::*;

#[derive(Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveDown,
    Land,
    Rotate,
}

pub struct Game {
    tx: Sender<Action>,
    board: Board,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        let (tx, rx) = channel();
        let board = Board::new(width, height);
        thread::spawn(move || {
            let mut iter = rx.iter();
            loop {
                match iter.next() {
                    Some(action) => (),
                    None => break,
                }
            }
        });

        Game {
            tx,
            board,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_game() {
        let mut game = Game::new(10, 20);
        game.tx.send(MoveLeft);
    }
}

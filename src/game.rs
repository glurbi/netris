use rand::prelude::*;
use crate::board::Board;
use actix::prelude::*;

#[derive(Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveDown,
    Land,
    Rotate,
    Stop,
}

impl Message for Action {
    type Result = ();
}

#[derive(Debug)]
pub struct Game {
    board: Board,
}

impl Actor for Game {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
       println!("I am alive!");
    }
}

impl Handler<Action> for Game {
    type Result = ();

    fn handle(&mut self, a: Action, ctx: &mut Context<Self>) -> Self::Result {
        println!("handling {:?}", &a);
        use Action::*;
        let _ = match a {
            MoveLeft => self.board.move_left(),
            MoveRight => self.board.move_right(),
            MoveDown => self.board.move_down(),
            Land => self.board.land(),
            Rotate => self.board.rotate(),
            Stop => { System::current().stop(); Ok(()) },
        };
    }
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            board: Board::new(width, height),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_game() {
        let system = System::new("test");
        let game = Game::new(10, 20);
        let addr = game.start();
        println!("{:?}", addr);
        addr.do_send(Action::MoveLeft);
        println!("AAA");
        addr.do_send(Action::Stop);
        println!("BBB");
        system.run();
    }
}

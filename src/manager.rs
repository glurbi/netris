use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;
use crate::game::Game;

type GameId = Uuid;

struct Manager {
    games: HashMap<GameId, Rc<RefCell<Game>>>,
}

impl Manager {

    fn new() -> Manager {
        Manager {
            games: HashMap::new(),
        }
    }

    fn create_game(&mut self) -> Result<GameId, String> {
        let uuid = Uuid::new_v4();
        Ok(uuid)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_single_player_game() {
        let mut manager = Manager::new();
        let res = manager.create_game();
        assert!(res.is_ok());
    }
}

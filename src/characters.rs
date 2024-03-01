use std::sync::{Arc, Weak};
use tokio::sync::Mutex;

use crate::{
    game::Game,
    player::Player,
};

pub type Num = usize;

pub struct Character {
    pub num: Num,
    player: Weak<Mutex<Player>>,
    game: Weak<Mutex<Game>>,
    role: Role,
}

impl Character {
    pub fn new(num: Num, player: Weak<Mutex<Player>>, role: Role, game: Weak<Mutex<Game>>) -> Self {
        Character { player, role, num, game }
    }

    pub fn set_game(&mut self, game: Weak<Mutex<Game>>) {
        self.game = game;
    }

    pub fn get_game(&self) -> Arc<Mutex<Game>> { 
        self.game.upgrade().unwrap()
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Role {
    Civilian,
    Mafia,
    Sheriff,
    Maniac,
}

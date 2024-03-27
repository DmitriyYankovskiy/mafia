use std::sync::{Arc, Weak};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

use super::{
    Game,
    player::Player,
    super::role::Role,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Num(pub usize);
impl Num {
    pub fn from_idx(num: usize) -> Self {
        Num(num + 1)
    }
    pub fn to_idx(&self) -> usize {
        self.0 - 1
    }
    pub fn next(&self) -> Self {
        Num(self.0 + 1)
    }
    pub fn first() -> Self {
        Num(1)
    }
}

pub struct Character {
    pub num: Num,
    player: Weak<Mutex<Player>>,
    game: Weak<Mutex<Game>>,
    pub role: Role,
    pub alive: bool,
}

impl Character {
    pub fn new(num: Num, player: Weak<Mutex<Player>>, role: Role, game: Weak<Mutex<Game>>) -> Self {
        Character { player, role, num, game, alive: true }
    }

    pub fn set_game(&mut self, game: Weak<Mutex<Game>>) {
        self.game = game;
    }

    pub fn get_game(&self) -> Arc<Mutex<Game>> { 
        self.game.upgrade().unwrap()
    }

    pub fn get_player(&self) -> Arc<Mutex<Player>> {
        self.player.upgrade().unwrap()
    }

    pub fn die(&mut self) {
        self.alive = false;
    }
}

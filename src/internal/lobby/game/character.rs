use std::sync::{Arc, Weak};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

use super::{
    Game,
    role::Role,
    message::{incom, outgo},
    super::player::Player,
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

#[derive(Debug)]
pub struct CharacterInfo {
    pub num: Num,
    pub role: Role,
    pub alive: bool,
}

pub struct Character {
    pub info: Mutex<CharacterInfo>,
    player: Arc<Player>,
    game: Mutex<Weak<Game>>,
}

impl Character {
    pub fn new(num: Num, player: Arc<Player>, role: Role, game: Weak<Game>) -> Self {
        Character { player, info: Mutex::new(CharacterInfo{role, num, alive: true}), game: Mutex::new(game)}
    }

    pub async fn set_game(&self, game: Weak<Game>) {
        *self.game.lock().await = game;
    }

    pub async fn get_game(&self) -> Arc<Game> { 
        self.game.lock().await.upgrade().unwrap()
    }

    pub fn get_player(&self) -> Arc<Player> {
        Arc::clone(&self.player)
    }

    pub async fn die(&self) {
        self.info.lock().await.alive = false;
    }


    pub async fn send(&self, m: outgo::M) {
        self.player.send(super::super::outgo::M::Game(m));
    }
}

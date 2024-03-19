use std::{
    collections::HashMap,
    sync::{Arc, Weak}
};
use tokio::sync::Mutex;
use rand::{
    thread_rng,
    seq::SliceRandom,
};

pub mod setup;
pub mod game;
pub mod role;

use setup::Setup;
use game::{
    Game,
    player::Player,
    character::{Character, Num},
};

#[derive(Clone)]
pub enum GameState {
    Setup(Arc<Mutex<Setup>>),
    On(Arc<Mutex<Game>>),
    End,
}

impl GameState {
    pub fn new() -> Self {
        GameState::Setup(Arc::new(Mutex::new(Setup::new())))
    }
    
    pub async fn start(mut self) -> Result<(), &'static str> {
        if let GameState::Setup(mut setup_arc) = self {
            let mut setup = setup_arc.lock().await;
            let mut rng = thread_rng();

            let mut players: HashMap<String, Arc<Mutex<Player>>> = setup.players.clone().into_iter().map(|(k, v)| (k, Arc::new(Mutex::new(v)))).collect();
            let mut roles = setup.get_roles().await;
            let mut characters = Vec::<Character>::new();

            let mut players_vec: Vec<Arc<Mutex<Player>>> = players.clone().into_iter().map(|(k, v)| Arc::clone(&v)).collect();
            players_vec.shuffle(&mut rng);
            roles.shuffle(&mut rng);

            for i in 0..(players_vec.len()) {
                characters.push(Character::new(Num::from_idx(i), Arc::downgrade(&players_vec[i]), roles[i], Weak::new()));
            }

            let mut game = Arc::new(Mutex::new(Game::new(characters.into_iter().map(Mutex::new).map(Arc::new).collect(), players)));

            for i in 0..(players_vec.len()) {
                game.lock().await.get_character(Num::from_idx(i)).lock().await.set_game(Arc::downgrade(&Arc::clone(&game)));
            }   

            self = GameState::On(game);

            Ok(())
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }
}
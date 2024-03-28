use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
use tokio::{
    sync::Mutex, task::JoinHandle,

};
use rand::seq::SliceRandom;

extern crate rand_pcg;

pub mod setup;
pub mod game;
pub mod role;

use rand_pcg::Pcg32;
use setup::Setup;
use game::{
    Game,
    player::Player,
    character::{Character, Num},
};

use crate::game_state::game::character;

#[derive(Clone)]
pub enum GameState {
    Setup(Setup),
    On{game: Arc<Mutex<Game>>/*, game_loop: Arc<Mutex<JoinHandle<()>>>*/},
    End,
}

fn time_now() -> u128 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros()
}

impl GameState {
    pub fn new() -> Self {
        
        GameState::Setup(Setup::new())
    }
    
    pub async fn start(&mut self) -> Result<JoinHandle<()>, &'static str> {
        if let GameState::Setup(mut setup) = self.clone() {
            let mut rng = Pcg32::new(time_now() as u64, time_now() as u64);

            let players: HashMap<String, Arc<Mutex<Player>>> = setup.players.clone().into_iter().map(|(k, v)| (k, Arc::new(Mutex::new(v)))).collect();
            let mut roles = setup.get_roles().await;
            let mut characters = Vec::<Character>::new();

            let mut players_vec: Vec<Arc<Mutex<Player>>> = players.clone().into_iter().map(|(_, v)| Arc::clone(&v)).collect();
            players_vec.shuffle(&mut rng);
            roles.shuffle(&mut rng);

            for i in 0..(players_vec.len()) {
                characters.push(Character::new(Num::from_idx(i), Arc::downgrade(&players_vec[i]), roles[i], Weak::new()));
            }
            
            let mut ind = 0usize;

            let characters: Vec<Arc<Mutex<Character>>> = characters.into_iter().map(Mutex::new).map(Arc::new).collect();
            for i in 0..(players_vec.len()) {
                players_vec[i].lock().await.character = Arc::downgrade(&characters[i])
            }

            let game = Arc::new(Mutex::new(Game::new(characters, players)));

            for i in 0..(players_vec.len()) {
                game.lock().await.get_character(Num::from_idx(i)).lock().await.set_game(Arc::downgrade(&Arc::clone(&game)));
            }   
            let task = tokio::spawn(Game::run(Arc::clone(&game)));
            println!("<game started>");
            *self = GameState::On{game};
            Ok(task)
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }
}
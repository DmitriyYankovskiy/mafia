use std::{
    collections::HashMap, future::Future, sync::{Arc, Weak}
};
use tokio::sync::Mutex;
use rand::seq::SliceRandom;

extern crate rand_pcg;

pub mod game;
pub mod player;

use rand_pcg::Pcg32;
use {
    game::{
        Game,
        character::{Character, Num},
    },
    player::Player,
};

pub struct Lobby {
    pub players: Mutex<HashMap<String, Arc<Player>>>,
    pub state: Mutex<State>,
}

#[derive(Clone)]
pub enum State {
    Setup,
    On{game: Arc<Game>},
    End,
}

fn time_now() -> u128 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros()
}

impl Lobby {
    pub fn new() -> Self {        
        Self {
            players: Mutex::new(HashMap::new()),
            state: Mutex::new(State::Setup),
        }
    }
    
    pub async fn start(&self) -> Result<impl Future<Output = ()>, &'static str> {
        let state = {
            self.state.lock().await.clone()
        };
        if let State::Setup = state {
            let mut rng = Pcg32::new(time_now() as u64, time_now() as u64);

            let mut roles = game::role::get_roles(self.players.lock().await.len()).await;
            let mut characters = Vec::<Character>::new();

            let mut players_vec: Vec<Arc<Player>> = self.players.lock().await.clone().into_iter().map(|(_, v)|v.clone()).collect();
            players_vec.shuffle(&mut rng);
            roles.shuffle(&mut rng);

            for i in 0..(players_vec.len()) {
                characters.push(Character::new(Num::from_idx(i), players_vec[i].clone(), roles[i], Weak::new()));
            }
            
            let characters: Vec<Arc<Character>> = characters.into_iter().map(Arc::new).collect();
            for i in 0..(players_vec.len()) {
                *players_vec[i].character.lock().await = Arc::downgrade(&characters[i]);
            }

            let game = Arc::new(Game::new(characters));

            for i in 0..(players_vec.len()) {
                game.get_character(Num::from_idx(i)).set_game(Arc::downgrade(&Arc::clone(&game))).await;
            }   
            println!("<game started>");
            *self.state.lock().await = State::On{game: Arc::clone(&game)};
            Ok(Game::run(game))
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }

    pub async fn add_player(&self, player: Arc<Player>) -> Result<usize, &str> {
        println!("  new character with name: {}", &player.info.lock().await.name);
        let name = {player.info.lock().await.name.clone()};
        self.players.lock().await.insert(name, player);
        Ok(self.players.lock().await.len())
    }
}
use std::{cell::RefCell, collections::{HashMap, HashSet}, hash, sync::{Arc, Weak}};
use serde::Deserialize;
use tokio::sync::{mpsc::{self, Receiver, Sender}, Mutex};

use crate::{
    characters::{Character, Role},
    game_loop::GameLoop,
};
use rand::{thread_rng, seq::SliceRandom};

pub enum GameState {
    Setup(Setup),
    On(Game),
    End,
}

impl GameState {
    pub fn new() -> GameState {
        let mut roles = HashMap::<Role, usize>::new();
        roles.insert(Role::Civilian, 0);
        roles.insert(Role::Sheriff, 0);
        roles.insert(Role::Maniac, 0);
        roles.insert(Role::Mafia, 0);
        GameState::Setup(Setup {
            players: HashMap::new(),
            roles,
        })
    }

    pub fn start(mut self) -> Result<(), &'static str> {
        if let GameState::Setup(mut setup) = self {
            let mut rng = thread_rng();

            let mut players: HashMap<String, Arc<Mutex<Player>>> = setup.players.clone().into_iter().map(|(k, v)| (k, Arc::new(Mutex::new(v)))).collect();
            let mut roles = setup.get_roles();

            let mut players_vec: Vec<Arc<Mutex<Player>>> = players.clone().into_iter().map(|(k, v)| v.clone()).collect();
            players_vec.shuffle(&mut rng);
            roles.shuffle(&mut rng);

            let mut characters = Vec::<Character>::new();

            for i in 0..(players_vec.len()) {
                characters.push(Character::new(i, Arc::downgrade(&players_vec[i]), roles[i]));
            }

            self = GameState::On(Game { characters: characters.into_iter().map(Mutex::new).map(Arc::new).collect(), players, game_loop: GameLoop::new() });

            Ok(())
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }

    pub fn add_player(&mut self, player: Player) -> Result<usize, &str> {
        if let GameState::Setup(setup) = self {
            Ok(setup.add_player(player))
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }
}

pub struct Setup {
    roles: HashMap<Role, usize>,
    players: HashMap<String, Player>,
}
impl Setup {
    pub fn add_player(&mut self, player: Player) -> usize {
        self.players.insert(player.name.clone(), player);
        *self.roles.get_mut(&Role::Civilian).unwrap() += 1;
        self.players.len()
    }

    pub fn get_roles(&mut self) -> Vec<Role> {
        let mut roles = Vec::<Role>::new();
        for (role, cnt) in &self.roles {
            roles.append(&mut vec![*role; *cnt]);
        }
        roles
    }
}

pub struct Game {
    characters: Vec<Arc<Mutex<Character>>>,
    players: HashMap<String, Arc<Mutex<Player>>>,
    game_loop: GameLoop,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub ws_tx: Sender<String>,
    pub ws_rx: Arc<Mutex<Receiver<String>>>,
    pub character: Weak<Character>,
}

impl Player {
    pub fn new(name: String, ws_tx: Sender<String>, ws_rx: Arc<Mutex<Receiver<String>>>) -> Self {
        Player { name, character: Weak::new(), ws_tx, ws_rx}
    }
}

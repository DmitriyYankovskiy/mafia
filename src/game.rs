use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub enum GameState {
    Setup(Setup),
    On(Game),
    End,
}

impl GameState {
    pub fn new() -> GameState {
        GameState::Setup(Setup {
            players: HashSet::<Player>::new(),
        })
    }

    pub fn start(mut self) -> Result<(), &'static str> {
        if let GameState::Setup(setup) = self {
            let mut players: Vec<Player> = setup.players.into_iter().collect();
            let mut rng = thread_rng();
            players.shuffle(&mut rng);

            let characters = Vec::<Character>::new();
            let roles = setup.get_roles();
            for i in 0..(players.len()) {
                characters.push(Character {
                    num: i,
                    player: players[i];
                    role: roles[i];
                });
            }

            self = GameState::On(Game {
                players: players,
            });

            Ok(())
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }

    pub fn add_player(&mut self, player: Player) -> Result<usize, &str>{
        if let GameState::Setup(setup) = self {                
            Ok(setup.add_player(player))
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }
}



pub struct Setup {
    players: HashSet<Player>,
}
impl Setup {
    fn add_player(&mut self, player: Player) -> usize {
        self.players.insert(player);
        self.players.len()
    }

    fn get_roles(&mut self) -> Vec<Role> {
       let mut roles = Vec::<Role>::new();
    }
}



pub struct Game {
    players: Vec<Character>,
}



pub struct Character {
    player: Player,
    role: Role,
    num: usize,
}



pub enum Role {
    Civilian,
    Mafia,
    Sheriff,
    Maniac,
}



#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Player {
    name: String,
    num: usize,
}

impl Player {
    fn new(name: String, num: usize) -> Self {
        Player {
            name,
            num,
        }
    }
}
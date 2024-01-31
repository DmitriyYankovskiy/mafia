use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use characters::{Role, Character};

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
            players: HashSet::<Player>::new(),
            roles,
        })
    }

    pub fn start(mut self) -> Result<(), &'static str> {
        if let GameState::Setup(mut setup) = self {
            let mut rng = thread_rng();
            
            let mut players: Vec<Player> = setup.players.clone().into_iter().collect();
            let mut roles = setup.get_roles();
            players.shuffle(&mut rng);
            roles.shuffle(&mut rng);
            
            let mut characters = Vec::<Character>::new();
            
            for i in 0..(players.len()) {
                characters.push(Character {
                    num: i,
                    player: players[i].clone(),
                    role: roles[i],
                });
            }

            self = GameState::On(Game {
                characters,
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
    roles: HashMap<Role, usize>,
    players: HashSet<Player>,
}
impl Setup {
    fn add_player(&mut self, player: Player) -> usize {
        self.players.insert(player);
        *self.roles.get_mut(&Role::Civilian).unwrap() += 1;
        self.players.len()
    }

    fn get_roles(&mut self) -> Vec<Role> {
        let mut roles = Vec::<Role>::new();
        for (role, cnt) in &self.roles {
            roles.append(&mut  vec![*role; *cnt]);
        }
        roles
    }
}



pub struct Game {
    characters: Vec<Character>,
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
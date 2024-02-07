use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash;
use std::rc::{Rc, Weak};

use crate::characters::{Character, Role};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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

            let mut players: HashMap<String, Rc<RefCell<Player>>> = setup.players.clone().into_iter().map(|(k, v)| (k, Rc::new(RefCell::new(v)))).collect();
            let mut roles = setup.get_roles();

            let mut players_vec: Vec<Rc<RefCell<Player>>> = players.clone().into_iter().map(|(k, v)| v.clone()).collect();
            players_vec.shuffle(&mut rng);
            roles.shuffle(&mut rng);

            let mut characters = Vec::<Character>::new();

            for i in 0..(players_vec.len()) {
                characters.push(Character::new(i, Rc::downgrade(&players_vec[i]), roles[i]));
            }

            self = GameState::On(Game { characters: characters.into_iter().map(RefCell::new).map(Rc::new).collect(), players});

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
    fn add_player(&mut self, player: Player) -> usize {
        self.players.insert(player.name.clone(), player);
        *self.roles.get_mut(&Role::Civilian).unwrap() += 1;
        self.players.len()
    }

    fn get_roles(&mut self) -> Vec<Role> {
        let mut roles = Vec::<Role>::new();
        for (role, cnt) in &self.roles {
            roles.append(&mut vec![*role; *cnt]);
        }
        roles
    }
}

pub struct Game {
    characters: Vec<Rc<RefCell<Character>>>,
    players: HashMap<String, Rc<RefCell<Player>>>,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    character: Weak<Character>,
}

impl Player {
    fn new(name: String, num: usize) -> Self {
        Player { name, character: Weak::new() }
    }
}

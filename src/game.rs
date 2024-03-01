use std::{collections::HashMap, sync::{Arc, Weak}};

use tokio::{
    fs::File, 
    io::AsyncReadExt, 
    sync::Mutex
};

use crate::{
    characters::{Character, Role, Num},
    game_loop::{GameLoop, Stage},
    player::Player,
};
use rand::{thread_rng, seq::SliceRandom};

#[derive(Clone)]
pub enum GameState {
    Setup(Arc<Mutex<Setup>>),
    On(Arc<Mutex<Game>>),
    End,
}

impl GameState {
    pub fn new() -> GameState {
        let mut roles = HashMap::<Role, usize>::new();
        roles.insert(Role::Civilian, 0);
        roles.insert(Role::Sheriff, 0);
        roles.insert(Role::Maniac, 0);
        roles.insert(Role::Mafia, 0);
        GameState::Setup(Arc::new(Mutex::new(Setup {
            players: HashMap::new(),
        })))
    }

    pub async fn start(mut self) -> Result<(), &'static str> {
        if let GameState::Setup(mut setup_arc) = self {
            let mut setup = setup_arc.lock().await;
            let mut rng = thread_rng();

            let mut players: HashMap<String, Arc<Mutex<Player>>> = setup.players.clone().into_iter().map(|(k, v)| (k, Arc::new(Mutex::new(v)))).collect();
            let mut roles = setup.get_roles().await;
            let mut characters = Vec::<Character>::new();

            let mut players_vec: Vec<Arc<Mutex<Player>>> = players.clone().into_iter().map(|(k, v)| v.clone()).collect();
            players_vec.shuffle(&mut rng);
            roles.shuffle(&mut rng);

            for i in 0..(players_vec.len()) {
                characters.push(Character::new(i, Arc::downgrade(&players_vec[i]), roles[i], Weak::new()));
            }

            let mut game = Arc::new(Mutex::new(Game { characters: characters.into_iter().map(Mutex::new).map(Arc::new).collect(), players, game_loop: GameLoop::new() }));

            for i in 0..(players_vec.len()) {
                game.lock().await.get_character(i).lock().await.set_game(Arc::downgrade(&game.clone()));
            }   

            self = GameState::On(game);

            Ok(())
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }
}


mod role {
    use serde::{Serialize, Deserialize};
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct RoleSet {
        pub mafia: usize,
        pub sheriff: bool,
        pub maniac: bool,
    }
}

pub struct Setup {
    players: HashMap<String, Player>,
}
impl Setup {
    pub async fn add_player(&mut self, player: Player) -> Result<usize, &str> {
        self.players.insert(player.name.clone(), player);
        Ok(self.players.len())
    }

    pub async fn get_roles(&mut self) -> Vec<Role> {
        let mut role_set = String::new();
        File::open("../rules/roles.json").await.unwrap().read_to_string(&mut role_set);
        let role_set = serde_json::from_str::<HashMap<usize, role::RoleSet>>(&role_set).unwrap();
        let role_set = role_set[&(self.players.len() - 1)];

        let mut roles = Vec::<Role>::new();
        for i in 0..role_set.mafia {
            roles.push(Role::Mafia);
        }
        if role_set.sheriff {
            roles.push(Role::Sheriff);
        }
        if role_set.maniac {
            roles.push(Role::Maniac);
        }
        while roles.len() < self.players.len() {
            roles.push(Role::Civilian);
        }

        roles
    }
}

pub struct Game {
    characters: Vec<Arc<Mutex<Character>>>,
    players: HashMap<String, Arc<Mutex<Player>>>,
    game_loop: GameLoop,
}

impl Game {
    pub async fn mafia_kill(&mut self, aftor: Num, target: Num) {
        if let Stage::Night {mafia_targets, maniac_target, checked} = &mut self.game_loop.stage {
            
        }
    }

    pub async fn maniac_kill(&mut self, aftor: Num, target: Num) {

    }

    pub fn vote(&mut self, aftor: Num, target: Num) {

    }

    pub fn accuse(&mut self, aftor: Num, target: Num) {

    }

    pub fn get_character(&self, num: Num) -> Arc<Mutex<Character>> {
        self.characters[num].clone()
    }
}

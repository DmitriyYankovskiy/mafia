use std::{
    collections::HashMap,
    fs::File,
    io::Read,
};
use super::{
    game::player::Player,
    role::{Role, RoleSet},
};

#[derive(Clone)]
pub struct Setup {
    pub players: HashMap<String, Player>,
}
impl Setup {
    pub fn new() -> Setup {
        Setup {
            players: HashMap::new(),
        }
    }

    pub async fn add_player(&mut self, player: Player) -> Result<usize, &str> {
        self.players.insert(player.name.clone(), player);
        Ok(self.players.len())
    }

    pub async fn get_roles(&mut self) -> Vec<Role> {
        let mut role_set = String::new();
        File::open("../rules/roles.json").unwrap().read_to_string(&mut role_set).unwrap();
        let role_set = serde_json::from_str::<HashMap<usize, RoleSet>>(&role_set).unwrap();
        let role_set = role_set[&(self.players.len() - 1)];

        let mut roles = Vec::<Role>::new();
        for _ in 0..role_set.mafia {
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
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
};

use serde::{Serialize, Deserialize};
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct RoleSet {
    pub mafia: usize,
    pub sheriff: bool,
    pub don: bool,
    pub civilian: usize,
}

impl RoleSet {
    pub fn new() -> Self {
        Self {
            mafia: 0,
            sheriff: false,
            don: true,
            civilian: 0,
        }
    }

    pub fn cnt_red(&self) -> usize {
        let mut cnt = self.civilian;
        if self.sheriff { cnt += 1; }
        cnt
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, serde::Serialize)]
pub enum Role {
    Civilian,
    Mafia,
    Sheriff,
    Don,
}

impl Role {
    pub fn is_black(&self) -> bool {
        match self {
            Self::Mafia => true,
            Self::Don => true,
            _ => false
        }
    }
}

pub async fn get_roles(cnt: usize) -> Vec<Role> {
    let mut role_set = String::new();
    File::open("./rules/roles.json").unwrap().read_to_string(&mut role_set).unwrap();
    let role_set = serde_json::from_str::<HashMap<usize, RoleSet>>(&role_set).unwrap();
    let role_set = role_set[&cnt];

    let mut roles = Vec::<Role>::new();
    for _ in 0..role_set.mafia {
        roles.push(Role::Mafia);
    }
    if role_set.sheriff {
        roles.push(Role::Sheriff);
    }
    if role_set.don {
        roles.push(Role::Don);
    }
    while roles.len() < cnt {
        roles.push(Role::Civilian);
    }

    roles
}
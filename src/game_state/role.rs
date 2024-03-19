use serde::{Serialize, Deserialize};
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct RoleSet {
    pub mafia: usize,
    pub sheriff: bool,
    pub maniac: bool,
    pub civilian: usize,
}

impl RoleSet {
    pub fn new() -> Self {
        Self {
            mafia: 0,
            sheriff: false,
            maniac: false,
            civilian: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Role {
    Civilian,
    Mafia,
    Sheriff,
    Maniac,
}
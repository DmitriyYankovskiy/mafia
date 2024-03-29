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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
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
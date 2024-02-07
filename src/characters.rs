use std::{cell::RefCell, rc::Weak};

use crate::game::Player;

pub type Num = usize;

pub struct Character {
    num: Num,
    player: Weak<RefCell<Player>>,
    role: Role,
}

impl Character {
    pub fn new(num: Num, player: Weak<RefCell<Player>>, role: Role) -> Self {
        Character { player, role, num }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Role {
    Civilian,
    Mafia,
    Sheriff,
    Maniac,
}

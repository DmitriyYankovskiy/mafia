use std::{collections::{HashMap, HashSet}, sync::Arc};


use tokio::sync::Mutex;

use super::character::Num;

pub struct GameLoop {
    pub stage: Arc<Mutex<Stage>>,
    round: usize,
}
pub enum Stage {
    Night {
        mafia_targets: HashMap<Num, Num>,
        maniac_target: Option<(Num, Num)>,
        checked: Option<(Num, Num)>,  
    },
    Sunrise { 
        idx: usize,
        dies: Vec<Num>,
    },
    Discussion {
        num: Num,
        accused:  HashMap<Num, Num>,
    },
    Voting {
        idx: usize,
        candidates: Vec<Candidate>,
        not_voted: HashSet<Num>,
    },
    Sunset {
        idx: usize,
        dies: Vec<Num>,
    },
}

impl GameLoop {
    pub fn new() -> Self {
        GameLoop {
            stage: Arc::new(Mutex::new(Stage::new())),
            round: 0,
        }
    }


    pub fn get_round(&self) -> usize {
        self.round
    }
}

impl Stage {
    fn new() -> Self {
        Self::Discussion {num: Num(1), accused: HashMap::new()}
    }
}

pub struct Candidate {
    pub num: Num,
    pub cnt_votes: usize,
}

impl Candidate {
    pub fn new(num: Num, cnt_votes: usize) -> Self {
        Self {
            num,
            cnt_votes,
        }
    }
}
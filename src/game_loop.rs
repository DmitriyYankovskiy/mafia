use core::num;
use std::collections::HashSet;

use crate::characters::Num;

struct GameLoop {
    stage: Stage,
    round: usize,
}

impl GameLoop {
    fn new() -> Self {
        GameLoop {
            stage: Stage::new(),
            round: 0,
        }
    }


    fn get_round(&self) -> usize {
        self.round
    }

    fn next(&mut self) {
        let stage = &mut self.stage; 
        match stage {
            Stage::Night {
                time,
                dies,
            } => {
                *stage = Stage::Sunrise { idx: 0, dies: dies };
            },
            Stage::Sunrise {
                idx,
                dies,
            } => {
                *stage = Stage::Discussion {num: self.get_round(), accuses: HashSet::<Num>::new()};
            }
        }
    }
}

pub enum Stage {
    Night {
        time: usize,
        dies: HashSet<Num>,
    },
    Sunrise {
        idx: usize,
        dies: HashSet<Num>,
    },
    Discussion {
        num: Num,
        accuses: HashSet<Num>,
    },
    Voting(Discussion),
    Sunset,
}

impl Stage {
    fn new() -> Self {
        Self::Discussion {num: 0, accuses: HashSet::<Num>::new()}
    }
}

struct Candidate {
    num: Num,
    cnt_vote: usize,
}

impl Candidate {
    pub fn new(num: Num, cnt_vote: usize) -> Self {
        Candidate {
            num,
            cnt_vote,
        }
    }
}

struct Discussion {
    idx: usize,
    candidates: Vec<Candidate>,
}

impl From<HashSet<Num>> for Discussion {
    fn from(value: HashSet<Num>) -> Self {
        Discussion {
            idx: 0,
            candidates: value.into_iter().map(|num: Num| Candidate::new(num, 0)).collect(),
        }
    }
}
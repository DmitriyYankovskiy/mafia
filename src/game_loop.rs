use core::num;
use std::collections::HashSet;

use crate::characters::Num;

struct GameLoop {
    stage: Stage,
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
        HashSet<Num>
    },
    Voting(Discussion),
    Sunset,
}

impl Stage {
    fn init(&mut self) {
        match self {
            Self::Night {time} => (),
            Sunrise()
        }
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
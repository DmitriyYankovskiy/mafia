use std::collections::HashSet;

use crate::characters::{Num, Role};

pub struct GameLoop {
    pub stage: Stage,
    round: usize,
}

pub enum Stage {
    Night {
        mafia_targets: HashSet<Num>,
        maniac_target: Option<Num>,
        checked: Option<Num>,
    },
    Sunrise { 
        idx: usize,
        dies: Vec<Num>,
    },
    Discussion {
        num: Num,
        accuses: HashSet<Num>,
    },
    Voting {
        canditates: Vec<Candidate>,
    },
    Sunset {
        idx: usize,
        dies: Vec<Num>,
    },
}

impl GameLoop {
    pub fn new() -> Self {
        GameLoop {
            stage: Stage::new(),
            round: 0,
        }
    }


    fn get_round(&self) -> usize {
        self.round
    }

    pub fn next(&mut self) {
        self.stage = match &self.stage {
            Stage::Night {
                checked,
                mafia_targets,
                maniac_target,
            } => {
                let mut dies: Vec<Num> = Vec::new();
                if mafia_targets.len() != 1 {
                    dies.push(mafia_targets.clone().into_iter().collect::<Vec<Num>>()[0]);
                }

                if let Some(maniac_target) = maniac_target {
                    dies.push(*maniac_target);
                }

                Stage::Sunrise { idx: 0, dies, }
            },
            Stage::Sunrise {
                idx,
                dies,
            } => {
                Stage::Discussion {num: self.get_round(), accuses: HashSet::<Num>::new()}
            }
            Stage::Discussion {
                num,
                accuses,
            } => {
                Stage::Voting {
                    canditates: accuses.into_iter().map(|&num: &Num| Candidate::new(num, 0)).collect(),
                }
            }
            Stage::Voting { 
                canditates 
            } => {
                let mut max_votes = 0;
                let mut dies = Vec::<Num>::new();
                for candidate in canditates {
                    if max_votes < candidate.cnt_votes {
                        max_votes = candidate.cnt_votes;
                        dies = Vec::new();
                        dies.push(candidate.num);
                    } else if max_votes == candidate.cnt_votes {
                        dies = Vec::new();
                    }
                }
                Stage::Sunset {
                    idx: 0,
                    dies,
                }
            }
            Stage::Sunset {
                idx,
                dies,
            } => {
                Stage::Night {
                    checked: None,
                    mafia_targets: HashSet::new(),
                    maniac_target: None,
                }
            }
        };
    }
}

impl Stage {
    fn new() -> Self {
        Self::Discussion {num: 0, accuses: HashSet::<Num>::new()}
    }
}

struct Candidate {
    num: Num,
    cnt_votes: usize,
}

impl Candidate {
    pub fn new(num: Num, cnt_votes: usize) -> Self {
        Self {
            num,
            cnt_votes,
        }
    }
}
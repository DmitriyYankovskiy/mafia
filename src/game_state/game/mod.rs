use std::{collections::{HashMap, HashSet}, sync::{Arc, Weak}};

use tokio::{
    fs::File, 
    io::AsyncReadExt, 
    sync::Mutex
};

use rand::{thread_rng, seq::SliceRandom};

use super::role::{Role, RoleSet};

pub mod game_loop;
pub mod character;
pub mod player;

use character::{Character, Num};
use game_loop::{GameLoop, Stage, Candidate};
use player::Player;

pub struct Game {
    characters: Vec<Arc<Mutex<Character>>>,
    players: HashMap<String, Arc<Mutex<Player>>>,
    game_loop: GameLoop,
}

impl Game {
    pub fn new(characters: Vec<Arc<Mutex<Character>>>, players: HashMap<String, Arc<Mutex<Player>>>) -> Self {
        Self {
            characters,
            players,
            game_loop: GameLoop::new(),
        }
    }

    pub async fn remain(&self) -> RoleSet {
        let mut roles = RoleSet::new();
        for character in self.characters.iter() {
            match (*character.lock().await).role {
                Role::Mafia => roles.mafia += 1,
                Role::Maniac => roles.maniac = true,
                Role::Sheriff => roles.sheriff = true,
                Role::Civilian => roles.civilian += 1,
            };
        }
        roles
    }

    pub async fn next(&mut self) {
        let stage = match &*self.game_loop.stage.lock().await {
            Stage::Night {
                mafia_targets,
                maniac_target,
                checked,
            } => {
                let mut dies: Vec<Num> = Vec::new();
                if mafia_targets.len() == self.remain().await.mafia {
                    let mut mafia_target = Option::<Num>::None;
                    for (_, target) in mafia_targets.into_iter() {
                        if (mafia_target.is_none())  {
                            mafia_target = Some(*target);
                        }

                        if (mafia_target.unwrap() != *target) {
                            mafia_target = None;
                            break;
                        }
                    }
                    if let Some(mafia_target) = mafia_target {
                        dies.push(mafia_target);
                    }
                }                

                if let Some(maniac_target) = maniac_target {
                    dies.push(maniac_target.1);
                }

                Stage::Sunrise { idx: 0, dies, }
            },
            Stage::Sunrise {
                idx,
                dies,
            } => {
                Stage::Discussion {num: Num(self.game_loop.get_round()), accused: HashMap::new()}
            }
            Stage::Discussion {accused, .. } => {
                Stage::Voting {
                    candidates: accused.into_iter().map(|(&_, &target): (&Num, &Num)| Candidate::new(target, 0)).collect(),
                    not_voted: {
                        let mut not_voted = HashSet::new();
                        for i in 0..(self.characters.len()) {
                            if self.get_character(Num::from_idx(i)).lock().await.alive {
                                not_voted.insert(Num::from_idx(i));
                            }
                        }
                        not_voted
                    },
                    
                    idx: 0,
                }
            }
            Stage::Voting {candidates, .. } => {
                let mut max_votes = 0;
                let mut dies = Vec::<Num>::new();
                for candidate in candidates {
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
            Stage::Sunset { .. } => {
                Stage::Night {
                    checked: None,
                    mafia_targets: HashMap::new(),
                    maniac_target: None,
                }
            }
        };

        *self.game_loop.stage.lock().await = stage;

        if let Stage::Voting {..} = &mut *self.game_loop.stage.lock().await {
            self.voting_timer();
        }
    }

    async fn voting_timer(&self) {
        let mut cnt = 0usize;
        match &mut *self.game_loop.stage.lock().await {
            Stage::Voting { candidates,.. } => {
                cnt = candidates.len();
            }
            _ => {return}
        }   
        for i in 0usize..cnt {
            if let Stage::Voting { idx, .. } = &mut *self.game_loop.stage.lock().await {
                *idx = i;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5));
        }        
    }

    pub async fn mafia_kill(&mut self, from: Num, target: Num) -> Result<(), ()> {
        if let Stage::Night {mafia_targets, ..} = &mut *self.game_loop.stage.lock().await {
            mafia_targets.insert(from, target);
            Ok(())
        } else {
            Err(())
        }
    }

    pub async fn maniac_kill(&mut self, from: Num, target: Num) -> Result<(), ()> {
        if let Stage::Night {maniac_target, .. } = &mut *self.game_loop.stage.lock().await {
            *maniac_target = Some((from, target));   
            Ok(())
        } else {
            Err(())
        }
    }

    pub async fn check(&mut self, from: Num, target: Num) -> Result<(), ()> {
        if let Stage::Night {checked, .. } = &mut *self.game_loop.stage.lock().await {
            *checked = Some((from, target)); 
            Ok(())
        } else {
            Err(())
        }
    }
    
    pub async fn accuse(&mut self, from: Num, target: Num) -> Result<bool, ()> {
        if let Stage::Discussion {accused, num, .. } = &mut *self.game_loop.stage.lock().await {
            if *num == from {
                let mut was_accused = false;
                for (accuser, accused) in accused.iter() {
                    if *accused == target {
                        was_accused = true;
                    }
                } 
                if was_accused {
                    return Ok(false);
                }
                accused.insert(from, target);
                Ok(true)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub async fn vote(&mut self, from: Num) -> Result<bool, ()> {
        if let Stage::Voting {idx, candidates, not_voted, .. } = &mut *self.game_loop.stage.lock().await {
            if not_voted.contains(&from) {
                not_voted.remove(&from);
                candidates[*idx].cnt_votes += 1;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(())
        }
    }    

    pub fn get_character(&self, num: Num) -> Arc<Mutex<Character>> {
        Arc::clone(&self.characters[num.to_idx()])
    }
}

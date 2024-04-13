use std::{collections::HashSet, fs::File, io::Read, sync::Arc, vec};

use tokio::{
    task::JoinHandle,
    sync::Mutex,
    time,
};

use serde::Deserialize;

use role::{Role, RoleSet};

pub mod game_loop;
pub mod character;
mod external;
pub mod role;

use character::{Character, Num};
use game_loop::Candidate;

use super::player::Player;

#[derive(Deserialize)]
struct TimeDelays {
    discussion: u64,
    voting: u64,
    night: u64,
    last_words: u64,
}

impl TimeDelays {
    pub fn discussion(&self) -> time::Duration {
        time::Duration::from_secs(self.discussion)
    }

    pub fn voting(&self) -> time::Duration {
        time::Duration::from_secs(self.voting)
    }

    pub fn night(&self) -> time::Duration {
        time::Duration::from_secs(self.night)
    } 

    pub fn last_words(&self) -> time::Duration {
        time::Duration::from_secs(self.last_words)
    } 
}

#[derive(Debug)]
enum MafiaTarget {
    Nobody,
    Somebody(Num),
    Miss,
}

impl MafiaTarget {
    pub fn new() -> MafiaTarget {
        return Self::Nobody;
    }

    pub fn add(&mut self, target: Num) {
        match self {
            Self::Nobody => {
                *self = Self::Somebody(target);
            },
            Self::Somebody(num) => {
                if num.to_idx() != target.to_idx() {
                    *self = Self::Miss;
                }
            },
            Self::Miss => {}
        }
    }
}



pub struct Game {
    characters: Vec<Arc<Character>>,
    time_rules: TimeDelays,
    round: Mutex<usize>,
}

impl Game {
    async fn send_action(characters: &Vec<Arc<Character>>, action: external::ActionInfo) {
        for character in characters {
            let _ = character.get_player().ws_sender.send(format!("{{\"Action\":{}}}", serde_json::to_string(&action).unwrap())).await;
        }
    }

    async fn send_time(&self, time: external::TimeInfo) {
        for character in &self.characters {
            let _ = character.get_player().ws_sender.send(format!("{{\"NextPhase\":{}}}", serde_json::to_string(&time).unwrap())).await;
        }
    }

    pub fn new(characters: Vec<Arc<Character>>) -> Self {
        let mut time_rules = String::new();
        File::open("./rules/times.json").unwrap().read_to_string(&mut time_rules).unwrap();
        let time_rules = serde_json::from_str::<TimeDelays>(&time_rules).unwrap();

        Self {
            time_rules,
            characters,
            round: Mutex::new(1),
        }
    }

    pub async fn remain(&self) -> RoleSet {
        let mut roles = RoleSet::new();
        for character in self.characters.iter() {
            match character.info.lock().await.role {
                Role::Mafia => roles.mafia += 1,
                Role::Don => roles.don = true,
                Role::Sheriff => roles.sheriff = true,
                Role::Civilian => roles.civilian += 1,
            };
        }
        roles
    }

    pub async fn run(me: Arc<Self>) {
        println!("game run");

        for character in &me.characters {
            let player = character.get_player();
            let character = character.clone();
            let cnt_characters = me.characters.len();
            println!("{}", cnt_characters);
            player.ws_sender.send(serde_json::to_string({
                let info = &character.info.lock().await; println!("READY");
                &external::StartInfo {
                    num: info.num,
                    cnt_characters: cnt_characters,
                    role: info.role,
                }
            }).unwrap()).await.unwrap();
        }

        println!("all");

        me.game_loop().await;
    }

    async fn game_loop(&self) {
        *self.round.lock().await = 0;
        loop {
            println!(" --- round: {} ---", self.round.lock().await);        

            *self.round.lock().await += 1;
            self.send_time(external::TimeInfo::Discussion).await;
            let candidates: Vec<Num> = self.discussion().await;

            self.send_time(external::TimeInfo::Voting).await;
            let dies = self.voting(candidates.into_iter().map(Candidate::new).collect()).await;
            
            self.send_time(external::TimeInfo::Sunset).await;
            self.sunset(dies).await;

            self.send_time(external::TimeInfo::Night).await;
            let dies = self.night().await;

            self.send_time(external::TimeInfo::Sunrise).await;
            self.sunrise(dies).await;

            if self.check_end().await {
                break;
            }
        }
    }

    async fn check_end(&self) -> bool {
        let remain = self.remain().await;
        remain.mafia >= remain.cnt_red()
    }

    async fn discussion(&self) -> Vec<Num> {
        println!("<discussion>");
        let candidates = Arc::new(Mutex::new(Vec::<Num>::new()));
        for i in 0..self.characters.len() {
            let num = Num::from_idx((i + *self.round.lock().await - 1) % self.characters.len());
            if !self.get_character(num).info.lock().await.alive {continue}

            println!("  player number {} saying:", num.to_idx() + 1);

            let character: Arc<Character> = self.get_character(num);
            let player = character.get_player();
            
            let candidates = Arc::clone(&candidates);
            let characters = self.characters.clone();
            let listner = tokio::spawn(async move {
                'recv: loop {
                    let num = player.recv_accuse().await;
                    for candidate in candidates.lock().await.iter() {
                        if *candidate == num {
                            continue 'recv;
                        }
                    }
                    Game::send_action(&characters, external::ActionInfo::Accuse { num }).await;
                    candidates.lock().await.push(num);
                }
            });

            time::sleep(self.time_rules.discussion()).await;
            listner.abort();
            let _ = listner.await;
        }
        Arc::try_unwrap(candidates).unwrap().into_inner()
    }

    async fn voting(&self, mut candidates: Vec<Candidate>) -> Vec<Num> {
        println!("<voting>");
        if candidates.is_empty() {
            return vec![];
        }
        
        let mut voted = HashSet::<Num>::new();
        for candidate in &mut candidates {
            let mut listners = vec![];
            let mut cnt = 0usize;
            for character in &self.characters {
                let num = character.info.lock().await.num;
                if voted.contains(&num) {
                    continue;
                }
                let player = character.get_player();

                let characters = self.characters.clone();
                listners.push(tokio::spawn(async move {
                    if player.recv_vote().await {
                        Game::send_action(&characters, external::ActionInfo::Vote { num }).await;
                        Some(num)
                    } else {
                        None
                    }
                }));
            }

            time::sleep(self.time_rules.voting()).await;

            for listner in &listners {
                listner.abort();
            };
            for listner in listners {
                if let Ok(Some(num)) = listner.await {
                    voted.insert(num);
                    cnt += 1;
                }
            };

            candidate.cnt_votes = cnt;
        }
        let max = candidates.iter().map(|cand| cand.cnt_votes).max().unwrap();
        let mut dies = vec![];
        for candidate in candidates {
            if candidate.cnt_votes == max {
                dies.push(candidate.num);
            }
        }

        dies
    }

    async fn sunset(&self, dies: Vec<Num>) {
        println!("<sunset>");
        self.dying(&dies).await;
    }

    async fn sunrise(&self, dies: Vec<Num>) {
        println!("<sunrise>");        
        self.dying(&dies).await;
    }

    async fn night(&self) -> Vec<Num>{
        println!("<night>");        
        
        let mut mafia_listners = Vec::<JoinHandle<Num>>::new();
        let mut sheriff_listner = None;
        let mut mafia_target = MafiaTarget::new();
        let mut sheriff_check  = Option::<Num>::None;
        
        for character in &self.characters {
            let player = character.get_player();
            match {player.get_character().await.info.lock().await.role} {
                Role::Mafia => {
                    mafia_listners.push(tokio::spawn(async move {
                        let num = player.recv_action().await;
                        num
                    }));
                },
                Role::Sheriff => {
                    sheriff_listner = Some(tokio::spawn(async move {
                        let num = player.recv_action().await;
                        num
                    }));
                },
                _ => continue,
            }       
        }
        time::sleep(self.time_rules.night()).await;
        for mafia_listner in &mafia_listners {
            mafia_listner.abort();
        }
        if let Some(listner) = &mut sheriff_listner {
            listner.abort();
            if let Ok(res) = listner.await {
                sheriff_check = Some(res);
            }            
        }

        if let Some(num) = sheriff_check {
            for character in &self.characters {
                if character.info.lock().await.role == Role::Sheriff {
                    self.get_character(num).info.lock().await.role.is_black();
                }
            }
        }


        for listner in mafia_listners {
            if let Ok(target) = listner.await {
                mafia_target.add(target);
            }
        };
        if let MafiaTarget::Somebody(num) = mafia_target {
            vec![num]
        } else {
            Vec::new()
        }
    }

    async fn dying(&self, dies: &Vec<Num>) {
        let characters = self.characters.clone();
        for die in dies {
            Game::send_action(&characters, external::ActionInfo::Die { num: *die }).await;
            self.get_character(*die).die().await;
        }
        for _ in dies {
            time::sleep(self.time_rules.last_words()).await;
        }
    }

    pub fn get_character(&self, num: Num) -> Arc<Character> {
        Arc::clone(&self.characters[num.to_idx()])
    }
}
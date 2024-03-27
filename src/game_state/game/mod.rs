use std::{collections::{HashMap, HashSet}, sync::Arc, time::Duration};

use tokio::{
    task::JoinHandle,
    sync::Mutex,
    time,
};

use super::role::{Role, RoleSet};

pub mod game_loop;
pub mod character;
pub mod player;

use character::{Character, Num};
use game_loop::Candidate;
use player::Player;


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
    characters: Vec<Arc<Mutex<Character>>>,
    players: HashMap<String, Arc<Mutex<Player>>>,
    round: usize,
}

impl Game {
    pub fn new(characters: Vec<Arc<Mutex<Character>>>, players: HashMap<String, Arc<Mutex<Player>>>) -> Self {
        Self {
            characters,
            players,
            round: 1,
        }
    }

    pub async fn remain(&self) -> RoleSet {
        let mut roles = RoleSet::new();
        for character in self.characters.iter() {
            match (*character.lock().await).role {
                Role::Mafia => roles.mafia += 1,
                Role::Don => roles.don = true,
                Role::Sheriff => roles.sheriff = true,
                Role::Civilian => roles.civilian += 1,
            };
        }
        roles
    }

    pub async fn run(me: Arc<Mutex<Self>>) {
        println!("game run");
        me.lock().await.game_loop().await;
    }

    async fn game_loop(&mut self) {
        self.round = 0;
        loop {
            println!(" --- round: {} ---", self.round);        

            self.round += 1;
            let candidates: Vec<Num> = self.discussion().await;
            let dies = self.voting(candidates.into_iter().map(Candidate::new).collect()).await;
            self.sunset(dies).await;
            let dies = self.night().await;
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

    async fn discussion(&mut self) -> Vec<Num> {
        let candidates = Arc::new(Mutex::new(Vec::<Num>::new()));
        for i in 0..self.characters.len() {
            let num = Num::from_idx((i + self.round - 1) % self.characters.len());
            if !self.get_character(num).lock().await.alive {continue}

            let character: Arc<Mutex<Character>> = self.get_character(num);
            let player = character.lock().await.get_player();
            
            let candidates = Arc::clone(&candidates);
            let listner = tokio::spawn(async move {
                'recv: loop {
                    let num = player.lock().await.recv_accuse().await;
                    for candidate in candidates.lock().await.iter() {
                        if *candidate == num {
                            continue 'recv;
                        }
                    }
                    candidates.lock().await.push(num);
                }
            });

            time::timeout(time::Duration::from_secs(60), listner).await.unwrap().unwrap();   
        }

        Arc::try_unwrap(candidates).unwrap().into_inner()
    }

    async fn voting(&mut self, mut candidates: Vec<Candidate>) -> Vec<Num> {
        println!("<voting>");
        if candidates.is_empty() {
            return vec![];
        }
        
        let mut voted = HashSet::<Num>::new();
        for candidate in &mut candidates {
            let mut listners = vec![];
            let mut cnt = 0usize;
            for (_, player) in &self.players {
                let num = player.lock().await.get_character().lock().await.num;
                if voted.contains(&num) {
                    continue;
                }

                let player_clone = Arc::clone(player);
                listners.push(tokio::spawn(async move {
                    if player_clone.lock().await.recv_vote().await {
                        Some(num)
                    } else {
                        None
                    }
                }));
            }
            let listners = listners.into_iter().map(
                    |listner| time::timeout(time::Duration::from_secs(3), listner
                ).into_inner())
                .collect::<Vec<JoinHandle<Option<Num>>>>();
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

    async fn sunset(&mut self, dies: Vec<Num>) {
        println!("<sunset>");
        self.dying(&dies).await;
    }

    async fn sunrise(&mut self, dies: Vec<Num>) {
        println!("<sunrise>");        
        self.dying(&dies).await;
        self.round += 1;
    }

    async fn night(&mut self) -> Vec<Num>{
        println!("<night>");        
        
        let mut mafia_listners = Vec::<JoinHandle<Num>>::new();
        let mut sheriff_listner = None;
        let mut mafia_target = MafiaTarget::new();
        let mut sheriff_check  = Option::<Num>::None;
        
        for (_, player) in &self.players {
            let player_clone = Arc::clone(player);
            match {player.lock().await.get_character().lock().await.role} {
                Role::Mafia => {
                    mafia_listners.push(tokio::spawn(async move {
                        let num = player_clone.lock().await.recv_action().await;
                        num
                    }));
                },
                Role::Sheriff => {
                    sheriff_listner = Some(tokio::spawn(async move {
                        let num = player_clone.lock().await.recv_action().await;
                        num
                    }));
                },
                _ => continue,
            }       
        }
        let duration = time::Duration::from_secs(10);
        let mafia_listners = mafia_listners.into_iter().map(
                |listner| time::timeout(duration, listner).into_inner()
            ).collect::<Vec<JoinHandle<Num>>>();
        if let Some(listner) = sheriff_listner {
            if let Ok(res) = time::timeout(duration, listner).await {
                sheriff_check = Some(res.unwrap());
            }            
        }

        if let Some(num) = sheriff_check {
            for character in &self.characters {
                if character.lock().await.role == Role::Sheriff {
                    self.get_character(num).lock().await.role.is_black();
                }
            }
        }


        for listner in mafia_listners {
            mafia_target.add(listner.await.unwrap());
        };
        if let MafiaTarget::Somebody(num) = mafia_target {
            vec![num]
        } else {
            Vec::new()
        }
    }

    async fn dying(&self, dies: &Vec<Num>) {
        for die in dies {
            self.get_character(*die).lock().await.die();
        }
        for _ in dies {
            time::sleep(time::Duration::from_secs(60)).await;
        }
    }

    pub fn get_character(&self, num: Num) -> Arc<Mutex<Character>> {
        Arc::clone(&self.characters[num.to_idx()])
    }
}

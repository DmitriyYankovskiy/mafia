pub mod character;
pub mod role;
pub mod message;

use std::{
    collections::HashSet, fs::File, io::Read, sync::{Arc, Weak}, vec
};

use super::Lobby;

use {
    tokio::{
        task::JoinHandle,
        sync::Mutex,
        time,
    },
    serde::Deserialize,
};

use {
    role::{Role, RoleSet},
    character::{Character, Num},
    message::outgo,
};

#[derive(Deserialize, Clone)]
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



#[derive(Clone, Copy)]
pub struct Candidate {
    pub num: Num,
    pub cnt_votes: usize,
}

impl Candidate {
    pub fn new(num: Num) -> Self {
        Self {
            num,
            cnt_votes: 0,
        }
    }
}

#[derive(Clone)]
pub struct Game {
    lobby: Weak<Lobby>,
    characters: Vec<Arc<Character>>,
    time_rules: TimeDelays,
    round: Arc<Mutex<usize>>,
}

impl Game {
    pub async fn send_all(&self, m: message::outgo::M) {
        self.lobby.upgrade().unwrap().send_all(super::outgo::M::Game(m)).await;
    }

    pub fn new(characters: Vec<Arc<Character>>, lobby: Weak<Lobby>) -> Self {
        let mut time_rules = String::new();
        File::open("./rules/times.json").unwrap().read_to_string(&mut time_rules).unwrap();
        let time_rules = serde_json::from_str::<TimeDelays>(&time_rules).unwrap();

        Self {
            time_rules,
            characters,
            lobby,
            round: Arc::new(Mutex::new(1)),
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
            let character = character.clone();
            let cnt_characters = me.characters.len();
            let (num, role) = {
                let info = character.info.lock().await;
                (info.num, info.role)
            };
            character.send(outgo::M::Start {
                num,
                cnt_characters,
                role,
            }).await;
        }

        me.game_loop().await;
    }

    async fn game_loop(&self) {
        *self.round.lock().await = 0;
        loop {
            println!(" --- round: {} ---", self.round.lock().await);        

            *self.round.lock().await += 1;
            self.send_all(outgo::M::Time(outgo::TimeInfo::Discussion)).await;
            let candidates: Vec<Num> = self.discussion().await;

            self.send_all(outgo::M::Time(outgo::TimeInfo::Voting { candidates: candidates.clone() })).await;
            let dies = self.voting(candidates.into_iter().map(Candidate::new).collect()).await;
            
            self.send_all(outgo::M::Time(outgo::TimeInfo::Sunset)).await;
            self.sunset(dies).await;

            self.send_all(outgo::M::Time(outgo::TimeInfo::Night {time: self.time_rules.night})).await;
            let dies = self.night().await;

            self.send_all(outgo::M::Time(outgo::TimeInfo::Sunrise)).await;
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
            
            self.send_all(outgo::M::Next { num }).await;

            println!("  player number {} saying:", num.to_idx() + 1);

            let character: Arc<Character> = self.get_character(num);
            let player = character.get_player();
            
            let candidates = Arc::clone(&candidates);
            let game = self.clone();
            let listner = tokio::spawn(async move {
                'recv: loop {
                    let num = player.recv_accuse().await;
                    for candidate in candidates.lock().await.iter() {
                        if *candidate == num {
                            continue 'recv;
                        }
                    }
                    game.send_all(outgo::M::Accuse { num }).await;
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
        let mut last = *candidates.last().unwrap();
        let mut sum_cnt = 0;
        candidates.pop();
        for candidate in &mut candidates {
            let mut listners = vec![];
            let mut cnt = 0usize;
            self.send_all(outgo::M::Next { num: candidate.num }).await;
            for character in &self.characters {
                let num = character.info.lock().await.num;
                if voted.contains(&num) {
                    continue;
                }
                let player = character.get_player();

                let game = self.clone();
                listners.push(tokio::spawn(async move {
                    if player.recv_vote().await {
                        game.send_all(outgo::M::Vote { from: num }).await;
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
            sum_cnt += cnt;
            candidate.cnt_votes = cnt;
        }
        println!("{} {sum_cnt}", self.remain().await.cnt());
        last.cnt_votes = self.remain().await.cnt() - sum_cnt;
        candidates.push(last);

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
       

        for character in &self.characters {
            let character = Arc::clone(character);
            let player = character.get_player();            
            match {player.get_character().await.info.lock().await.role} {
                Role::Mafia => {
                    mafia_listners.push(tokio::spawn(async move {
                        let num = player.recv_action().await;
                        num
                    }));
                },
                Role::Sheriff => {
                    let game = self.clone();
                    sheriff_listner = Some(tokio::spawn(async move {
                        let num = player.recv_action().await;
                        character.send(outgo::M::Check {
                            num: num,
                            res: game.get_character(num).info.lock().await.role.is_black()
                        }).await;
                    }));
                },
                _ => continue,
            }       
        }
        time::sleep(self.time_rules.night()).await;
        
        for mafia_listner in &mafia_listners {
            mafia_listner.abort();
        }
        if let Some(listner) = sheriff_listner {
            listner.abort();
            let _ = listner.await;
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
        for die in dies {
            self.send_all(outgo::M::Die { num: *die, time: self.time_rules.last_words }).await;
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
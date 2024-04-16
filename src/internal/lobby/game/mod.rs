pub mod character;
pub mod role;
pub mod messages;

use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    sync::Arc,
    vec
};

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
};

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


pub struct Game {
    characters: Vec<Arc<Character>>,
    time_rules: TimeDelays,
    round: Mutex<usize>,
}

impl Game {   
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
                &messages::StartInfo {
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
            messages::send_time(&self.characters, messages::TimeInfo::Discussion, None).await;
            let candidates: Vec<Num> = self.discussion().await;

            messages::send_time(&self.characters, messages::TimeInfo::Voting, Some(candidates.clone())).await;
            let dies = self.voting(candidates.into_iter().map(Candidate::new).collect()).await;
            
            messages::send_time(&self.characters, messages::TimeInfo::Sunset, None).await;
            self.sunset(dies).await;

            messages::send_time(&self.characters, messages::TimeInfo::Night, None).await;
            let dies = self.night().await;

            messages::send_time(&self.characters, messages::TimeInfo::Sunrise, None).await;
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
            
            messages::send_who_tell(&self.characters, num).await;
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
                    messages::send_action(&characters, messages::ActionInfo::Accuse { num }).await;
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
            messages::send_who_put_it_on(&self.characters, candidate.num).await;
            for character in &self.characters {
                let num = character.info.lock().await.num;
                if voted.contains(&num) {
                    continue;
                }
                let player = character.get_player();

                let characters = self.characters.clone();
                listners.push(tokio::spawn(async move {
                    if player.recv_vote().await {
                        messages::send_action(&characters, messages::ActionInfo::Vote { num }).await;
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
            messages::send_action(&characters, messages::ActionInfo::Die { num: *die }).await;
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
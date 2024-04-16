use std::sync::Arc;

use super::{
    character::{Num, Character},
    role::Role,
};


#[derive(Debug, serde::Serialize)]
pub struct StartInfo {
    pub num: Num,
    pub cnt_characters: usize,
    pub role: Role,
}


#[derive(Debug, serde::Serialize)]
pub enum TimeInfo {
    Night,
    Sunrise, 
    Discussion,
    Voting,
    Sunset,
}

#[derive(Debug, serde::Serialize)]
pub enum ActionInfo {
    Die {num: Num},
    Vote {num: Num},
    Accuse {num: Num},
}

pub async fn send_who_put_it_on(characters: &Vec<Arc<Character>>, num: Num) {
    for character in characters {
        let _ = character.get_player().ws_sender.send(format!("{{\"WhomVoted\":{}}}", serde_json::to_string(&num).unwrap())).await;
    }
}

pub async fn send_action(characters: &Vec<Arc<Character>>, action: ActionInfo) {
    for character in characters {
        let _ = character.get_player().ws_sender.send(format!("{{\"Action\":{}}}", serde_json::to_string(&action).unwrap())).await;
    }
}

pub async fn send_time(characters: &Vec<Arc<Character>>, time: TimeInfo, nums: Option<Vec<Num>>) {
        for character in characters {
            if let TimeInfo::Voting = time {
                let nums = nums.clone().unwrap();
                let _ = character.get_player().ws_sender.send(format!(
                    "{{\"NextPhase\":{},\n \"Votes\":{}}}",
                    serde_json::to_string(&time).unwrap(),
                    serde_json::to_string(&nums).unwrap()
                )).await;
            } else {
                let _ = character.get_player().ws_sender.send(format!("{{\"NextPhase\":{}}}", serde_json::to_string(&time).unwrap())).await;
            }
        }
}

pub async fn send_who_tell(characters: &Vec<Arc<Character>>, num: Num) {
    for character in characters {
        let _ = character.get_player().ws_sender.send(format!("{{\"WhoTell\":{}}}", serde_json::to_string(&num).unwrap())).await;
    }
}
use std::sync::{Arc, Weak};
use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};

use super::{
    character::Character,
    Game,
};

mod message {
    use serde::{Serialize, Deserialize};
    use super::super::character::Num;

    #[derive(Serialize, Deserialize)]
    pub enum Case {
        MafiaKill,
        ManiacKill,
        Check,
        Vote,
        Accuse,
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct Message {
        pub case: Case,
        pub target: Num,
    }
}
use message::{Message, Case};


#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub ws_tx: Sender<String>,
    pub ws_rx: Arc<Mutex<Receiver<String>>>,
    pub character: Weak<Mutex<Character>>,
}

impl Player {
    pub fn new(name: String, ws_tx: Sender<String>, ws_rx: Arc<Mutex<Receiver<String>>>) -> Self {
        Player { name, character: Weak::new(), ws_tx, ws_rx}
    }

    pub fn get_character(&self) -> Arc<Mutex<Character>> {
        self.character.upgrade().unwrap()
    }

    pub async fn get_game(&self) -> Arc<Mutex<Game>> {
        self.get_character().lock().await.get_game()
    }

    pub async fn listen(&self) {
        while let Some(msg) = self.ws_rx.lock().await.recv().await {
            println!("OMG: i read: {msg}");

            let msg = serde_json::from_str::<Message>(&msg).unwrap();
            let game = self.get_game().await;
            match msg.case {
                Case::MafiaKill => {
                    _ = game.lock().await.mafia_kill(
                        self.get_character().lock().await.num,
                        msg.target,
                    ).await;
                },
                Case::ManiacKill => {
                    _ = game.lock().await.maniac_kill(
                        self.get_character().lock().await.num,
                        msg.target,
                    ).await;
                },
                Case::Check => {
                    _ = game.lock().await.check(
                        self.get_character().lock().await.num,
                        msg.target,
                    ).await;
                },
                Case::Vote => {
                    _ = game.lock().await.vote(
                        self.get_character().lock().await.num,
                    )
                },
                Case::Accuse => {
                    _ = game.lock().await.accuse(
                        self.get_character().lock().await.num,
                        msg.target,
                    )
                },
            };           

        }
    }
}
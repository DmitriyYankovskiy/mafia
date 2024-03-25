use std::sync::{Arc, Weak};
use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};

use super::{
    character::{Character, Num},
    Game,
};

pub mod message {
    use serde::{Serialize, Deserialize};
    use super::super::character::Num;
    
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    pub enum Message {
        Action {target: Num},
        Vote,
        Accuse {target: Num},
    }
}
use message::Message;


#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub ws_sender: Sender<String>,
    pub ws_receiver: Arc<Mutex<Receiver<String>>>,
    pub action_receiver: Arc<Mutex<Receiver<Message>>>,
    pub action_sender: Sender<Message>,
    pub character: Weak<Mutex<Character>>,
}

impl Player {
    pub fn new(name: String, ws_sender: Sender<String>, ws_receiver: Arc<Mutex<Receiver<String>>>) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        Player { 
            name, character: Weak::new(), ws_sender, ws_receiver,
            action_receiver: Arc::new(Mutex::new(receiver)),
            action_sender: sender,
        }
    }

    pub fn get_character(&self) -> Arc<Mutex<Character>> {
        self.character.upgrade().unwrap()
    }

    pub async fn get_game(&self) -> Arc<Mutex<Game>> {
        self.get_character().lock().await.get_game()
    }

    pub async fn listen(&self) {
        while let Some(msg) = self.ws_receiver.lock().await.recv().await {
            println!("OMG: i read: {msg}");

            let msg = serde_json::from_str::<Message>(&msg).unwrap();
            self.action_sender.send(msg).await.unwrap();
        }
    }

    pub async fn recv_accuse(&self) -> Num {
        loop { 
            let msg = self.action_receiver.lock().await.recv().await.unwrap(); 
            match msg {
                Message::Accuse {target} => {return target}
                _ => {continue}
            }
        }
    }

    pub async fn recv_vote(&self) -> bool {
        loop {
            let msg = self.action_receiver.lock().await.recv().await.unwrap();
            if let Message::Vote = msg {
                return true;
            }
        }
    }

    pub async fn recv_action(&self) -> Num {
        loop {
            let msg = self.action_receiver.lock().await.recv().await.unwrap();
            if let Message::Action { target } = msg {
                return target;
            }
        }
    }
}
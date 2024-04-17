use std::sync::{Arc, Weak};
use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};

use super::{
    game::{character::{Character, Num}, Game, self},
    message::{incom, outgo},
};

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
}

#[derive(Debug)]
pub struct Player {
    pub info: Mutex<PlayerInfo>,
    ws_sender: Sender<outgo::M>,
    pub ws_receiver: Mutex<Receiver<incom::M>>,
    pub action_receiver: Mutex<Receiver<game::message::incom::M>>,
    action_sender: Sender<game::message::incom::M>,
    pub character: Mutex<Weak<Character>>,
}

impl Player {
    pub fn new(name: String, ws_sender: Sender<outgo::M>, ws_receiver: Mutex<Receiver<incom::M>>) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        Player { 
            info: Mutex::new(PlayerInfo{name}),
            character: Mutex::new(Weak::new()),
            ws_sender, ws_receiver,
            action_receiver: Mutex::new(receiver),
            action_sender: sender,
        }
    }

    pub async fn get_character(&self) -> Arc<Character> {
        self.character.lock().await.upgrade().unwrap()
    }

    pub async fn get_game(&self) -> Arc<Game> {
        self.get_character().await.get_game().await
    }

    pub async fn listen(me: Arc<Self>) {
        while let Some(msg) = me.ws_receiver.lock().await.recv().await {
            if let incom::M::Game(msg) = msg {
                me.action_sender.send(msg).await.unwrap();
            }
        }
    }

    pub async fn recv_accuse(&self) -> Num {
        loop { 
            let msg = self.action_receiver.lock().await.recv().await.unwrap(); 
            match msg {
                game::message::incom::M::Accuse {target} => return target,
                _ => {continue}
            }
        }
    }

    pub async fn recv_vote(&self) -> bool {
        loop {
            let msg = self.action_receiver.lock().await.recv().await.unwrap();
            match msg {
                game::message::incom::M::Vote => return true,
                _ => continue,
            }
        }
    }

    pub async fn recv_action(&self) -> Num {
        loop {
            let msg = self.action_receiver.lock().await.recv().await.unwrap();
            match msg {
                game::message::incom::M::Action {target} => return target,
                _ => {continue}
            }
        }
    }

    pub async fn send(&self, m: outgo::M) {
        self.ws_sender.send(m).await.unwrap();
    }
}
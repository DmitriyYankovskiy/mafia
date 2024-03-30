use std::sync::Arc;
use tokio::{
    task::JoinHandle,
    io::{
        AsyncBufReadExt,
        AsyncReadExt,
        BufReader
    }
};

use crate::{
    Lobby, State,
    lobby::game::character::Num,
};

pub struct Listner {
    lobby: Arc<Lobby>
}
impl Listner {
    pub fn new(lobby: Arc<Lobby>) -> Self {
        Self {lobby}
    }
    pub async fn start(self) -> JoinHandle<()> {
        let lobby = Arc::clone(&self.lobby);
        let mut game_loop;

        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        println!("console ::");
        println!("-- setup --");
        loop {
            let mut command = String::new();
            reader.read_line(&mut command).await.expect("Can't read command");
            let words: Vec<&str> = command.trim().split(' ').into_iter().collect();
            match words[0] {
                "start" => {
                    println!("-- on --");
                    game_loop = tokio::spawn(lobby.start().await.unwrap());
                },
                "character" => {
                    let num: usize = words[1].parse::<usize>().unwrap() - 1;
                    if let State::On{game} = &*self.lobby.state.lock().await {
                        dbg!(game.get_character(Num::from_idx(num)).info.lock().await.role);
                    }
                }
                _ => continue,
            }
        }
    }
}
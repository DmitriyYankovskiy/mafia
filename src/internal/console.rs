use std::sync::Arc;
use tokio::{
    task::JoinHandle,
    io::{
        AsyncBufReadExt,
        BufReader
    }
};


use super::lobby::{game::character::Num, Lobby, State};

pub struct Console {
    lobby: Arc<Lobby>
}
impl Console {
    pub fn new(lobby: Arc<Lobby>) -> Self {
        Self {lobby}
    }
    pub async fn start(self) -> JoinHandle<()> {
        let lobby = Arc::clone(&self.lobby);
        let mut game_loop = None;

        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        println!("console ::");
        println!("-- setup --");
        loop {
            let mut command = String::new();
            reader.read_line(&mut command).await.expect("Can't read command");
            let words: Vec<&str> = command.trim().split(' ').into_iter().collect();
            match words[0] {
                "s" => {
                    println!("-- on --");
                    game_loop = Some(tokio::spawn(lobby.start(Arc::downgrade(&lobby)).await.unwrap()));
                },
                "ch/r" => {
                    let num: usize = words[1].parse::<usize>().unwrap() - 1;
                    if let State::On{game} = &*self.lobby.state.lock().await {
                        dbg!(game.get_character(Num::from_idx(num)).info.lock().await.role);
                    }
                },
                "q" => {
                    if let Some(game_loop) = &game_loop {
                        game_loop.abort();
                    }
                }
                _ => continue,
            }
        }
    }
}
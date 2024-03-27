use std::sync::Arc;
use tokio::{io::AsyncReadExt, sync::Mutex};

use super::GameState;

pub struct Listner {
    game_state: Arc<Mutex<GameState>>,
}
impl Listner {
    pub fn new(game_state: Arc<Mutex<GameState>>) -> Self {
        Self {game_state}
    }
    pub async fn listen(self) {
        println!(">>>");
        loop {
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).expect("Can't read command");
            match command.trim() {
                "start" => {
                    let task = self.game_state.lock().await.start().await.unwrap();
                    tokio::spawn(task);
                },
                _ => continue,
            }
        }
    }
}
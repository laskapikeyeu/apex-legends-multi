use std::process::Command;
use std::thread;
use std::time::Duration;

mod features;

#[tokio::main]
async fn main() {
    features::initialize();
    loop {
        features::check_game_running().await;
        thread::sleep(Duration::from_secs(1));
    }
}
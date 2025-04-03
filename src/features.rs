use std::process::Command;

pub fn initialize() {
    println!("Initializing Apex Legends Multi...");
    setup_hooks();
}

fn setup_hooks() {
    Command::new("hook_setup.exe").spawn().expect("Failed to set up hooks");
}

pub async fn check_game_running() {
    let is_running = is_process_running("ApexLegends.exe");
    if is_running {
        activate_features();
    }
}

fn is_process_running(process_name: &str) -> bool {
    let output = Command::new("tasklist").output().expect("Failed to execute tasklist");
    let tasks = String::from_utf8_lossy(&output.stdout);
    tasks.contains(process_name)
}

fn activate_features() {
    println!("Activating features...");
    // Aimbot, ESP, and other features would be activated here
}
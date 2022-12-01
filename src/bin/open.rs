use std::{env, process};

fn main() {
    let editor = if let Ok(e) = env::var("VISUAL") {
        e
    } else if let Ok(e) = env::var("EDITOR") {
        e
    } else {
        "nano".to_string()
    };

    let problem: u32 = pico_args::Arguments::from_env().free_from_str().expect("What problem should be opened?");

    process::Command::new(&editor)
        .arg(format!("src/bin/{:02}.rs", problem))
        .spawn()
        .expect(&format!("Failed to execute {}", editor));
}

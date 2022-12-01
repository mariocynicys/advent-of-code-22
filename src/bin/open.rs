use std::{env, process};

fn main() {
    let editor = if let Ok(e) = env::var("VISUAL") {
        e
    } else if let Ok(e) = env::var("EDITOR") {
        e
    } else {
        "code".to_string()
    };

    let problem: u32 = pico_args::Arguments::from_env().free_from_str().expect("What problem should be opened?");

    process::Command::new(&editor)
        .arg(format!("src/bin/{:02}.rs", problem))
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to execute {}", editor));
}

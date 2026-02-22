//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! GNU Go Rust Rewrite (gnugo-rs) - Main Entry Point

use gnugo_rs::ui::terminal::TerminalUI;

fn main() {
    println!("GNU Go Rust Rewrite (gnugo-rs) - Starting...");
    
    // Initialize with a 19x19 board
    let mut ui = TerminalUI::new(19);
    
    // Run the game
    match ui.run() {
        Ok(_) => println!("Game exited normally"),
        Err(e) => eprintln!("Game error: {}", e),
    }
}
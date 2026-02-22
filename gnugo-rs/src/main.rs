//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! GNU Go Rust Rewrite (gnugo-rs) - Main Entry Point

use gnugo_rs::ui::terminal::TerminalUI;

#[cfg(feature = "ko_test")]
use gnugo_rs::engine::ko_test::test_ko_rule;

fn main() {
    println!("GNU Go Rust Rewrite (gnugo-rs) - Starting...");
    
    // Simple argument check for test mode
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--test-ko" {
        #[cfg(feature = "ko_test")]
        {
            test_ko_rule();
            return;
        }
        #[cfg(not(feature = "ko_test"))]
        {
            eprintln!("Ko test feature not enabled. Build with '--features ko_test'");
            return;
        }
    }
    
    // Normal game mode
    let mut ui = TerminalUI::new(19);
    
    // Run the game
    match ui.run() {
        Ok(_) => println!("Game exited normally"),
        Err(e) => eprintln!("Game error: {}", e),
    }
}
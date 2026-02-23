//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! GNU Go Rust Rewrite (gnugo-rs) - Main Entry Point

use gnugo_rs::ui::terminal::TerminalUI;
use gnugo_rs::gtp::GTPHandler;

#[cfg(feature = "ko_test")]
use gnugo_rs::engine::ko_test::test_ko_rule;

fn main() {
    println!("GNU Go Rust Rewrite (gnugo-rs) - Starting...");
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--test-ko" => {
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
            },
            "--gtp" => {
                println!("Starting in GTP mode...");
                let mut gtp_handler = GTPHandler::new(19);
                if let Err(e) = gtp_handler.run() {
                    eprintln!("GTP error: {}", e);
                }
                return;
            },
            "--help" => {
                print_help();
                return;
            },
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                print_help();
                return;
            }
        }
    }
    
    // Normal interactive game mode
    let mut ui = TerminalUI::new(19);
    
    // Run the game
    match ui.run() {
        Ok(_) => println!("Game exited normally"),
        Err(e) => eprintln!("Game error: {}", e),
    }
}

fn print_help() {
    println!("GNU Go Rust Rewrite (gnugo-rs)");
    println!("Usage:");
    println!("  gnugo_rs              - Start interactive terminal game");
    println!("  gnugo_rs --gtp        - Start in GTP protocol mode");
    println!("  gnugo_rs --test-ko    - Run ko rule tests (requires ko_test feature)");
    println!("  gnugo_rs --help       - Show this help message");
}
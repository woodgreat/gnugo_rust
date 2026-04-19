//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

use std::io::{self, Write};
use crate::engine::game::Game;
use crate::engine::board::Stone;
use super::board_view::draw_board;

/// Terminal-based interface
pub struct TerminalUI {
    game: Game,
}

impl TerminalUI {
    pub fn new(size: usize) -> Self {
        TerminalUI {
            game: Game::new(size),
        }
    }

    /// Main game loop
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.clear_screen()?;
            draw_board(&self.game.board);
            println!();
            
            // Display game status
            if self.game.is_game_over() {
                println!("Game Status: {}", self.game.status());
                if let Some(winner) = self.game.winner() {
                    println!("Winner: {:?}", winner);
                } else {
                    println!("Result: Tie");
                }
                println!("Captured - Black: {}, White: {}", 
                    self.game.captured(Stone::Black),
                    self.game.captured(Stone::White));
                println!();
                print!("Press Enter to exit...");
                io::stdout().flush()?;
                io::stdin().read_line(&mut String::new())?;
                break;
            }
            
            println!("Current player: {:?}", self.game.current_player());
            println!("Pass count: {}", self.game.pass_count());
            println!("Commands: move (e.g. A1), pass, resign, quit");
            
            print!("Enter command: ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            let input = input.trim().to_lowercase();
            
            match input.as_str() {
                "quit" => break,
                "pass" => {
                    if let Err(e) = self.game.pass() {
                        println!("{}", e);
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                },
                "resign" => {
                    if let Err(e) = self.game.resign() {
                        println!("{}", e);
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                },
                _ => {
                    if let Some((x, y)) = parse_move(&input) {
                        match self.game.make_move(x, y) {
                            Ok(()) => {},
                            Err(e) => {
                                println!("{}", e);
                                std::thread::sleep(std::time::Duration::from_secs(1));
                            }
                        }
                    } else {
                        println!("Invalid command! Use: A1, pass, resign, quit");
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                }
            }
        }
        Ok(())
    }

    fn clear_screen(&self) -> io::Result<()> {
        // Use Windows-specific clear command for better compatibility
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("cmd")
                .args(&["/C", "cls"])
                .status();
        }
        #[cfg(not(windows))]
        {
            print!("\x1B[2J\x1B[H");
            io::stdout().flush()?;
        }
        Ok(())
    }
}

/// Parse move input like "D4" or "d4" into (x, y) coordinates
/// Returns 1-based coordinates (1,1) is top-left
fn parse_move(input: &str) -> Option<(usize, usize)> {
    if input.is_empty() {
        return None;
    }
    
    let mut chars = input.chars();
    let col_char = chars.next()?.to_ascii_uppercase();
    
    // Skip 'I' as in original GNU Go
    if col_char == 'I' {
        return None;
    }
    
    // Convert column letter to X coordinate (A=1, B=2, ...) 
    let mut x = (col_char as u8 - b'A') as usize + 1;
    
    // If letter is after 'I', subtract 1 (skip I)
    if col_char > 'I' {
        x = x.saturating_sub(1);
    }
    
    // Parse row number (1-based from top)
    let row_str: String = chars.collect();
    let y: usize = row_str.parse().ok()?; 
    
    // Return 1-based coordinates (x, y)
    Some((x, y))
}
//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

use std::io::{self, Write};
use crate::engine::board::{Board, Stone};
use super::board_view::draw_board;

/// Terminal-based interface
pub struct TerminalUI {
    board: Board,
    current_player: Stone,
}

impl TerminalUI {
    pub fn new(size: usize) -> Self {
        TerminalUI {
            board: Board::new(size),
            current_player: Stone::Black,
        }
    }

    /// Main game loop
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.clear_screen()?;
            draw_board(&self.board);
            println!("Current player: {:?}", self.current_player);
            
            print!("Move (e.g. A1) or 'quit' to exit: ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            let input = input.trim();
            if input.to_lowercase() == "quit" {
                break;
            }
            
            if let Some((x, y)) = parse_move(input) {
                if x < self.board.size() && y < self.board.size() {
                    if self.board.place_stone(y, x, self.current_player).is_ok() {
                        // Switch players
                        self.current_player = match self.current_player {
                            Stone::Black => Stone::White,
                            Stone::White => Stone::Black,
                            Stone::Empty => Stone::Black, // Should not happen
                        };
                    } else {
                        println!("Invalid move!");
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                } else {
                    println!("Position out of bounds!");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            } else {
                println!("Invalid input format! Use A1, B2, etc.");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        Ok(())
    }

    fn clear_screen(&self) -> io::Result<()> {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush()
    }
}

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
    
    // Convert column letter to index (A=0, B=1, ...) 
    let mut x = (col_char as u8 - b'A') as usize;
    
    // If letter is after 'I', subtract 1 (skip I)
    if col_char > 'I' {
        x = x.saturating_sub(1);
    }
    
    // Parse row number (1-based)
    let row_str: String = chars.collect();
    let y: usize = row_str.parse().ok()?; 
    
    // Convert 1-based to 0-based indexing, return as (row, col) = (y, x)
    Some((y.saturating_sub(1), x))
}
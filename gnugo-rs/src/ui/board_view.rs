//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

use crate::engine::board::{Board, Stone};

/// Draws the board state to terminal with correct coordinate system
/// (following GNU Go's original coordinate layout: origin at bottom-left)
pub fn draw_board(board: &Board) {
    let size = board.size();
    
    // Column headers (A-H, J-T) - exactly matching GNU Go's display
    print!("  ");
    for x in 0..size {
        let col_char = match x {
            0..=7 => (b'A' + x as u8) as char,  // A-H
            _ => (b'A' + x as u8 + 1) as char,  // J-T (skip I)
        };
        print!(" {} ", col_char);
    }
    println!();
    // Board rows - display from top to bottom (19 at top, 1 at bottom)
    for display_row in 0..size {
        let internal_row = size - 1 - display_row; // Convert display row to internal row
        
        // Display row number: top = 19, bottom = 1
        print!("{:2}", size - display_row); 
        for x in 0..size {
            match board.get_stone(x, internal_row) {
                Stone::Black => print!(" ○ "),
                Stone::White => print!(" ● "),
                Stone::Empty => {
                    if board.is_hoshi_point(x, internal_row) {
                        print!(" + ");
                    } else {
                        print!(" · ");
                    }
                }
            }
        }
        
        println!(" {:2}", size - display_row); // Right side row numbers
    }
    
    // Column footers (A, B, C...) - skip I
    print!("  ");
    for x in 0..size {
        let col_char = if x < 8 {
            (b'A' + x as u8) as char
        } else {
            (b'A' + x as u8 + 1) as char // Skip I
        };
        print!(" {} ", col_char);
    }
    println!();
}

/// Converts stone to display character
impl Stone {
    pub fn to_char(&self) -> char {
        match self {
            Stone::Black => '○',
            Stone::White => '●',
            Stone::Empty => '·',
        }
    }
}
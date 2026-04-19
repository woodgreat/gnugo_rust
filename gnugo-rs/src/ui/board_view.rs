//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

use crate::engine::board::{Board, Stone};

/// Draws the board state to terminal with correct coordinate system
/// 
/// # Coordinate System
/// - Origin (1,1) is at the **top-left** corner
/// - X increases from left to right: 1, 2, 3, ..., 19 (columns A, B, C...)
/// - Y increases from top to bottom: 1, 2, 3, ..., 19 (rows)
/// - Display shows row 1 at top, row 19 at bottom (like a spreadsheet)
pub fn draw_board(board: &Board) {
    let size = board.size();
    
    // Column headers (A-H, J-T) - skip I as per Go convention
    print!("  ");
    for x in 1..=size {
        let col_char = match x {
            1..=8 => (b'A' + (x - 1) as u8) as char,  // A-H
            _ => (b'A' + x as u8) as char,  // J-T (skip I)
        };
        print!(" {} ", col_char);
    }
    println!();
    
    // Board rows - display from top (y=1) to bottom (y=size)
    for y in 1..=size {
        // Display row number on left
        print!("{:2}", y);
        for x in 1..=size {
            match board.get_stone(x, y) {
                Stone::Black => print!(" ○ "),
                Stone::White => print!(" ● "),
                Stone::Empty => {
                    if board.is_hoshi_point(x, y) {
                        print!(" + ");
                    } else {
                        print!(" · ");
                    }
                }
            }
        }
        
        // Display row number on right
        println!(" {:2}", y);
    }
    
    // Column footers (A, B, C...) - skip I
    print!("  ");
    for x in 1..=size {
        let col_char = match x {
            1..=8 => (b'A' + (x - 1) as u8) as char,  // A-H
            _ => (b'A' + x as u8) as char,  // J-T (skip I)
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
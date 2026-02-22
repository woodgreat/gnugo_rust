//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

use crate::engine::board::{Board, Stone};

/// Draws the board state to terminal
pub fn draw_board(board: &Board) {
    // Column headers (A, B, C...)
    print!("  ");
    for x in 0..board.size() {
        print!(" {} ", (b'A' + x as u8) as char); 
    }
    println!();
    
    // Board rows with row numbers
    for y in 0..board.size() {
        print!("{:2}", y+1);
        for x in 0..board.size() {
            match board.get_stone(x, y) {
                Stone::Black => print!(" ○ "),
                Stone::White => print!(" ● "),
                Stone::Empty => print!(" · "),
            }
        }
        println!();
    }
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
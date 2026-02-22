//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Move generation and validation

/// Represents a move in Go
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    /// Row coordinate (0-based)
    pub row: usize,
    /// Column coordinate (0-based)
    pub col: usize,
    /// Player making the move (true = black, false = white)
    pub player: bool,
}

impl Move {
    /// Creates a new move
    pub fn new(row: usize, col: usize, player: bool) -> Self {
        Move { row, col, player }
    }
}

/// Trait for move generation
pub trait MoveGenerator {
    /// Generates valid moves for the current position
    fn generate_moves(&self) -> Vec<Move>;
    
    /// Validates if a move is legal
    fn is_legal(&self, mv: &Move) -> bool;
}
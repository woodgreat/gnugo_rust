//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Move generation and validation

use crate::engine::board::Board;
use crate::engine::board::Stone;

/// Represents a move in Go
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    /// Row coordinate
    pub row: usize,
    /// Column coordinate  
    pub col: usize,
}

impl Move {
    /// Creates a new move
    pub fn new(row: usize, col: usize) -> Self {
        Move { row, col }
    }
}

/// Generates valid moves for a given board state
pub struct MoveGenerator;

impl MoveGenerator {
    /// Generates all valid moves for the current player
    pub fn generate_valid_moves(board: &Board, player: Stone) -> Vec<Move> {
        let mut moves = Vec::new();
        let size = board.size();
        
        for row in 0..size {
            for col in 0..size {
                if Self::is_valid_move(board, row, col, player) {
                    moves.push(Move::new(row, col));
                }
            }
        }
        
        moves
    }
    
    /// Checks if a move is valid
    pub fn is_valid_move(board: &Board, row: usize, col: usize, player: Stone) -> bool {
        // Check if position is on board
        if row >= board.size() || col >= board.size() {
            return false;
        }
        
        // Check if position is empty
        if board.get_stone(row, col) != Stone::Empty {
            return false;
        }
        
        // Check for suicide (basic implementation)
        // This is a simplified check - a full implementation would be more complex
        let mut has_liberty = false;
        
        // Check adjacent positions for liberties or friendly stones
        if row > 0 {
            let adjacent_stone = board.get_stone(row - 1, col);
            if adjacent_stone == Stone::Empty || adjacent_stone == player {
                has_liberty = true;
            }
        }
        
        if row < board.size() - 1 {
            let adjacent_stone = board.get_stone(row + 1, col);
            if adjacent_stone == Stone::Empty || adjacent_stone == player {
                has_liberty = true;
            }
        }
        
        if col > 0 {
            let adjacent_stone = board.get_stone(row, col - 1);
            if adjacent_stone == Stone::Empty || adjacent_stone == player {
                has_liberty = true;
            }
        }
        
        if col < board.size() - 1 {
            let adjacent_stone = board.get_stone(row, col + 1);
            if adjacent_stone == Stone::Empty || adjacent_stone == player {
                has_liberty = true;
            }
        }
        
        // If there's no liberty, it might be suicide - but we need to check if it captures
        if !has_liberty {
            // Check if this move captures opponent stones
            let opponent = match player {
                Stone::Black => Stone::White,
                Stone::White => Stone::Black,
                _ => Stone::Empty,
            };
            
            // Check adjacent opponent stones for capture
            if row > 0 && board.get_stone(row - 1, col) == opponent {
                // Would need to check if the group gets captured
                // Simplified: allow if captures opponent
                return true;
            }
            
            if row < board.size() - 1 && board.get_stone(row + 1, col) == opponent {
                return true;
            }
            
            if col > 0 && board.get_stone(row, col - 1) == opponent {
                return true;
            }
            
            if col < board.size() - 1 && board.get_stone(row, col + 1) == opponent {
                return true;
            }
            
            // If no liberties and no captures, it's invalid
            return false;
        }
        
        true
    }
    
    /// Generates all possible moves (including invalid ones)
    pub fn generate_all_moves(board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let size = board.size();
        
        for row in 0..size {
            for col in 0..size {
                moves.push(Move::new(row, col));
            }
        }
        
        moves
    }
}
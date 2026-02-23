//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! AI module for Go game

use crate::engine::board::{Board, Stone};
use crate::engine::evaluation::Evaluator;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// AI difficulty levels
#[derive(Debug, Clone, Copy)]
pub enum AIDifficulty {
    Beginner,   // Random moves
    Intermediate, // Basic evaluation
    Advanced,    // Better evaluation (future)
}

/// AI player
pub struct AI {
    difficulty: AIDifficulty,
}

impl AI {
    /// Create a new AI with given difficulty
    pub fn new(difficulty: AIDifficulty) -> Self {
        AI { difficulty }
    }

    /// Get the best move for the current player
    pub fn get_best_move(&self, board: &Board, player: Stone) -> Option<(usize, usize)> {
        match self.difficulty {
            AIDifficulty::Beginner => self.random_move(board),
            AIDifficulty::Intermediate => self.greedy_move(board, player),
            AIDifficulty::Advanced => self.greedy_move(board, player), // TODO: implement minimax
        }
    }

    /// Random move (beginner level)
    fn random_move(&self, board: &Board) -> Option<(usize, usize)> {
        let size = board.size();
        let mut valid_moves = Vec::new();
        
        // Find all empty positions
        for y in 0..size {
            for x in 0..size {
                if board.get_stone(x, y) == Stone::Empty {
                    valid_moves.push((x, y));
                }
            }
        }
        
        if valid_moves.is_empty() {
            return None;
        }
        
        // Randomly select a move
        let mut rng = thread_rng();
        Some(*valid_moves.choose(&mut rng).unwrap())
    }

    /// Greedy move based on evaluation (intermediate level)
    fn greedy_move(&self, board: &Board, player: Stone) -> Option<(usize, usize)> {
        let size = board.size();
        let mut valid_moves = Vec::new();
        
        // Find all empty positions
        for y in 0..size {
            for x in 0..size {
                if board.get_stone(x, y) == Stone::Empty {
                    valid_moves.push((x, y));
                }
            }
        }
        
        if valid_moves.is_empty() {
            return None;
        }
        
        // Find the move with best evaluation
        let mut best_move = valid_moves[0];
        let mut best_score = i32::MIN;
        
        for (x, y) in valid_moves {
            // Create a temporary board to test the move
            let mut test_board = board.clone();
            
            // Try to place the stone
            if test_board.place_stone(x, y, player).is_ok() {
                let score = Evaluator::evaluate_position(&test_board);
                
                // For black, higher is better; for white, lower is better
                let adjusted_score = if player == Stone::Black { score } else { -score };
                
                if adjusted_score > best_score {
                    best_score = adjusted_score;
                    best_move = (x, y);
                }
            }
        }
        
        Some(best_move)
    }
}

/// Get a random valid move (standalone function for simple AI)
pub fn get_random_move(board: &Board) -> Option<(usize, usize)> {
    let ai = AI::new(AIDifficulty::Beginner);
    ai.get_best_move(board, Stone::Black)
}
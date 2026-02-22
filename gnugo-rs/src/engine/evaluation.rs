//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Position evaluation and scoring

use crate::engine::board::Board;
use crate::engine::board::Stone;

/// Evaluates the strength of a position
pub struct Evaluator;

impl Evaluator {
    /// Evaluates the position for black player
    /// Returns a score where positive values favor black, negative favor white
    pub fn evaluate_position(board: &Board) -> i32 {
        let mut score = 0;
        
        // Count stones
        let black_stones = board.stones_on_board(Stone::Black);
        let white_stones = board.stones_on_board(Stone::White);
        
        score += (black_stones as i32) - (white_stones as i32);
        
        // Evaluate territory and influence (simplified)
        score += Evaluator::evaluate_territory(board);
        score += Evaluator::evaluate_influence(board);
        
        score
    }
    
    /// Evaluates territorial advantage
    fn evaluate_territory(board: &Board) -> i32 {
        let mut territory_score = 0;
        let size = board.size();
        
        // Simple territorial evaluation
        for row in 0..size {
            for col in 0..size {
                let stone = board.get_stone(row, col);
                match stone {
                    Stone::Black => {
                        // Black stone - adds to black territory
                        territory_score += 1;
                    }
                    Stone::White => {
                        // White stone - adds to white territory
                        territory_score -= 1;
                    }
                    Stone::Empty => {
                        // Empty point - check surrounding influence
                        let black_adjacent = Evaluator::count_adjacent_stones(board, row, col, Stone::Black);
                        let white_adjacent = Evaluator::count_adjacent_stones(board, row, col, Stone::White);
                        
                        if black_adjacent > white_adjacent {
                            territory_score += 1;  // Black influence
                        } else if white_adjacent > black_adjacent {
                            territory_score -= 1;  // White influence
                        }
                    }
                }
            }
        }
        
        territory_score
    }
    
    /// Evaluates positional influence
    fn evaluate_influence(board: &Board) -> i32 {
        let mut influence_score = 0;
        let size = board.size();
        
        // Simplified influence evaluation based on stone positions
        for row in 0..size {
            for col in 0..size {
                let stone = board.get_stone(row, col);
                match stone {
                    Stone::Black => {
                        // Black stones have influence in surrounding area
                        influence_score += Evaluator::calculate_influence(board, row, col, Stone::Black);
                    }
                    Stone::White => {
                        // White stones have influence in surrounding area
                        influence_score -= Evaluator::calculate_influence(board, row, col, Stone::White);
                    }
                    Stone::Empty => {
                        // No influence from empty points
                    }
                }
            }
        }
        
        influence_score
    }
    
    /// Counts adjacent stones of a particular color
    fn count_adjacent_stones(board: &Board, row: usize, col: usize, color: Stone) -> usize {
        let mut count = 0;
        let size = board.size();
        
        // Check up
        if row > 0 && board.get_stone(row - 1, col) == color {
            count += 1;
        }
        
        // Check down
        if row < size - 1 && board.get_stone(row + 1, col) == color {
            count += 1;
        }
        
        // Check left
        if col > 0 && board.get_stone(row, col - 1) == color {
            count += 1;
        }
        
        // Check right
        if col < size - 1 && board.get_stone(row, col + 1) == color {
            count += 1;
        }
        
        count
    }
    
    /// Calculates influence from a stone at position
    fn calculate_influence(board: &Board, row: usize, col: usize, color: Stone) -> i32 {
        let mut influence = 0;
        let size = board.size();
        
        // Influence decreases with distance
        for r in 0..size {
            for c in 0..size {
                let distance = (r as i32 - row as i32).abs() + (c as i32 - col as i32).abs();
                if distance <= 3 && board.get_stone(r, c) == color {
                    // Influence value decreases with distance
                    influence += 4 - distance as i32;
                }
            }
        }
        
        influence
    }
    
    /// Estimates the score for a position using simple territory counting
    pub fn estimate_score(board: &Board) -> (i32, i32) {
        let mut black_score = 0i32;
        let mut white_score = 0i32;
        
        let size = board.size();
        
        // Count stones
        black_score += board.stones_on_board(Stone::Black) as i32;
        white_score += board.stones_on_board(Stone::White) as i32;
        
        // Simple territory estimation
        for row in 0..size {
            for col in 0..size {
                match board.get_stone(row, col) {
                    Stone::Black => {
                        black_score += 1;
                    }
                    Stone::White => {
                        white_score += 1;
                    }
                    Stone::Empty => {
                        // Check if surrounded by black or white
                        let mut black_count = 0;
                        let mut white_count = 0;
                        
                        // Check adjacent positions
                        if row > 0 && board.get_stone(row - 1, col) == Stone::Black {
                            black_count += 1;
                        }
                        if row < size - 1 && board.get_stone(row + 1, col) == Stone::Black {
                            black_count += 1;
                        }
                        if col > 0 && board.get_stone(row, col - 1) == Stone::Black {
                            black_count += 1;
                        }
                        if col < size - 1 && board.get_stone(row, col + 1) == Stone::Black {
                            black_count += 1;
                        }
                        
                        if row > 0 && board.get_stone(row - 1, col) == Stone::White {
                            white_count += 1;
                        }
                        if row < size - 1 && board.get_stone(row + 1, col) == Stone::White {
                            white_count += 1;
                        }
                        if col > 0 && board.get_stone(row, col - 1) == Stone::White {
                            white_count += 1;
                        }
                        if col < size - 1 && board.get_stone(row, col + 1) == Stone::White {
                            white_count += 1;
                        }
                        
                        if black_count > white_count {
                            black_score += 1;
                        } else if white_count > black_count {
                            white_score += 1;
                        }
                    }
                }
            }
        }
        
        (black_score, white_score)
    }
}
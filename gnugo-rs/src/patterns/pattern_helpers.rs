//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Helper functions for pattern matching

use crate::engine::board::{Board, Stone};
use crate::patterns::pattern_transform::Transformation;

/// Checks if a move is allowed in the pattern
pub fn move_allowed(
    board: &Board,
    pos: (usize, usize),
    color: Stone,
    transform: Option<Transformation>,
) -> bool {
    let size = board.size();
    let (x, y) = if let Some(trans) = transform {
        trans.apply(pos.0, pos.1, size)
    } else {
        pos
    };

    if x >= size || y >= size {
        return false;
    }

    let stone = board.get_stone(x, y);
    stone == Stone::Empty || stone == color
}

/// Checks if position is on board after transformation
pub fn on_board_after_transform(
    pos: (usize, usize),
    transform: Transformation,
    board_size: usize,
) -> bool {
    let (x, y) = transform.apply(pos.0, pos.1, board_size);
    x < board_size && y < board_size
}

/// Checks if position is on edge after transformation
pub fn on_edge_after_transform(
    pos: (usize, usize),
    transform: Transformation,
    board_size: usize,
) -> bool {
    let (x, y) = transform.apply(pos.0, pos.1, board_size);
    x == 0 || y == 0 || x == board_size - 1 || y == board_size - 1
}

/// Applies autohelper function from pattern database
pub fn apply_autohelper(
    value: i32,
    a: i32,
    b: i32,
    c: i32,
    d: i32,
) -> i32 {
    match value {
        0 => a,
        1 => b,
        2 => c,
        3 => d,
        _ => 0,
    }
}

/// Pattern matching constraints
#[derive(Debug, Clone, Copy)]
pub struct PatternConstraint {
    pub min_edge_distance: usize,
    pub max_edge_distance: usize,
    pub required_stones: usize,
}

impl PatternConstraint {
    pub fn new(min_edge: usize, max_edge: usize, required: usize) -> Self {
        Self {
            min_edge_distance: min_edge,
            max_edge_distance: max_edge,
            required_stones: required,
        }
    }
    
    pub fn check(&self, board: &Board, x: usize, y: usize) -> bool {
        let size = board.size();
        let edge_dist = usize::min(
            usize::min(x, size - 1 - x),
            usize::min(y, size - 1 - y)
        );
        
        edge_dist >= self.min_edge_distance 
            && edge_dist <= self.max_edge_distance
    }
}
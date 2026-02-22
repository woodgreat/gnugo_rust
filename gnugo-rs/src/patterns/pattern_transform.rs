//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Pattern transformation functions similar to transform.c

use crate::engine::board::Board;
use crate::engine::board::Stone;

/// Transformation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transformation {
    Identity,
    Rot90,
    Rot180,
    Rot270,
    Mirror,
    MirrorRot90,
    MirrorRot180,
    MirrorRot270,
}

impl Transformation {
    /// Applies transformation to a coordinate (x,y) on size x size board
    pub fn apply(&self, x: usize, y: usize, size: usize) -> (usize, usize) {
        match self {
            Transformation::Identity => (x, y),
            Transformation::Rot90 => (y, size - 1 - x),
            Transformation::Rot180 => (size - 1 - x, size - 1 - y),
            Transformation::Rot270 => (size - 1 - y, x),
            Transformation::Mirror => (size - 1 - x, y),
            Transformation::MirrorRot90 => (y, x),
            Transformation::MirrorRot180 => (x, size - 1 - y),
            Transformation::MirrorRot270 => (size - 1 - y, size - 1 - x),
        }
    }
    
    /// Returns all possible transformations
    pub fn all() -> [Self; 8] {
        [
            Self::Identity,
            Self::Rot90,
            Self::Rot180,
            Self::Rot270,
            Self::Mirror,
            Self::MirrorRot90,
            Self::MirrorRot180,
            Self::MirrorRot270,
        ]
    }
}

/// Checks if two patterns match under any transformation
pub fn patterns_match(
    board: &Board, 
    pattern: &[(usize, usize, Stone)],
    transformations: &[Transformation]
) -> Option<Transformation> {
    let size = board.size();
    
    'trans: for &trans in transformations {
        let matched = true;
        
        for &(x, y, expected) in pattern {
            let (tx, ty) = trans.apply(x, y, size);
            if board.get_stone(tx, ty) != expected {
                continue 'trans;
            }
        }
        
        if matched {
            return Some(trans);
        }
    }
    
    None
}

/// Helper function to transform pattern coordinates
pub fn transform_pattern(
    pattern: &[(usize, usize, Stone)], 
    trans: Transformation,
    size: usize
) -> Vec<(usize, usize, Stone)> {
    pattern.iter()
        .map(|&(x, y, stone)| {
            let (tx, ty) = trans.apply(x, y, size);
            (tx, ty, stone)
        })
        .collect()
}
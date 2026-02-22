//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Implementation of core pattern matching algorithms

use crate::engine::board::{Board, Stone};
use crate::patterns::{
    PatternDatabase, PatternMatchResult,
    pattern_transform::Transformation,
    pattern_helpers::PatternConstraint,
};

/// Match a pattern at a specific board position
pub struct PatternMatcher<'a> {
    board: &'a Board,
    db: &'a PatternDatabase,
    callback: &'a mut dyn FnMut(PatternMatchResult),
    constraints: PatternConstraint,
}

impl<'a> PatternMatcher<'a> {
    pub fn new(
        board: &'a Board, 
        db: &'a PatternDatabase,
        callback: &'a mut dyn FnMut(PatternMatchResult),
        constraints: PatternConstraint,
    ) -> Self {
        Self {
            board,
            db,
            callback,
            constraints,
        }
    }

    /// Main pattern matching function
    pub fn match_all_positions(&mut self) {
        let size = self.board.size();
        
        for y in 0..size {
            for x in 0..size {
                if !self.constraints.check(self.board, x, y) {
                    continue;
                }
                
                self.match_at_position(x, y);
            }
        }
    }

    fn match_at_position(&mut self, x: usize, y: usize) {
        let stone = self.board.get_stone(x, y);
        if stone == Stone::Empty {
            return;
        }

        for (pattern_id, pattern_values) in self.db.get_patterns().iter() {
            if let Some(trans) = self.pattern_fits(x, y, *pattern_id) {
                for val in pattern_values {
                    (self.callback)(PatternMatchResult {
                        pattern_id: *pattern_id,
                        value: val.value,
                        position: if trans == Transformation::Identity {
                            (x, y)
                        } else {
                            let (tx, ty) = trans.apply(x, y, self.board.size());
                            (tx, ty)
                        },
                        transform: trans,
                    });
                }
            }
        }
    }

    fn pattern_fits(&self, _x: usize, _y: usize, _pattern_id: u32) -> Option<Transformation> {
        // Get pattern info from database
        // For now just return Identity for demonstration
        Some(Transformation::Identity)
    }
}

/// Finds all patterns matching at a specific position
pub fn find_patterns_at(
    board: &Board,
    x: usize,
    y: usize,
    db: &PatternDatabase,
) -> Vec<PatternMatchResult> {
    let mut results = Vec::new();
    let mut callback = |res: PatternMatchResult| results.push(res);
    let constraints = PatternConstraint::new(0, board.size(), 1);
    
    let mut matcher = PatternMatcher::new(board, db, &mut callback, constraints);
    matcher.match_at_position(x, y);
    
    results
}
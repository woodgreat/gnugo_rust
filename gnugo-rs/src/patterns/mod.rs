//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Pattern matching system for GNU Go Rust rewrite

pub mod pattern_database;
pub mod pattern_matching;
pub mod pattern_transform;
pub mod pattern_helpers;
pub mod pattern_matcher_impl;

pub use pattern_database::PatternDatabase;
pub use pattern_matching::PatternMatcher;
pub use pattern_transform::Transformation;
pub use pattern_helpers::{PatternConstraint, move_allowed, on_board_after_transform};
pub use pattern_matcher_impl::find_patterns_at;

/// Represents a pattern value
#[derive(Debug, Clone, Copy)]
pub struct PatVal {
    pub pattern_id: u32,
    pub value: i32,
}

impl PatVal {
    pub fn new(pattern_id: u32, value: i32) -> Self {
        PatVal { pattern_id, value }
    }
}

/// Pattern matching result
#[derive(Debug, Clone)]
pub struct PatternMatchResult {
    pub pattern_id: u32,
    pub value: i32,
    pub position: (usize, usize),
    pub transform: Transformation,
}

/// Pattern type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    Attack,
    Defense,
    Fuseki,
    Joseki,
    Endgame,
}
//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Position evaluation and scoring

/// Evaluation result for a position
#[derive(Debug, Clone, Copy)]
pub struct Evaluation {
    /// Score for black (positive = advantage for black)
    pub score: f64,
    /// Confidence in the evaluation
    pub confidence: f64,
}

impl Evaluation {
    /// Creates a new evaluation
    pub fn new(score: f64, confidence: f64) -> Self {
        Evaluation { score, confidence }
    }
}

/// Trait for position evaluation
pub trait Evaluator {
    /// Evaluates the current position
    fn evaluate(&self) -> Evaluation;
    
    /// Estimates territory for each player
    fn estimate_territory(&self) -> [f64; 2]; // [black, white]
}
//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Pattern matching algorithms

use super::{PatternDatabase, PatternMatchResult, PatternType, PatVal};
use crate::patterns::pattern_database::PatternDatabases;
use crate::engine::board::Board;
use crate::engine::board::Stone;
use crate::patterns::pattern_transform::Transformation;
use std::collections::HashMap;
use std::io;

/// Pattern matcher structure
pub struct PatternMatcher {
    databases: PatternDatabases,
    pattern_cache: HashMap<(u32, usize, usize), Vec<PatternMatchResult>>,
}

impl PatternMatcher {
    /// Creates a new pattern matcher
    pub fn new() -> Self {
        PatternMatcher {
            databases: PatternDatabases::new(),
            pattern_cache: HashMap::new(),
        }
    }
    
    /// Loads all pattern databases
    pub fn load_databases(&mut self) -> io::Result<()> {
        self.databases.load_all()
    }
    
    /// Finds all matching patterns on the board
    pub fn find_matching_patterns(&mut self, board: &Board, pattern_type: PatternType) -> Vec<PatternMatchResult> {
        let mut results = Vec::new();
        
        // Get the appropriate database
        let db = match pattern_type {
            PatternType::Attack => self.databases.get_attack_db(),
            PatternType::Defense => self.databases.get_defense_db(),
            PatternType::Fuseki => self.databases.get_fuseki_db(),
            PatternType::Joseki => self.databases.get_joseki_db(),
            PatternType::Endgame => self.databases.get_endgame_db(),
        };
        
        // For each position on the board
        let size = board.size();
        for row in 0..size {
            for col in 0..size {
                // Check if this position has been cached
                if let Some(cached) = self.pattern_cache.get(&(pattern_type as u32, row, col)) {
                    results.extend(cached.iter().cloned());
                    continue;
                }
                
                // Search for patterns at this position
                let matches = self.search_patterns_at_position(board, row, col, db);
                
                // Cache the results
                self.pattern_cache.insert((pattern_type as u32, row, col), matches.clone());
                
                // Add to results
                results.extend(matches);
            }
        }
        
        results
    }
    
    /// Searches for patterns at a specific position
    fn search_patterns_at_position(&self, board: &Board, row: usize, col: usize, db: &PatternDatabase) -> Vec<PatternMatchResult> {
        let mut results = Vec::new();
        
        // Get the stone at this position
        let stone = board.get_stone(row, col);
        
        // If the position is empty, skip
        if stone == Stone::Empty {
            return results;
        }
        
        // For each pattern in the database
        for (pattern_id, pattern_values) in db.get_patterns().iter() {
            // Check if the pattern matches at this position
            if self.pattern_matches(board, row, col, *pattern_id) {
                // Add all pattern values to results
                for val in pattern_values {
                    results.push(PatternMatchResult {
                        pattern_id: *pattern_id,
                        value: val.value,
                        position: (row, col),
                        transform: Transformation::Identity,
                    });
                }
            }
        }
        
        results
    }
    
    /// Checks if a pattern matches at a specific position
    fn pattern_matches(&self, _board: &Board, _row: usize, _col: usize, _pattern_id: u32) -> bool {
        // This is a simplified implementation
        // In a real implementation, this would check the pattern against the board
        
        // For now, we'll just return true for demonstration purposes
        // In a real implementation, this would be replaced with actual pattern matching logic
        true
    }
    
    /// Evaluates the board using pattern matching
    pub fn evaluate_board(&mut self, board: &Board) -> i32 {
        let mut total_value = 0;
        
        // Evaluate all pattern types
        for pattern_type in [
            PatternType::Attack,
            PatternType::Defense,
            PatternType::Fuseki,
            PatternType::Joseki,
            PatternType::Endgame,
        ].iter() {
            let matches = self.find_matching_patterns(board, *pattern_type);
            
            // Sum the values of all matches
            for m in matches {
                total_value += m.value;
            }
        }
        
        total_value
    }
    
    /// Clears the pattern cache
    pub fn clear_cache(&mut self) {
        self.pattern_cache.clear();
    }
}

/// Predefined pattern values (from patterns.c)
pub const PATTERNS: &[(&[PatVal], &str)] = &[
    (&[
        PatVal { pattern_id: 722, value: 1 },
        PatVal { pattern_id: 795, value: 2 },
        PatVal { pattern_id: 684, value: 2 },
        PatVal { pattern_id: 758, value: 0 },
        PatVal { pattern_id: 759, value: 0 },
        PatVal { pattern_id: 796, value: 0 },
    ], "pat0"),
    (&[
        PatVal { pattern_id: 649, value: 1 },
        PatVal { pattern_id: 684, value: 1 },
        PatVal { pattern_id: 685, value: 2 },
        PatVal { pattern_id: 611, value: 2 },
    ], "pat1"),
    (&[
        PatVal { pattern_id: 722, value: 1 },
        PatVal { pattern_id: 648, value: 1 },
        PatVal { pattern_id: 686, value: 2 },
        PatVal { pattern_id: 684, value: 2 },
    ], "pat2"),
    (&[
        PatVal { pattern_id: 759, value: 1 },
        PatVal { pattern_id: 685, value: 1 },
        PatVal { pattern_id: 723, value: 2 },
        PatVal { pattern_id: 684, value: 2 },
    ], "pat3"),
    (&[
        PatVal { pattern_id: 758, value: 1 },
        PatVal { pattern_id: 684, value: 1 },
        PatVal { pattern_id: 722, value: 2 },
        PatVal { pattern_id: 757, value: 0 },
    ], "pat4"),
];
//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Eye pattern detection and analysis

use super::board::{Board, Stone};

/// Eye pattern data structure
#[derive(Debug, Clone)]
pub struct EyeData {
    pub origin: (usize, usize),          // Origin of the eye space
    pub color: Stone,                     // Color that controls the eye
    pub esize: usize,                     // Number of eye intersections
    pub msize: usize,                     // Number of marginal intersections
    pub value: EyeValue,                  // Eye value
    pub marginal: bool,                   // Is this a marginal eye?
    pub neighbors: usize,                 // Number of neighboring stones
    pub marginal_neighbors: usize,        // Number of marginal neighbors
}

/// Eye value representation
#[derive(Debug, Clone, Copy)]
pub struct EyeValue {
    pub min_eyes: u8,
    pub max_eyes: u8,
    pub is_eye: bool,
}

impl EyeValue {
    pub fn to_string(&self) -> String {
        if self.is_eye {
            format!("{}.{}", self.min_eyes, self.max_eyes)
        } else {
            "0.0".to_string()
        }
    }
}

/// Half-eye pattern data structure
#[derive(Debug, Clone)]
pub struct HalfEyeData {
    pub value: f32,                       // Topological eye value
    pub eye_type: HalfEyeType,            // Type of half-eye
    pub attack_point: Option<(usize, usize)>, // Attack point if any
    pub defense_point: Option<(usize, usize)>, // Defense point if any
}

/// Half-eye types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HalfEyeType {
    Normal,
    Marginal,
    False,
    Half,
    Unknown,
}

/// Eye pattern analyzer
pub struct EyeAnalyzer {
    // Configuration and state for eye detection
}

impl EyeAnalyzer {
    pub fn new() -> Self {
        EyeAnalyzer {}
    }

    /// Analyze eye patterns for the entire board
    pub fn analyze_eyes(&self, board: &Board, color: Stone) -> Vec<EyeData> {
        let mut eyes = Vec::new();
        let size = board.size();
        
        // Simple eye detection algorithm
        for y in 0..size {
            for x in 0..size {
                if let Some(eye) = self.detect_eye(board, x, y, color) {
                    eyes.push(eye);
                }
            }
        }
        
        eyes
    }

    /// Detect if a position is part of an eye
    fn detect_eye(&self, board: &Board, x: usize, y: usize, color: Stone) -> Option<EyeData> {
        if board.get_stone(x, y) != Stone::Empty {
            return None;
        }

        // Check if this empty point is surrounded by stones of the same color
        let mut neighbors = 0;
        let mut enemy_neighbors = 0;
        let mut empty_neighbors = 0;
        
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            
            if nx >= 0 && nx < board.size() as isize && ny >= 0 && ny < board.size() as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                match board.get_stone(nx, ny) {
                    Stone::Black => {
                        if color == Stone::Black {
                            neighbors += 1;
                        } else {
                            enemy_neighbors += 1;
                        }
                    }
                    Stone::White => {
                        if color == Stone::White {
                            neighbors += 1;
                        } else {
                            enemy_neighbors += 1;
                        }
                    }
                    Stone::Empty => empty_neighbors += 1,
                }
            }
        }

        // Simple eye detection logic
        if neighbors >= 3 && enemy_neighbors == 0 {
            Some(EyeData {
                origin: (x, y),
                color,
                esize: 1,
                msize: 0,
                value: EyeValue {
                    min_eyes: 1,
                    max_eyes: 1,
                    is_eye: true,
                },
                marginal: empty_neighbors > 0,
                neighbors,
                marginal_neighbors: empty_neighbors,
            })
        } else {
            None
        }
    }

    /// Check if a move is a ladder attack
    pub fn is_ladder_attack(&self, board: &Board, x: usize, y: usize) -> bool {
        // Simple ladder detection: check if a string has exactly 2 liberties
        // and if attacking it would be effective
        
        let stone = board.get_stone(x, y);
        if stone == Stone::Empty {
            return false;
        }

        // Count liberties
        let liberties = board.count_liberties(x, y);
        if liberties != 2 {
            return false;
        }

        // TODO: Implement proper ladder analysis
        // For now, return true for any string with 2 liberties
        true
    }

    /// Find attack point for a ladder
    pub fn find_ladder_attack_point(&self, board: &Board, x: usize, y: usize) -> Option<(usize, usize)> {
        if !self.is_ladder_attack(board, x, y) {
            return None;
        }

        // Find the liberty that would capture the string
        let liberties = board.find_liberties(x, y);
        if liberties.is_empty() {
            return None;
        }

        // Return the first liberty as the attack point
        Some(liberties[0])
    }

    /// Load eye patterns from file (placeholder)
    pub fn load_from_file(&mut self, _path: &str) -> Result<(), String> {
        // TODO: Implement pattern database loading
        Ok(())
    }
}

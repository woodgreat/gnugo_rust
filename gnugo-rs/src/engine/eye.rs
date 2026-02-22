//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Eye pattern recognition

use crate::engine::board::Board;
use crate::engine::board::Stone;

/// Represents an eye vertex pattern
#[derive(Debug, Clone, Copy)]
pub struct EyeVertex {
    pub x: i32,
    pub y: i32,
    pub type_code: i32,
    pub flags: i32,
    pub neighbors: [i32; 4],
}

impl EyeVertex {
    pub fn new(x: i32, y: i32, type_code: i32, flags: i32, neighbors: [i32; 4]) -> Self {
        EyeVertex {
            x,
            y,
            type_code,
            flags,
            neighbors,
        }
    }
}

/// Eye pattern database
pub struct EyeDatabase {
    eye_patterns: Vec<Vec<EyeVertex>>,
}

impl EyeDatabase {
    /// Creates a new eye database
    pub fn new() -> Self {
        EyeDatabase {
            eye_patterns: Vec::new(),
        }
    }
    
    /// Loads eye patterns from a database file
    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        // In a real implementation, this would load eye patterns from a file
        // For now, we'll add some predefined patterns
        self.add_eye_pattern(&[
            EyeVertex::new(0, 0, 3, 0, [-1, -1, -1, -1]),
        ]);
        
        self.add_eye_pattern(&[
            EyeVertex::new(1, 0, 2, 0, [-1, -1, -1, -1]),
        ]);
        
        self.add_eye_pattern(&[
            EyeVertex::new(1, 0, 1, 0, [-1, -1, -1, -1]),
        ]);
        
        self.add_eye_pattern(&[
            EyeVertex::new(0, 0, 3, 1, [1, -1, -1, -1]),
            EyeVertex::new(0, 0, 3, 1, [0, -1, -1, -1]),
        ]);
        
        Ok(())
    }
    
    /// Adds an eye pattern to the database
    pub fn add_eye_pattern(&mut self, pattern: &[EyeVertex]) {
        self.eye_patterns.push(pattern.to_vec());
    }
    
    /// Gets the number of eye patterns
    pub fn get_pattern_count(&self) -> usize {
        self.eye_patterns.len()
    }
}

/// Eye recognizer
pub struct EyeRecognizer {
    eye_db: EyeDatabase,
}

impl EyeRecognizer {
    /// Creates a new eye recognizer
    pub fn new() -> Self {
        EyeRecognizer {
            eye_db: EyeDatabase::new(),
        }
    }
    
    /// Loads eye patterns
    pub fn load_patterns(&mut self) -> Result<(), String> {
        self.eye_db.load_from_file("patterns/eyes.db")
    }
    
    /// Recognizes eyes on the board
    pub fn recognize_eyes(&self, board: &Board, color: Stone) -> Vec<(usize, usize)> {
        let mut eyes = Vec::new();
        let size = board.size();
        
        for row in 0..size {
            for col in 0..size {
                if board.get_stone(row, col) == Stone::Empty && 
                   self.is_eye(board, row, col, color) {
                    eyes.push((row, col));
                }
            }
        }
        
        eyes
    }
    
    /// Checks if a position is an eye for the given color
    fn is_eye(&self, board: &Board, row: usize, col: usize, color: Stone) -> bool {
        // Check if the position is empty
        if board.get_stone(row, col) != Stone::Empty {
            return false;
        }
        
        // Check surrounding stones
        let size = board.size();
        let mut surrounded = true;
        
        // Check all four directions
        if row > 0 && board.get_stone(row - 1, col) != color {
            surrounded = false;
        }
        if row < size - 1 && board.get_stone(row + 1, col) != color {
            surrounded = false;
        }
        if col > 0 && board.get_stone(row, col - 1) != color {
            surrounded = false;
        }
        if col < size - 1 && board.get_stone(row, col + 1) != color {
            surrounded = false;
        }
        
        surrounded
    }
}
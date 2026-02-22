//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Board representation and management

/// Represents a Go board
#[derive(Debug, Clone)]
pub struct Board {
    /// Board size (typically 9, 13, or 19)
    size: usize,
    /// Board state represented as a 2D array
    grid: Vec<Vec<Stone>>,
}

/// Represents a stone on the board
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stone {
    Empty,
    Black,
    White,
}

impl Board {
    /// Creates a new empty board of the given size
    pub fn new(size: usize) -> Self {
        let mut grid = Vec::with_capacity(size);
        for _ in 0..size {
            grid.push(vec![Stone::Empty; size]);
        }
        
        Board { size, grid }
    }
    
    /// Gets the stone at the given position
    pub fn get_stone(&self, row: usize, col: usize) -> Stone {
        if row < self.size && col < self.size {
            self.grid[row][col]
        } else {
            Stone::Empty
        }
    }
    
    /// Gets the board size
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Places a stone at the given position
    pub fn place_stone(&mut self, row: usize, col: usize, stone: Stone) -> bool {
        if row < self.size && col < self.size {
            self.grid[row][col] = stone;
            true
        } else {
            false
        }
    }
}
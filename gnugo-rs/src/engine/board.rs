//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

use std::fmt;

/// Represents a stone on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stone {
    Empty,
    Black,
    White,
}

/// Represents the Go board
#[derive(Debug, Clone)]
pub struct Board {
    grid: Vec<Vec<Stone>>,
    size: usize,
}

impl Board {
    /// Creates a new empty board of given size
    pub fn new(size: usize) -> Self {
        Board {
            grid: vec![vec![Stone::Empty; size]; size],
            size,
        }
    }

    /// Returns the size of the board
    pub fn size(&self) -> usize {
        self.size
    }

    /// Gets the stone at a specific position (x, y)
    pub fn get_stone(&self, x: usize, y: usize) -> Stone {
        self.grid[y][x]
    }

    /// Places a stone on the board
    pub fn place_stone(&mut self, x: usize, y: usize, stone: Stone) -> Result<(), &'static str> {
        if x >= self.size || y >= self.size {
            return Err("Position out of bounds");
        }
        
        if self.grid[y][x] != Stone::Empty {
            return Err("Position already occupied");
        }
        
        self.grid[y][x] = stone;
        Ok(())
    }

    /// Checks if a position is a hoshi point (star point)
    pub fn is_hoshi_point(&self, x: usize, y: usize) -> bool {
        // No hoshi points on these boards
        if self.size == 2 || self.size == 4 {
            return false;
        }

        // 3x3 board: middle point only
        if self.size == 3 {
            return x == 1 && y == 1;
        }

        // 5x5 board: specific pattern
        if self.size == 5 {
            return (x == 1 && (y == 1 || y == 3)) ||
                   (x == 2 && y == 2) ||
                   (x == 3 && (y == 1 || y == 3));
        }

        // 3-3 points for sizes 7-11, 4-4 for larger
        let hoshi = if self.size <= 11 { 2 } else { 3 };
        let middle = self.size / 2;

        // Normalize coordinates by mirroring to lower numbers
        let m = if x >= middle { self.size - 1 - x } else { x };
        let n = if y >= middle { self.size - 1 - y } else { y };

        // Check corner hoshi
        if m == hoshi && n == hoshi {
            return true;
        }

        // Even sized boards only have corner hoshi
        if self.size % 2 == 0 {
            return false;
        }

        // Boards less than 12 only have middle point
        if self.size < 12 {
            return m == middle && n == middle;
        }

        // Midpoint hoshi for larger boards
        (m == hoshi || m == middle) && (n == hoshi || n == middle)
    }

    /// Counts the number of stones of a specific color on the board
    pub fn stones_on_board(&self, color: Stone) -> usize {
        let mut count = 0;
        for row in &self.grid {
            for &stone in row {
                if stone == color {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stone::Black => write!(f, "Black"),
            Stone::White => write!(f, "White"),
            Stone::Empty => write!(f, "Empty"),
        }
    }
}
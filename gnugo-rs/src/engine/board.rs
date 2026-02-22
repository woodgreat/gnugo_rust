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

/// Represents a group of connected stones
pub struct StoneGroup {
    color: Stone,
    positions: Vec<(usize, usize)>,
    liberties: usize,
}

/// Represents the Go board
#[derive(Debug, Clone)]
pub struct Board {
    grid: Vec<Vec<Stone>>,
    size: usize,
    captured: [usize; 2], // [black, white]
    ko_point: Option<(usize, usize)>, // Ko threat position (if any)
}

impl Board {
    /// Creates a new empty board of given size
    pub fn new(size: usize) -> Self {
        Board {
            grid: vec![vec![Stone::Empty; size]; size],
            size,
            captured: [0, 0],
            ko_point: None,
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

    /// Gets the number of captured stones
    pub fn get_captured(&self) -> [usize; 2] {
        self.captured
    }

    /// Gets the current ko point
    pub fn get_ko_point(&self) -> Option<(usize, usize)> {
        self.ko_point
    }
    
    /// Sets the ko point
    pub fn set_ko_point(&mut self, x: usize, y: usize) {
        self.ko_point = Some((x, y));
    }
    
    /// Clears the ko point
    pub fn clear_ko_point(&mut self) {
        self.ko_point = None;
    }

    /// Directly set a stone at position (x, y) without validation (for testing)
    pub fn set_stone(&mut self, x: usize, y: usize, stone: Stone) {
        self.grid[y][x] = stone;
    }

    /// Finds a group of connected stones at position (x, y) (public for testing)
    pub fn find_group(&self, x: usize, y: usize) -> Option<StoneGroup> {
        if x >= self.size || y >= self.size || self.grid[y][x] == Stone::Empty {
            return None;
        }
        
        let color = self.grid[y][x];
        let mut visited = vec![vec![false; self.size]; self.size];
        let mut positions = Vec::new();
        let mut queue = vec![(x, y)];
        visited[y][x] = true;
        
        while let Some((cx, cy)) = queue.pop() {
            positions.push((cx, cy));
            
            // Check 4 neighbors
            let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            for &(dx, dy) in &neighbors {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                
                if nx >= 0 && nx < self.size as isize && ny >= 0 && ny < self.size as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    
                    if !visited[ny][nx] && self.grid[ny][nx] == color {
                        visited[ny][nx] = true;
                        queue.push((nx, ny));
                    }
                }
            }
        }
        
        // Count liberties
        let liberties = self.count_liberties_for_positions(&positions);
        
        Some(StoneGroup {
            color,
            positions,
            liberties,
        })
    }

    /// Places a stone on the board and handles captures
    pub fn place_stone(&mut self, x: usize, y: usize, stone: Stone) -> Result<(), &'static str> {
        if x >= self.size || y >= self.size {
            return Err("Position out of bounds");
        }
        
        if self.grid[y][x] != Stone::Empty {
            return Err("Position already occupied");
        }
        
        // Check ko rule
        if let Some((ko_x, ko_y)) = self.ko_point {
            if x == ko_x && y == ko_y {
                return Err("Ko threat violation");
            }
        }
        
        // Place the stone
        self.grid[y][x] = stone;
        
        let opponent = match stone {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            Stone::Empty => return Err("Cannot place empty stone"),
        };
        
        // Check and capture opponent stones in all 4 directions
        let mut captured_any = false;
        let mut captured_single_stone = false;
        let mut capture_position = (0, 0);
        
        // Check all 4 directions for captures
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            
            if nx >= 0 && nx < self.size as isize && ny >= 0 && ny < self.size as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                
                if self.grid[ny][nx] == opponent {
                    if let Some(group) = self.find_group(nx, ny) {
                        if group.liberties == 0 {
                            // Check if this is a single stone capture (potential ko)
                            if group.positions.len() == 1 {
                                captured_single_stone = true;
                                capture_position = group.positions[0];
                            }
                            
                            self.capture_group(&group);
                            captured_any = true;
                        }
                    }
                }
            }
        }
        
        // Set ko point if exactly one stone was captured
        if captured_single_stone {
            self.set_ko_point(capture_position.0, capture_position.1);
        } else {
            self.clear_ko_point();
        }
        
        // If no opponent was captured, check if our own stone has liberties
        if !captured_any {
            if let Some(own_group) = self.find_group(x, y) {
                if own_group.liberties == 0 {
                    // Suicide - remove our own stone
                    self.grid[y][x] = Stone::Empty;
                    return Err("Suicide move not allowed");
                }
            }
        }
        
        Ok(())
    }

    /// Counts liberties for a group of positions
    fn count_liberties_for_positions(&self, positions: &[(usize, usize)]) -> usize {
        let mut liberties = 0;
        let mut checked = vec![vec![false; self.size]; self.size];
        
        for &(x, y) in positions {
            let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            for &(dx, dy) in &neighbors {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                
                if nx >= 0 && nx < self.size as isize && ny >= 0 && ny < self.size as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    
                    if !checked[ny][nx] && self.grid[ny][nx] == Stone::Empty {
                        liberties += 1;
                        checked[ny][nx] = true;
                    }
                }
            }
        }
        
        liberties
    }

    /// Captures a group of stones
    fn capture_group(&mut self, group: &StoneGroup) {
        let count = group.positions.len();
        
        for &(x, y) in &group.positions {
            self.grid[y][x] = Stone::Empty;
        }
        
        // Update captured count
        match group.color {
            Stone::Black => self.captured[0] += count,
            Stone::White => self.captured[1] += count,
            Stone::Empty => {}
        }
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
            return (x == 1 && (y == 1 || y == 3))
                || (x == 2 && y == 2)
                || (x == 3 && (y == 1 || y == 3));
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
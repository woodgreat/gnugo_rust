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
    /// Captured stones count
    captured: [usize; 2], // [black, white]
}

/// Represents a stone on the board
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stone {
    Empty,
    Black,
    White,
}

/// Represents a group of connected stones
#[derive(Debug, Clone)]
pub struct Group {
    /// The color of the stones in the group
    color: Stone,
    /// The positions of the stones in the group
    positions: Vec<(usize, usize)>,
    /// The number of liberties for the group
    liberties: usize,
}

impl Board {
    /// Creates a new empty board of the given size
    pub fn new(size: usize) -> Self {
        let mut grid = Vec::with_capacity(size);
        for _ in 0..size {
            grid.push(vec![Stone::Empty; size]);
        }
        
        Board { 
            size, 
            grid,
            captured: [0, 0],
        }
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
    
    /// Gets the number of captured stones for each player
    pub fn get_captured(&self) -> [usize; 2] {
        self.captured
    }
    
    /// Places a stone at the given position and handles captures
    pub fn place_stone(&mut self, row: usize, col: usize, stone: Stone) -> Result<(), String> {
        if row >= self.size || col >= self.size {
            return Err("Position out of bounds".to_string());
        }
        
        if self.grid[row][col] != Stone::Empty {
            return Err("Position already occupied".to_string());
        }
        
        // Place the stone
        self.grid[row][col] = stone;
        
        // Check for captures in all four directions
        let opponent = match stone {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            _ => Stone::Empty,
        };
        
        // Check up
        if row > 0 && self.grid[row-1][col] == opponent {
            self.check_and_capture(row-1, col);
        }
        
        // Check down
        if row < self.size - 1 && self.grid[row+1][col] == opponent {
            self.check_and_capture(row+1, col);
        }
        
        // Check left
        if col > 0 && self.grid[row][col-1] == opponent {
            self.check_and_capture(row, col-1);
        }
        
        // Check right
        if col < self.size - 1 && self.grid[row][col+1] == opponent {
            self.check_and_capture(row, col+1);
        }
        
        Ok(())
    }
    
    /// Checks if a group has no liberties and captures it if so
    fn check_and_capture(&mut self, row: usize, col: usize) {
        if let Some(group) = self.find_group(row, col) {
            if group.liberties == 0 {
                self.capture_group(&group);
            }
        }
    }
    
    /// Captures a group of stones
    fn capture_group(&mut self, group: &Group) {
        let count = group.positions.len();
        
        // Remove the stones from the board
        for &(r, c) in &group.positions {
            self.grid[r][c] = Stone::Empty;
        }
        
        // Update captured count
        match group.color {
            Stone::Black => self.captured[0] += count,
            Stone::White => self.captured[1] += count,
            _ => (),
        }
    }
    
    /// Finds the group of connected stones at the given position
    pub fn find_group(&self, row: usize, col: usize) -> Option<Group> {
        if row >= self.size || col >= self.size || self.grid[row][col] == Stone::Empty {
            return None;
        }
        
        let color = self.grid[row][col];
        let mut visited = vec![vec![false; self.size]; self.size];
        let mut positions = Vec::new();
        let mut queue = vec![(row, col)];
        visited[row][col] = true;
        
        while let Some((r, c)) = queue.pop() {
            positions.push((r, c));
            
            // Check all four directions
            if r > 0 && !visited[r-1][c] && self.grid[r-1][c] == color {
                visited[r-1][c] = true;
                queue.push((r-1, c));
            }
            if r < self.size - 1 && !visited[r+1][c] && self.grid[r+1][c] == color {
                visited[r+1][c] = true;
                queue.push((r+1, c));
            }
            if c > 0 && !visited[r][c-1] && self.grid[r][c-1] == color {
                visited[r][c-1] = true;
                queue.push((r, c-1));
            }
            if c < self.size - 1 && !visited[r][c+1] && self.grid[r][c+1] == color {
                visited[r][c+1] = true;
                queue.push((r, c+1));
            }
        }
        
        let liberties = self.calculate_group_liberties(&positions);
        
        Some(Group {
            color,
            positions,
            liberties,
        })
    }
    
    /// Calculates the number of liberties for a group of stones
    fn calculate_group_liberties(&self, positions: &[(usize, usize)]) -> usize {
        let mut liberties = 0;
        let mut checked = vec![vec![false; self.size]; self.size];
        
        for &(r, c) in positions {
            // Check up
            if r > 0 && !checked[r-1][c] && self.grid[r-1][c] == Stone::Empty {
                liberties += 1;
                checked[r-1][c] = true;
            }
            
            // Check down
            if r < self.size - 1 && !checked[r+1][c] && self.grid[r+1][c] == Stone::Empty {
                liberties += 1;
                checked[r+1][c] = true;
            }
            
            // Check left
            if c > 0 && !checked[r][c-1] && self.grid[r][c-1] == Stone::Empty {
                liberties += 1;
                checked[r][c-1] = true;
            }
            
            // Check right
            if c < self.size - 1 && !checked[r][c+1] && self.grid[r][c+1] == Stone::Empty {
                liberties += 1;
                checked[r][c+1] = true;
            }
        }
        
        liberties
    }
}
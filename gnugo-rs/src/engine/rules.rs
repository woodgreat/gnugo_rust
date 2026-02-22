//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

use crate::engine::board::{Board, Stone};

/// Represents the Go game rules configuration
#[derive(Debug, Clone, Copy)]
pub struct GameRules {
    /// Allow suicide moves
    pub allow_suicide: bool,
    /// Ko rule type
    pub ko_rule: KoRule,
}

/// Different types of ko rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KoRule {
    /// No ko restrictions
    None,
    /// Simple ko rule (most common)
    Simple,
    /// Superko rules (various types)
    Superko,
}

impl Default for GameRules {
    fn default() -> Self {
        GameRules {
            allow_suicide: false,
            ko_rule: KoRule::Simple,
        }
    }
}

impl GameRules {
    /// Checks if a move is legal according to game rules
    pub fn is_legal_move(&self, board: &Board, x: usize, y: usize, stone: Stone) -> Result<(), &'static str> {
        if x >= board.size() || y >= board.size() {
            return Err("Position out of bounds");
        }
        
        // Check if position is empty
        if board.get_stone(x, y) != Stone::Empty {
            return Err("Position already occupied");
        }
        
        // Check ko rule
        if self.ko_rule != KoRule::None {
            if let Some((ko_x, ko_y)) = board.get_ko_point() {
                if x == ko_x && y == ko_y {
                    return Err("Ko threat violation");
                }
            }
        }
        
        // Create a temporary board to test the move
        let mut test_board = board.clone();
        
        // Try to place the stone
        if let Err(e) = test_board.place_stone(x, y, stone) {
            if !self.allow_suicide && e == "Suicide move not allowed" {
                return Err("Suicide move not allowed");
            }
            return Err(e);
        }
        
        Ok(())
    }
}

/// Extension trait for Board to add ko rule support
pub trait BoardExt {
    /// Gets the current ko point
    fn get_ko_point(&self) -> Option<(usize, usize)>;
    
    /// Sets the ko point
    fn set_ko_point(&mut self, x: usize, y: usize);
    
    /// Clears the ko point
    fn clear_ko_point(&mut self);
    
    /// Check for ko threat after a capture
    fn check_ko_threat(&mut self, captured_group_size: usize, capture_pos: (usize, usize));
}

impl BoardExt for Board {
    fn get_ko_point(&self) -> Option<(usize, usize)> {
        self.ko_point
    }
    
    fn set_ko_point(&mut self, x: usize, y: usize) {
        self.ko_point = Some((x, y));
    }
    
    fn clear_ko_point(&mut self) {
        self.ko_point = None;
    }
    
    fn check_ko_threat(&mut self, captured_group_size: usize, capture_pos: (usize, usize)) {
        // If exactly one stone was captured, it might be a ko threat
        if captured_group_size == 1 {
            self.set_ko_point(capture_pos.0, capture_pos.1);
        } else {
            self.clear_ko_point();
        }
    }
}

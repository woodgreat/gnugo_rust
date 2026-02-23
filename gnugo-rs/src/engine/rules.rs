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
            if board.get_ko_point() == Some((x, y)) {
                return Err("Ko threat violation");
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
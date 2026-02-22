//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Game logic and state management

use crate::engine::board::Board;
use crate::engine::board::Stone;

/// Represents the state of a Go game
#[derive(Debug, Clone)]
pub struct Game {
    /// The current board state
    pub board: Board,
    /// Current player to move (true = black, false = white)
    pub current_player: bool,
    /// Game history for undo functionality
    pub history: Vec<GameState>,
    /// Captured stones count
    pub captured_stones: [u32; 2], // [black, white]
}

/// Represents a snapshot of game state
#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Board,
    pub current_player: bool,
    pub captured_stones: [u32; 2],
}

impl Game {
    /// Creates a new game with an empty board
    pub fn new(size: usize) -> Self {
        Game {
            board: Board::new(size),
            current_player: true, // Black moves first
            history: Vec::new(),
            captured_stones: [0, 0],
        }
    }
    
    /// Makes a move on the board
    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), String> {
        // Save current state for potential undo
        self.history.push(GameState {
            board: self.board.clone(),
            current_player: self.current_player,
            captured_stones: self.captured_stones,
        });
        
        // Try to place the stone
        let stone = if self.current_player { 
            Stone::Black 
        } else { 
            Stone::White 
        };
        
        match self.board.place_stone(row, col, stone) {
            Ok(()) => {
                // Update captured stones count
                let [black_captured, white_captured] = self.board.get_captured();
                self.captured_stones[0] = black_captured as u32;
                self.captured_stones[1] = white_captured as u32;
                
                // Switch players
                self.current_player = !self.current_player;
                
                Ok(())
            },
            Err(e) => {
                // Undo the state change
                self.history.pop();
                Err(e)
            }
        }
    }
    
    /// Undoes the last move
    pub fn undo_move(&mut self) -> Option<()> {
        if let Some(last_state) = self.history.pop() {
            self.board = last_state.board;
            self.current_player = last_state.current_player;
            self.captured_stones = last_state.captured_stones;
            Some(())
        } else {
            None
        }
    }
    
    /// Returns the current player
    pub fn current_player(&self) -> Stone {
        if self.current_player {
            Stone::Black
        } else {
            Stone::White
        }
    }
    
    /// Returns the number of stones captured by a player
    pub fn captured(&self, color: Stone) -> u32 {
        match color {
            Stone::Black => self.captured_stones[0],
            Stone::White => self.captured_stones[1],
            _ => 0,
        }
    }
}
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
    /// Pass count - consecutive passes
    pass_count: u32,
    /// Game status
    status: GameStatus,
    /// Winner (if game is over)
    winner: Option<Stone>,
}

/// Game status
#[derive(Debug, Clone, Copy, PartialEq)]
enum GameStatus {
    InProgress,
    Ended,
    Resigned,
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
            pass_count: 0,
            status: GameStatus::InProgress,
            winner: None,
        }
    }
    
    /// Makes a move on the board
    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), String> {
        if self.status != GameStatus::InProgress {
            return Err("Game is already over".to_string());
        }

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
                // Update captured stones count and reset pass count when a move is made
                self.update_captured_stones();
                self.reset_pass_count();
                
                // Switch players
                self.current_player = !self.current_player;
                
                Ok(())
            },
            Err(e) => {
                // Undo the state change
                self.history.pop();
                Err(e.to_string())
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

    /// Player passes turn
    pub fn pass(&mut self) -> Result<(), String> {
        if self.status != GameStatus::InProgress {
            return Err("Game is already over".to_string());
        }

        self.pass_count += 1;
        
        // If both players pass consecutively, end the game
        if self.pass_count >= 2 {
            self.status = GameStatus::Ended;
            self.determine_winner();
        }
        
        // Switch players
        self.current_player = !self.current_player;
        
        Ok(())
    }

    /// Player resigns
    pub fn resign(&mut self) -> Result<(), String> {
        if self.status != GameStatus::InProgress {
            return Err("Game is already over".to_string());
        }

        self.status = GameStatus::Resigned;
        self.winner = Some(match self.current_player() {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            _ => Stone::Empty,
        });
        
        Ok(())
    }

    /// Check if game is over
    pub fn is_game_over(&self) -> bool {
        self.status != GameStatus::InProgress
    }

    /// Get game status
    pub fn status(&self) -> &'static str {
        match self.status {
            GameStatus::InProgress => "In Progress",
            GameStatus::Ended => "Ended by agreement",
            GameStatus::Resigned => "Resigned",
        }
    }

    /// Get winner (if any)
    pub fn winner(&self) -> Option<Stone> {
        self.winner
    }

    /// Score territory and determine winner (simple implementation)
    fn determine_winner(&mut self) {
        // Simple scoring: count stones + territory
        let black_score = self.board.stones_on_board(Stone::Black) as i32 + self.captured_stones[0] as i32;
        let white_score = self.board.stones_on_board(Stone::White) as i32 + self.captured_stones[1] as i32;
        
        if black_score > white_score {
            self.winner = Some(Stone::Black);
        } else if white_score > black_score {
            self.winner = Some(Stone::White);
        } else {
            self.winner = None; // Tie
        }
    }

    /// Get pass count
    pub fn pass_count(&self) -> u32 {
        self.pass_count
    }

    /// Reset pass count (当移动时重置)
    fn reset_pass_count(&mut self) {
        self.pass_count = 0;
    }

    /// Update captured stones count from board
    fn update_captured_stones(&mut self) {
        let [black_captured, white_captured] = self.board.get_captured();
        self.captured_stones[0] = black_captured as u32;
        self.captured_stones[1] = white_captured as u32;
    }
}
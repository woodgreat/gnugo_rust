//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Configurable board renderer with support for custom symbols and colors

use crate::engine::board::{Board, Stone};

/// Configuration for board rendering
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub empty_symbol: char,
    pub black_symbol: char,
    pub white_symbol: char,
    pub use_colors: bool,
    pub show_coordinates: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            empty_symbol: '·',
            black_symbol: '○',
            white_symbol: '●',
            use_colors: true,
            show_coordinates: true,
        }
    }
}

/// Configurable board renderer
pub struct BoardRenderer {
    config: RenderConfig,
}

impl BoardRenderer {
    /// Creates a new renderer with custom configuration
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }
    
    /// Creates a renderer with default configuration
    pub fn with_defaults() -> Self {
        Self::new(RenderConfig::default())
    }
    
    /// Renders the board to a string
    pub fn render(&self, board: &Board) -> String {
        let mut output = String::new();
        
        if self.config.show_coordinates {
            // Column headers (A, B, C...)
            output.push_str("  ");
            for x in 0..board.size() {
                let col_char = (b'A' + x as u8) as char;
                output.push_str(&format!(" {} ", col_char));
            }
            output.push('\n');
        }
        
        // Board rows
        for y in 0..board.size() {
            if self.config.show_coordinates {
                output.push_str(&format!("{:2}", y + 1));
            }
            
            for x in 0..board.size() {
                let stone = board.get_stone(x, y);
                let symbol = self.stone_to_symbol(stone);
                
                if self.config.use_colors {
                    output.push_str(&self.colorize(symbol, stone));
                } else {
                    output.push_str(&format!(" {} ", symbol));
                }
            }
            output.push('\n');
        }
        
        output
    }
    
    /// Converts stone to appropriate symbol
    fn stone_to_symbol(&self, stone: Stone) -> char {
        match stone {
            Stone::Empty => self.config.empty_symbol,
            Stone::Black => self.config.black_symbol,
            Stone::White => self.config.white_symbol,
        }
    }
    
    /// Adds color to the symbol (if enabled)
    fn colorize(&self, symbol: char, stone: Stone) -> String {
        if !self.config.use_colors {
            return format!(" {} ", symbol);
        }
        
        match stone {
            Stone::Black => format!("\x1b[34m{} \x1b[0m", symbol), // Blue
            Stone::White => format!("\x1b[37m{} \x1b[0m", symbol), // White
            Stone::Empty => format!(" {} ", symbol),
        }
    }
    
    /// Updates render configuration
    pub fn update_config(&mut self, config: RenderConfig) {
        self.config = config;
    }
    
    /// Gets current configuration
    pub fn config(&self) -> &RenderConfig {
        &self.config
    }
}
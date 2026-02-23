//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! SGF (Smart Game Format) file support for GNU Go Rust

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::engine::board::{Board, Stone};
use crate::engine::game::Game;

/// SGF property types
#[derive(Debug, Clone, PartialEq)]
pub enum SGFProperty {
    Number(i32),
    Real(f32),
    Double(i32), // 1 or 2
    Color(Stone),
    Text(String),
    Point((usize, usize)),
    Move((usize, usize)),
    None,
}

/// SGF node in the game tree
#[derive(Debug, Clone)]
pub struct SGFNode {
    pub properties: HashMap<String, Vec<SGFProperty>>,
    pub children: Vec<SGFNode>,
}

/// SGF game tree structure
#[derive(Debug, Clone)]
pub struct SGFTree {
    pub root: SGFNode,
    pub current: usize, // current node index
}

/// SGF file parser and generator
pub struct SGFHandler;

impl SGFHandler {
    pub fn new() -> Self {
        SGFHandler
    }

    /// Load SGF file and return game tree
    pub fn load_file<P: AsRef<Path>>(&self, path: P) -> Result<SGFTree, String> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let file = File::open(path).map_err(|e| format!("Cannot open file '{}': {}", path_str, e))?;
        let reader = BufReader::new(file);
        
        let mut content = String::new();
        let mut line_number = 0;
        for line in reader.lines() {
            line_number += 1;
            let line_content = line.map_err(|e| format!("Read error at line {} in '{}': {}", line_number, path_str, e))?;
            content.push_str(&line_content);
        }

        self.parse(&content).map_err(|e| format!("Parse error in '{}': {}", path_str, e))
    }

    /// Parse SGF content string
    pub fn parse(&self, content: &str) -> Result<SGFTree, String> {
        let mut chars = content.chars().peekable();
        let mut position = 0;
        self.parse_tree(&mut chars, &mut position)
    }

    /// Parse SGF game tree with position tracking
    fn parse_tree(&self, chars: &mut std::iter::Peekable<std::str::Chars>, position: &mut usize) -> Result<SGFTree, String> {
        self.skip_whitespace(chars, position);
        
        if chars.next() != Some('(') {
            return Err(format!("Expected '(' at position {}", position));
        }
        *position += 1;

        let root = self.parse_node(chars, position)?;
        
        self.skip_whitespace(chars, position);
        if chars.next() != Some(')') {
            return Err(format!("Expected ')' at position {}", position));
        }
        *position += 1;

        Ok(SGFTree {
            root,
            current: 0,
        })
    }

    /// Parse SGF node with position tracking
    fn parse_node(&self, chars: &mut std::iter::Peekable<std::str::Chars>, position: &mut usize) -> Result<SGFNode, String> {
        self.skip_whitespace(chars, position);
        
        if chars.peek() != Some(&';') {
            return Err(format!("Expected ';' at position {}", position));
        }
        chars.next(); // consume ';'
        *position += 1;

        let mut properties = HashMap::new();
        let mut children = Vec::new();

        // Parse properties
        while let Some(&c) = chars.peek() {
            if c == '(' || c == ')' {
                break;
            }
            
            if c.is_ascii_uppercase() {
                let (key, values) = self.parse_property(chars, position)?;
                properties.insert(key, values);
            } else if c == ';' {
                // ';' indicates start of a new node, not end of properties
                break;
            } else {
                return Err(format!("Unexpected character '{}' at position {}", c, position));
            }
        }

        // Parse children
        while let Some(&c) = chars.peek() {
            match c {
                ';' => {
                    children.push(self.parse_node(chars, position)?);
                }
                '(' => {
                    children.push(self.parse_branch(chars, position)?);
                }
                _ => break,
            }
        }

        Ok(SGFNode {
            properties,
            children,
        })
    }

    /// Parse property with values and position tracking
    fn parse_property(&self, chars: &mut std::iter::Peekable<std::str::Chars>, position: &mut usize) -> Result<(String, Vec<SGFProperty>), String> {
        let key = self.read_identifier(chars, position)?;
        let mut values = Vec::new();

        while chars.peek() == Some(&'[') {
            chars.next(); // consume '['
            *position += 1;
            let value = self.parse_value(chars, position)?;
            values.push(value);
            
            if chars.next() != Some(']') {
                return Err(format!("Expected ']' after property value at position {}", position));
            }
            *position += 1;
        }

        Ok((key, values))
    }

    /// Parse property value with position tracking
    fn parse_value(&self, chars: &mut std::iter::Peekable<std::str::Chars>, position: &mut usize) -> Result<SGFProperty, String> {
        let mut value_str = String::new();
        
        while let Some(&c) = chars.peek() {
            if c == ']' || c == '\\' {
                break;
            }
            if c == '\\' {
                chars.next(); // consume escape
                *position += 1;
                if let Some(c) = chars.next() {
                    value_str.push(c);
                    *position += 1;
                }
            } else {
                value_str.push(chars.next().unwrap());
                *position += 1;
            }
        }

        // Try to parse as different types
        if let Ok(num) = value_str.parse::<i32>() {
            return Ok(SGFProperty::Number(num));
        }
        if let Ok(real) = value_str.parse::<f32>() {
            return Ok(SGFProperty::Real(real));
        }
        if value_str == "1" || value_str == "2" {
            return Ok(SGFProperty::Double(value_str.parse().unwrap()));
        }
        if value_str == "B" {
            return Ok(SGFProperty::Color(Stone::Black));
        }
        if value_str == "W" {
            return Ok(SGFProperty::Color(Stone::White));
        }
        if value_str.len() == 2 {
            if let Some((x, y)) = self.parse_point(&value_str) {
                return Ok(SGFProperty::Point((x, y)));
            }
        }

        Ok(SGFProperty::Text(value_str))
    }

    /// Parse SGF point (e.g., "dd")
    fn parse_point(&self, s: &str) -> Option<(usize, usize)> {
        if s.len() != 2 {
            return None;
        }
        
        let mut chars = s.chars();
        let col_char = chars.next()?;
        let row_char = chars.next()?;
        
        if !col_char.is_ascii_lowercase() || !row_char.is_ascii_lowercase() {
            return None;
        }
        
        let x = (col_char as u8 - b'a') as usize;
        let y = (row_char as u8 - b'a') as usize;
        
        Some((x, y))
    }

    /// Parse branch (subtree) with position tracking
    fn parse_branch(&self, chars: &mut std::iter::Peekable<std::str::Chars>, position: &mut usize) -> Result<SGFNode, String> {
        if chars.next() != Some('(') {
            return Err(format!("Expected '(' at position {}", position));
        }
        *position += 1;
        
        let node = self.parse_node(chars, position)?;
        
        if chars.next() != Some(')') {
            return Err(format!("Expected ')' at position {}", position));
        }
        *position += 1;
        
        Ok(node)
    }

    /// Read identifier (property key) with position tracking
    fn read_identifier(&self, chars: &mut std::iter::Peekable<std::str::Chars>, position: &mut usize) -> Result<String, String> {
        let mut ident = String::new();
        
        while let Some(&c) = chars.peek() {
            if c.is_ascii_uppercase() {
                ident.push(chars.next().unwrap());
                *position += 1;
            } else {
                break;
            }
        }
        
        if ident.is_empty() {
            return Err(format!("Expected identifier at position {}", position));
        }
        
        Ok(ident)
    }

    /// Skip whitespace characters with position tracking
    fn skip_whitespace(&self, chars: &mut std::iter::Peekable<std::str::Chars>, position: &mut usize) {
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                *position += 1;
            } else {
                break;
            }
        }
    }

    /// Convert game to SGF format with move history
    pub fn game_to_sgf(&self, game: &Game, filename: Option<&str>) -> Result<String, String> {
        let mut sgf = String::new();
        
        // SGF header
        sgf.push_str("(;FF[4]GM[1]SZ[");
        sgf.push_str(&game.board.size().to_string());
        sgf.push_str("]KM[");
        sgf.push_str(&game.komi.to_string());
        sgf.push_str("]\n");

        // Export current board state as setup properties
        if game.board.size() > 0 {
            let mut black_stones = Vec::new();
            let mut white_stones = Vec::new();
            
            println!("DEBUG: Board size: {}", game.board.size());
            
            for y in 0..game.board.size() {
                for x in 0..game.board.size() {
                    let stone = game.board.get_stone(x, y);
                    if stone != Stone::Empty {
                        let point = format_sgf_point(x, y);
                        println!("DEBUG: Stone at ({},{}) = {:?} -> {}", x, y, stone, point);
                        match stone {
                            Stone::Black => black_stones.push(point),
                            Stone::White => white_stones.push(point),
                            Stone::Empty => continue,
                        }
                    }
                }
            }
            
            println!("DEBUG: Black stones: {:?}", black_stones);
            println!("DEBUG: White stones: {:?}", white_stones);
            
            if !black_stones.is_empty() {
                sgf.push_str(&format!(";AB[{}]", black_stones.join("][")));
                println!("DEBUG: Added AB property");
            }
            if !white_stones.is_empty() {
                sgf.push_str(&format!(";AW[{}]", white_stones.join("][")));
                println!("DEBUG: Added AW property");
            }
            sgf.push('\n');
        }

        sgf.push_str(")\n");

        println!("DEBUG: Final SGF content:\n{}", sgf);

        // Write to file if filename provided
        if let Some(filename) = filename {
            let mut file = File::create(filename)
                .map_err(|e| format!("Cannot create file '{}': {}", filename, e))?;
            file.write_all(sgf.as_bytes())
                .map_err(|e| format!("Write error to '{}': {}", filename, e))?;
        }

        Ok(sgf)
    }

    /// Apply SGF tree to game
    pub fn apply_to_game(&self, tree: &SGFTree, game: &mut Game) -> Result<(), String> {
        // Start with empty board of correct size
        if let Some(SGFProperty::Number(size)) = tree.root.properties.get("SZ").and_then(|v| v.first()) {
            *game = Game::new(*size as usize);
        }

        // Apply komi
        if let Some(SGFProperty::Real(komi)) = tree.root.properties.get("KM").and_then(|v| v.first()) {
            game.komi = *komi;
        }

        // Apply moves from SGF tree
        self.apply_moves(&tree.root, game)
    }

    /// Recursively apply moves from SGF node
    fn apply_moves(&self, node: &SGFNode, game: &mut Game) -> Result<(), String> {
        // Save game state for branch support
        let game_snapshot = game.clone();
        
        // Apply moves from this node
        if let Some(moves) = node.properties.get("B") {
            for mv in moves {
                if let SGFProperty::Point((x, y)) = mv {
                    game.make_move(*y, *x)
                        .map_err(|e| format!("Failed to apply black move: {}", e))?;
                }
            }
        }
        
        if let Some(moves) = node.properties.get("W") {
            for mv in moves {
                if let SGFProperty::Point((x, y)) = mv {
                    game.make_move(*y, *x)
                        .map_err(|e| format!("Failed to apply white move: {}", e))?;
                }
            }
        }

        // Recursively apply moves from children (branches)
        for (i, child) in node.children.iter().enumerate() {
            if i > 0 {
                // For branches beyond the first, restore snapshot
                *game = game_snapshot.clone();
            }
            self.apply_moves(child, game)?;
        }

        Ok(())
    }
}

/// Format point to SGF format (e.g., "dd")
pub fn format_sgf_point(x: usize, y: usize) -> String {
    let col_char = (b'a' + x as u8) as char;
    let row_char = (b'a' + y as u8) as char;
    format!("{}{}", col_char, row_char)
}

/// Convert Stone to SGF color
pub fn stone_to_sgf_color(stone: Stone) -> &'static str {
    match stone {
        Stone::Black => "B",
        Stone::White => "W",
        Stone::Empty => "",
    }
}
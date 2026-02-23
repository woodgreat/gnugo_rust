//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Go Text Protocol (GTP) implementation for GNU Go Rust

use std::io::{self, BufRead, Write};
use std::fs;
use crate::engine::game::Game;
use crate::engine::board::Stone;
use crate::engine::eye::EyeAnalyzer;
use crate::sgf::SGFHandler;

/// GTP protocol handler
pub struct GTPHandler {
    game: Game,
    eye_analyzer: EyeAnalyzer,
}

impl GTPHandler {
    /// Create a new GTP handler
    pub fn new(size: usize) -> Self {
        GTPHandler {
            game: Game::new(size),
            eye_analyzer: EyeAnalyzer::new(),
        }
    }

    /// Run GTP protocol loop
    pub fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();        
        for line in stdin.lock().lines() {
            let line = line?;
            let response = self.process_command(&line);            
            writeln!(stdout, "{}", response)?;
            stdout.flush()?;
        }        
        Ok(())
    }

    /// Process a single GTP command
    fn process_command(&mut self, command: &str) -> String {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return "".to_string();
        }

        // Handle command ID (optional)
        let (id, cmd_parts) = if let Ok(id) = parts[0].parse::<u32>() {
            (Some(id), &parts[1..])
        } else {
            (None, &parts[..])
        };

        if cmd_parts.is_empty() {
            return self.format_response(id, "unknown command");
        }

        let response = match cmd_parts[0] {
            "protocol_version" => self.protocol_version(),
            "name" => self.name(),
            "version" => self.version(),
            "boardsize" => self.boardsize(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "clear_board" => self.clear_board(),
            "komi" => self.komi(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "get_komi" => self.get_komi(),
            "play" => self.play(if cmd_parts.len() > 2 { (cmd_parts[1], cmd_parts[2]) } else { ("", "") }),
            "genmove" => self.genmove(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "genmove_black" => self.genmove_black(),
            "genmove_white" => self.genmove_white(),
            "undo" => self.undo(),
            "captures" => self.captures(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "final_score" => self.final_score(),
            "time_settings" => self.time_settings(),
            "is_legal" => self.is_legal(if cmd_parts.len() > 2 { (cmd_parts[1], cmd_parts[2]) } else { ("", "") }),
            "list_stones" => self.list_stones(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "countlib" => self.countlib(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "findlib" => self.findlib(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "ladder_attack" => self.ladder_attack(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "eye_data" => self.eye_data(if cmd_parts.len() > 2 { (cmd_parts[1], cmd_parts[2]) } else { ("", "") }),
            "loadsgf" => self.loadsgf(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "printsgf" => self.printsgf(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            "quit" => "quit".to_string(),
            "list_commands" => self.list_commands(),
            "showboard" => self.showboard(),
            "known_command" => self.known_command(if cmd_parts.len() > 1 { cmd_parts[1] } else { "" }),
            _ => format!("unknown command: {}", cmd_parts[0]),
        };

        self.format_response(id, &response)
    }

    /// Format GTP response
    fn format_response(&self, id: Option<u32>, content: &str) -> String {
        let prefix = if content.starts_with('?') { "?" } else { "=" };
        let id_str = id.map(|i| i.to_string()).unwrap_or_default();
        if content.is_empty() {
            format!("{}{}\n\n", prefix, id_str)
        } else {
            format!("{}{} {}\n\n", prefix, id_str, content)
        }
    }

    // GTP command implementations
    fn protocol_version(&self) -> String { "2".to_string() }
    fn name(&self) -> String { "gnugo_rs".to_string() }
    fn version(&self) -> String { "0.2.0".to_string() }

    fn boardsize(&mut self, size_str: &str) -> String {
        match size_str.parse::<usize>() {
            Ok(size) if (1..=25).contains(&size) => {
                self.game = Game::new(size);
                "".to_string()
            }
            _ => "? unacceptable size".to_string(),
        }
    }

    fn clear_board(&mut self) -> String {
        let size = self.game.board.size();
        self.game = Game::new(size);
        "".to_string()
    }

    fn komi(&mut self, komi_str: &str) -> String {
        match komi_str.parse::<f32>() {
            Ok(komi) if (-360.0..360.0).contains(&komi) => {
                self.game.komi = komi;
                "".to_string()
            }
            _ => "? invalid komi".to_string(),
        }
    }

    fn get_komi(&self) -> String { format!("{}", self.game.komi) }

    fn known_command(&self, command: &str) -> String {
        let commands = vec![
            "protocol_version", "name", "version", "boardsize", 
            "clear_board", "komi", "get_komi", "play", "genmove", 
            "genmove_black", "genmove_white", "undo", "captures",
            "final_score", "time_settings", "quit",
            "list_commands", "showboard", "known_command",
            "is_legal", "list_stones", "countlib", "findlib",
            "echo", "echo_err", "ladder_attack", "eye_data",
            "loadsgf", "printsgf",
        ];
        if commands.contains(&command) { "true".to_string() } else { "false".to_string() }
    }

    fn play(&mut self, (color, move_str): (&str, &str)) -> String {
        let _stone = match color.to_lowercase().as_str() {
            "black" => Stone::Black,
            "white" => Stone::White,
            _ => return "? invalid color".to_string(),
        };

        if move_str.to_lowercase() == "pass" {
            return match self.game.pass() {
                Ok(()) => "".to_string(),
                Err(e) => format!("? {}", e),
            };
        }

        if let Some((x, y)) = parse_gtp_move(move_str, self.game.board.size()) {
            match self.game.make_move(y, x) {
                Ok(()) => "".to_string(),
                Err(e) => format!("? {}", e),
            }
        } else {
            "? invalid move".to_string()
        }
    }

    fn genmove(&mut self, color: &str) -> String {
        let _stone = match color.to_lowercase().as_str() {
            "black" => Stone::Black,
            "white" => Stone::White,
            _ => return "? invalid color".to_string(),
        };

        // Simple AI: find first valid move
        let size = self.game.board.size();
        for y in 0..size {
            for x in 0..size {
                if self.game.board.get_stone(x, y) == Stone::Empty {
                    if self.game.make_move(y, x).is_ok() {
                        return format_move(x, y);
                    }
                }
            }
        }

        // If no valid moves, pass
        match self.game.pass() {
            Ok(()) => "pass".to_string(),
            Err(e) => format!("? {}", e),
        }
    }

    fn genmove_black(&mut self) -> String { self.genmove("black") }
    fn genmove_white(&mut self) -> String { self.genmove("white") }

    fn undo(&mut self) -> String {
        match self.game.undo_move() {
            Some(()) => "".to_string(),
            None => "? cannot undo".to_string(),
        }
    }

    fn captures(&self, color: &str) -> String {
        let stone = match color.to_lowercase().as_str() {
            "black" => Stone::Black,
            "white" => Stone::White,
            _ => return "? invalid color".to_string(),
        };
        format!("{}", self.game.captured(stone))
    }

    fn final_score(&self) -> String {
        if !self.game.is_game_over() {
            return "? game not finished".to_string();
        }
        match self.game.winner() {
            Some(Stone::Black) => "B+".to_string(),
            Some(Stone::White) => "W+".to_string(),
            _ => "0".to_string(),
        }
    }

    fn time_settings(&self) -> String { "".to_string() }

    fn is_legal(&self, (color, move_str): (&str, &str)) -> String {
        let stone = match color.to_lowercase().as_str() {
            "black" => Stone::Black,
            "white" => Stone::White,
            _ => return "? invalid color".to_string(),
        };
        
        if move_str.to_lowercase() == "pass" { return "1".to_string(); }
        
        if let Some((x, y)) = parse_gtp_move(move_str, self.game.board.size()) {
            if self.game.board.get_stone(x, y) == Stone::Empty {
                let mut test_board = self.game.board.clone();
                return match test_board.place_stone(y, x, stone) {
                    Ok(()) => "1".to_string(),
                    Err(_) => "0".to_string(),
                };
            }
        }
        "0".to_string()
    }

    fn list_stones(&self, color: &str) -> String {
        let stone = match color.to_lowercase().as_str() {
            "black" => Stone::Black,
            "white" => Stone::White,
            _ => return "? invalid color".to_string(),
        };
        
        let size = self.game.board.size();
        let mut stones = Vec::new();
        for y in 0..size {
            for x in 0..size {
                if self.game.board.get_stone(x, y) == stone {
                    stones.push(format_move(x, y));
                }
            }
        }
        stones.join("\n")
    }

    fn countlib(&self, move_str: &str) -> String {
        if let Some((x, y)) = parse_gtp_move(move_str, self.game.board.size()) {
            let liberties = self.game.board.count_liberties(x, y);
            format!("{}", liberties)
        } else {
            "? invalid move".to_string()
        }
    }

    fn findlib(&self, move_str: &str) -> String {
        if let Some((x, y)) = parse_gtp_move(move_str, self.game.board.size()) {
            let liberties = self.game.board.find_liberties(x, y);
            liberties.iter().map(|&(x, y)| format_move(x, y)).collect::<Vec<_>>().join("\n")
        } else {
            "? invalid move".to_string()
        }
    }

    /// Implementation of ladder_attack command
    fn ladder_attack(&self, move_str: &str) -> String {
        if let Some((x, y)) = parse_gtp_move(move_str, self.game.board.size()) {
            if self.game.board.get_stone(x, y) == Stone::Empty {
                return "? vertex must not be empty".to_string();
            }
            
            let liberties = self.game.board.count_liberties(x, y);
            if liberties != 2 {
                return "? string must have exactly 2 liberties".to_string();
            }
            
            // Use eye analyzer to find attack point
            if let Some(attack_point) = self.eye_analyzer.find_ladder_attack_point(&self.game.board, x, y) {
                format!("1 {}", format_move(attack_point.0, attack_point.1))
            } else {
                "0".to_string()
            }
        } else {
            "? invalid move".to_string()
        }
    }

    /// Implementation of eye_data command
    fn eye_data(&self, (color, move_str): (&str, &str)) -> String {
        let stone_color = match color.to_lowercase().as_str() {
            "black" => Stone::Black,
            "white" => Stone::White,
            _ => return "? invalid color".to_string(),
        };
        
        if let Some((x, y)) = parse_gtp_move(move_str, self.game.board.size()) {
            let eyes = self.eye_analyzer.analyze_eyes(&self.game.board, stone_color);
            
            // Find eye data for the specified position
            for eye in eyes {
                if eye.origin == (x, y) {
                    return format!(
                        "origin {} {}\ncolor {}\nesize {}\nmsize {}\nvalue {}\nmarginal {}\nneighbors {}\nmarginal_neighbors {}",
                        eye.origin.0, eye.origin.1, 
                        match eye.color {
                            Stone::Black => "black",
                            Stone::White => "white",
                            Stone::Empty => "empty",
                        },
                        eye.esize, eye.msize, eye.value.to_string(), 
                        eye.marginal as u8, eye.neighbors, eye.marginal_neighbors
                    );
                }
            }
            
            "? no eye data for this position".to_string()
        } else {
            "? invalid move".to_string()
        }
    }

    fn list_commands(&self) -> String {
        vec![
            "protocol_version", "name", "version", "boardsize", "clear_board",
            "komi", "get_komi", "play", "genmove", "genmove_black", "genmove_white",
            "undo", "captures", "final_score", "time_settings",
            "is_legal", "list_stones", "quit", "list_commands", "showboard", "known_command",
            "countlib", "findlib", "echo", "echo_err",
            "ladder_attack", "eye_data",
        ].join("\n")
    }

    fn showboard(&self) -> String {
        let board = &self.game.board;
        let size = board.size();
        let mut result = String::new();
        for y in 0..size {
            for x in 0..size {
                let c = match board.get_stone(x, y) {
                    Stone::Black => 'X',
                    Stone::White => 'O',
                    Stone::Empty => '.',
                };
                result.push(c);
                result.push(' ');
            }
            result.push('\n');
        }
        result
    }

    /// Implementation of loadsgf command
    fn loadsgf(&mut self, filename: &str) -> String {
        if filename.is_empty() {
            return "? missing filename".to_string();
        }

        let sgf_handler = SGFHandler::new();
        match sgf_handler.load_file(filename) {
            Ok(tree) => {
                println!("DEBUG: SGF tree loaded successfully");
                println!("DEBUG: Root properties: {:?}", tree.root.properties.keys());
                
                if let Err(e) = sgf_handler.apply_to_game(&tree, &mut self.game) {
                    return format!("? {}", e);
                }
                "".to_string()
            }
            Err(e) => format!("? {}", e),
        }
    }

    /// Implementation of printsgf command
    fn printsgf(&self, filename: &str) -> String {
        let sgf_handler = SGFHandler::new();
        match sgf_handler.game_to_sgf(&self.game, if filename.is_empty() { None } else { Some(filename) }) {
            Ok(sgf_content) => {
                if filename.is_empty() {
                    sgf_content
                } else {
                    "".to_string()
                }
            }
            Err(e) => format!("? {}", e),
        }
    }
}

/// Parse GTP move format (e.g., "D4")
fn parse_gtp_move(move_str: &str, board_size: usize) -> Option<(usize, usize)> {
    if move_str.len() < 2 { return None; }
    
    let mut chars = move_str.chars();
    let col_char = chars.next()?.to_ascii_uppercase();
    if col_char == 'I' { return None; }
    
    let mut x = (col_char as u8 - b'A') as usize;
    if col_char > 'I' { x -= 1; }
    
    let row_str: String = chars.collect();
    let y: usize = match row_str.parse::<usize>() {
        Ok(n) if n > 0 && n <= board_size => n - 1,
        _ => return None,
    };
    
    if x < board_size && y < board_size { Some((x, y)) } else { None }
}

/// Format move to GTP format (e.g., "D4")
fn format_move(x: usize, y: usize) -> String {
    let col_char = if x < 8 { (b'A' + x as u8) as char } else { (b'A' + x as u8 + 1) as char };
    format!("{}{}", col_char, y + 1)
}
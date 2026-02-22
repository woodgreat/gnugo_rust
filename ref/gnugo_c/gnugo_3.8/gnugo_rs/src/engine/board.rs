//! 从engine/board.c转写
//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later
#[derive(Debug)]
pub struct Board {
    pub size: u32,
    pub stones: Vec<Stone>,
}

#[repr(u8)]
pub enum Stone {
    Empty = 0,
    Black = 1,
    White = 2,
}

impl Board {
    pub fn new(size: u32) -> Self {
        Self {
            size,
            stones: vec![Stone::Empty; (size * size) as usize],
        }
    }
}
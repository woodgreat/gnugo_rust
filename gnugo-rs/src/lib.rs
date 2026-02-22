//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! GNU Go Rust Rewrite (gnugo-rs) - Core Library

pub mod engine;
pub mod patterns;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
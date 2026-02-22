//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Pattern database loader for GNU Go

use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::patterns::{PatternDatabase, PatternType, PatVal};

/// Loads a pattern database from a .db file
pub fn load_database(path: &str, pattern_type: PatternType) -> io::Result<PatternDatabase> {
    let mut db = PatternDatabase::new(path, pattern_type);
    
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    // Read header
    let magic = reader.read_u32::<LittleEndian>()?;
    if magic != 0x474E5547 { // "GNU Go" magic number
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid pattern database"));
    }
    
    let _version = reader.read_u32::<LittleEndian>()?;
    let pattern_count = reader.read_u32::<LittleEndian>()?;
    
    // Read patterns
    for _ in 0..pattern_count {
        let pattern_id = reader.read_u32::<LittleEndian>()?;
        let value_count = reader.read_u32::<LittleEndian>()?;
        
        let mut values = Vec::with_capacity(value_count as usize);
        for _ in 0..value_count {
            let val_pattern_id = reader.read_u32::<LittleEndian>()?;
            let val_value = reader.read_i32::<LittleEndian>()?;
            values.push(PatVal::new(val_pattern_id, val_value));
        }
        
        db.add_pattern(pattern_id, values);
    }
    
    Ok(db)
}

/// Loads all standard pattern databases
pub fn load_all_databases() -> io::Result<(
    PatternDatabase,
    PatternDatabase,
    PatternDatabase,
    PatternDatabase,
    PatternDatabase
)> {
    let attack_db = load_database("patterns/attack.db", PatternType::Attack)?;
    let defense_db = load_database("patterns/defense.db", PatternType::Defense)?;
    let fuseki_db = load_database("patterns/fuseki.db", PatternType::Fuseki)?;
    let joseki_db = load_database("patterns/joseki.db", PatternType::Joseki)?;
    let endgame_db = load_database("patterns/endgame.db", PatternType::Endgame)?;
    
    Ok((attack_db, defense_db, fuseki_db, joseki_db, endgame_db))
}

/// Helper function to check if a pattern database exists
pub fn database_exists(path: &str) -> bool {
    Path::new(path).exists()
}
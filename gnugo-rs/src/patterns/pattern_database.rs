//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Pattern database management

use super::PatVal;
use std::collections::HashMap;

/// Pattern database structure
pub struct PatternDatabase {
    patterns: HashMap<u32, Vec<PatVal>>,
    pattern_type: super::PatternType,
    name: String,
}

impl PatternDatabase {
    /// Creates a new pattern database
    pub fn new(name: &str, pattern_type: super::PatternType) -> Self {
        PatternDatabase {
            patterns: HashMap::new(),
            pattern_type,
            name: name.to_string(),
        }
    }
    
    /// Adds a pattern to the database
    pub fn add_pattern(&mut self, pattern_id: u32, values: Vec<PatVal>) {
        self.patterns.insert(pattern_id, values);
    }
    
    /// Loads patterns from a database file
    pub fn load_from_file(&mut self, _path: &str) -> Result<(), String> {
        // In a real implementation, this would read from a .db file
        // For now, we'll just return Ok(()) as a placeholder
        Ok(())
    }
    
    /// Saves patterns to a database file
    pub fn save_to_file(&self, _path: &str) -> Result<(), String> {
        // In a real implementation, this would write to a .db file
        // For now, we'll just return Ok(()) as a placeholder
        Ok(())
    }
    
    /// Gets the pattern values for a given pattern ID
    pub fn get_pattern_values(&self, pattern_id: u32) -> Option<&Vec<PatVal>> {
        self.patterns.get(&pattern_id)
    }
    
    /// Gets the number of patterns in the database
    pub fn get_pattern_count(&self) -> usize {
        self.patterns.len()
    }
    
    /// Gets the pattern type
    pub fn get_pattern_type(&self) -> super::PatternType {
        self.pattern_type
    }
    
    /// Gets the database name
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    /// Gets all patterns in the database
    pub fn get_patterns(&self) -> &HashMap<u32, Vec<PatVal>> {
        &self.patterns
    }
}

/// Predefined pattern databases
pub struct PatternDatabases {
    attack_db: PatternDatabase,
    defense_db: PatternDatabase,
    fuseki_db: PatternDatabase,
    joseki_db: PatternDatabase,
    endgame_db: PatternDatabase,
}

impl PatternDatabases {
    /// Creates a new set of pattern databases
    pub fn new() -> Self {
        PatternDatabases {
            attack_db: PatternDatabase::new("attack", super::PatternType::Attack),
            defense_db: PatternDatabase::new("defense", super::PatternType::Defense),
            fuseki_db: PatternDatabase::new("fuseki", super::PatternType::Fuseki),
            joseki_db: PatternDatabase::new("joseki", super::PatternType::Joseki),
            endgame_db: PatternDatabase::new("endgame", super::PatternType::Endgame),
        }
    }
    
    /// Loads all pattern databases
    pub fn load_all(&mut self) -> Result<(), String> {
        self.attack_db.load_from_file("patterns/attack.db")?;
        self.defense_db.load_from_file("patterns/defense.db")?;
        self.fuseki_db.load_from_file("patterns/fuseki.db")?;
        self.joseki_db.load_from_file("patterns/joseki.db")?;
        self.endgame_db.load_from_file("patterns/endgame.db")?;
        Ok(())
    }
    
    /// Gets the attack pattern database
    pub fn get_attack_db(&self) -> &PatternDatabase {
        &self.attack_db
    }
    
    /// Gets the defense pattern database
    pub fn get_defense_db(&self) -> &PatternDatabase {
        &self.defense_db
    }
    
    /// Gets the fuseki pattern database
    pub fn get_fuseki_db(&self) -> &PatternDatabase {
        &self.fuseki_db
    }
    
    /// Gets the joseki pattern database
    pub fn get_joseki_db(&self) -> &PatternDatabase {
        &self.joseki_db
    }
    
    /// Gets the endgame pattern database
    pub fn get_endgame_db(&self) -> &PatternDatabase {
        &self.endgame_db
    }
}
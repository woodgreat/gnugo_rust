//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Pattern database management

/// Pattern database for storing and retrieving Go patterns
pub struct PatternDatabase {
    /// Database of patterns indexed by type
    patterns: std::collections::HashMap<String, Vec<PatternEntry>>,
}

/// Entry in the pattern database
#[derive(Debug, Clone)]
pub struct PatternEntry {
    /// Pattern identifier
    pub id: String,
    /// Pattern data
    pub data: Vec<u8>,
    /// Pattern metadata
    pub metadata: PatternMetadata,
}

/// Metadata for patterns
#[derive(Debug, Clone)]
pub struct PatternMetadata {
    /// Pattern type
    pub pattern_type: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub modified_at: u64,
    /// Pattern difficulty level
    pub difficulty: u32,
}

impl PatternDatabase {
    /// Creates a new pattern database
    pub fn new() -> Self {
        PatternDatabase {
            patterns: std::collections::HashMap::new(),
        }
    }
    
    /// Inserts a pattern into the database
    pub fn insert_pattern(&mut self, entry: PatternEntry) {
        self.patterns
            .entry(entry.metadata.pattern_type.clone())
            .or_insert_with(Vec::new)
            .push(entry);
    }
    
    /// Retrieves patterns of a specific type
    pub fn get_patterns_by_type(&self, pattern_type: &str) -> Option<&Vec<PatternEntry>> {
        self.patterns.get(pattern_type)
    }
    
    /// Updates a pattern in the database
    pub fn update_pattern(&mut self, id: &str, new_data: Vec<u8>) -> bool {
        for (_, patterns) in self.patterns.iter_mut() {
            for entry in patterns.iter_mut() {
                if entry.id == id {
                    entry.data = new_data;
                    entry.metadata.modified_at = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    return true;
                }
            }
        }
        false
    }
}
gnugo-rs/
├── ref/                # Original C code baseline (read-only reference)
├── doc/                # Documentation (inherited from gnugo)
├── playground/         # Experimental sandbox
└── gnugo_rs/           # Main codebase
    ├── Cargo.toml      # Project configuration
    ├── COPYING         # GPLv3 license
    ├── AUTHORS         # Developer list (wood&zulu_ai)
    ├── commit_template.txt # GNU standard commit template
    └── src/
        ├── main.rs     # Entry point
        ├── lib.rs      # Module exports
        ├── engine/     # Core engine (corresponds to original engine/)
        │   ├── mod.rs
        │   ├── board.rs
        │   ├── game.rs
        │   ├── move_generation.rs
        │   └── evaluation.rs
        └── patterns/   # Pattern matching (to be developed)
            ├── mod.rs
            ├── pattern_matching.rs
            └── pattern_database.rs
//! Manual test for ko rule functionality

use super::*;

/// Test ko rule manually
pub fn test_ko_rule() {
    println!("Testing Ko Rule...");
    
    let mut board = Board::new(9);
    
    // Create a basic ko situation
    // Setup: black captures white single stone
    board.set_stone(1, 2, Stone::White);
    board.set_stone(1, 1, Stone::Black);
    board.set_stone(2, 1, Stone::Black);
    board.set_stone(2, 2, Stone::White);
    board.set_stone(2, 3, Stone::Black);
    board.set_stone(1, 3, Stone::White);
    
    println!("Initial setup completed");
    
    // Black captures white at (1,2)
    match board.place_stone(1, 2, Stone::Black) {
        Ok(()) => println!("✓ Black captured white stone at (1,2)"),
        Err(e) => println!("✗ Black capture failed: {}", e),
    }
    
    // Check ko point
    match board.get_ko_point() {
        Some((x, y)) => println!("✓ Ko point set at ({},{})", x, y),
        None => println!("✗ Ko point not set"),
    }
    
    // White tries to immediately recapture
    match board.place_stone(2, 2, Stone::White) {
        Ok(()) => println!("✗ White recapture should have failed!"),
        Err(e) => println!("✓ White recapture correctly blocked: {}", e),
    }
    
    println!("Ko rule test completed!");
}
//! Copyright (C) 2026 wood&zulu_ai
//! License: GPL-3.0-or-later

//! Integration tests for the GNU Go Rust implementation

#[cfg(test)]
mod tests {
    use gnugo_rs::engine::board::Board;
    use gnugo_rs::engine::board::Stone;
    use gnugo_rs::engine::game::Game;
    
    #[test]
    fn test_board_creation() {
        let board = Board::new(9);
        // Test that board was created with correct size
        assert_eq!(board.size(), 9);
        // Using 1-based coordinates: (1, 1) is top-left
        assert_eq!(board.get_stone(1, 1), Stone::Empty);
    }
    
    #[test]
    fn test_board_placement() {
        let mut board = Board::new(9);
        // Using 1-based coordinates: (1, 1) is the top-left corner
        let result = board.place_stone(1, 1, Stone::Black);
        assert!(result.is_ok());
        assert_eq!(board.get_stone(1, 1), Stone::Black);
    }
    
    #[test]
    fn test_game_creation() {
        let game = Game::new(13);
        // Test that game was created with correct board size
        assert_eq!(game.current_player, true); // Black starts
    }
    
    #[test]
    fn test_game_move() {
        let mut game = Game::new(9);
        // Using 1-based coordinates: (5, 5) is the center of a 9x9 board
        let result = game.make_move(5, 5);
        assert!(result.is_ok());
        assert_eq!(game.current_player, false); // White's turn now
    }
    
    #[test]
    fn test_undo_move() {
        let mut game = Game::new(9);
        // Using 1-based coordinates
        game.make_move(5, 5).unwrap();
        let result = game.undo_move();
        assert!(result.is_some());
    }
}

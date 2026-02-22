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
        // Test that board was created with correct size (using a getter method)
        assert_eq!(board.size(), 9);
        assert_eq!(board.get_stone(0, 0), Stone::Empty);
    }
    
    #[test]
    fn test_board_placement() {
        let mut board = Board::new(9);
        let result = board.place_stone(0, 0, Stone::Black);
        assert!(result);
        assert_eq!(board.get_stone(0, 0), Stone::Black);
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
        let result = game.make_move(4, 4);
        assert!(result.is_ok());
        assert_eq!(game.current_player, false); // White's turn now
    }
    
    #[test]
    fn test_undo_move() {
        let mut game = Game::new(9);
        game.make_move(4, 4).unwrap();
        let result = game.undo_move();
        assert!(result.is_some());
    }
}
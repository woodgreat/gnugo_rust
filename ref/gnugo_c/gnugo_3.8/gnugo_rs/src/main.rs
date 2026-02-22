use gnugo_rs::engine::board::Board;

fn main() {
    let board = Board::new(19);
    println!("Initialized {}x{} board", board.size, board.size);
}
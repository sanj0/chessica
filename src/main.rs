pub mod chess;

use chess::{Board, Piece};

fn main() -> Result<(), String> {
    let board = Board::parse_fen(Board::STARTING_FEN)?;
    println!("{board}");
    Ok(())
}

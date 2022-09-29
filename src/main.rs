#[macro_use]
extern crate lazy_static;

pub mod ai;
pub mod chess;
pub mod move_gen;
pub mod baked_moves;

use chess::{Board, CastleKind, Move, Piece};

fn main() -> Result<(), String> {
    let mut board = Board::parse_fen(Board::STARTING_FEN)?;
    loop {
        use std::time::{SystemTime, UNIX_EPOCH};
        println!("{board}");
        let _ = std::io::stdin().read_line(&mut String::new());
        let moves = board.gen_pseudo_legal(board.turn);
        println!("{} pseudo legal moves ...", moves.len());
        let m = moves.get(
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize
            % moves.len()).unwrap();
        println!("playing {m:?}");
        board.play(m);
    }
    Ok(())
}

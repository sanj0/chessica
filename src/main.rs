pub mod chess;
pub mod fen;
pub mod r#move;

fn main() -> Result<(), String> {
    let board = fen::parse_board(fen::STARTING_FEN)?;
    println!("{board}");
    Ok(())
}

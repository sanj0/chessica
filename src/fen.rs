use crate::chess::{Board, Piece};

pub const FEN_WHITE: char = 'w';
pub const FEN_BLACK: char = 'b';
pub const FEN_NEW_RANK: char = '/';
pub const FEN_WHITE_PAWN: char = 'P';
pub const FEN_WHITE_KNIGHT: char = 'N';
pub const FEN_WHITE_BISHOP: char = 'B';
pub const FEN_WHITE_ROOK: char = 'R';
pub const FEN_WHITE_QUEEN: char = 'Q';
pub const FEN_WHITE_KING: char = 'K';
pub const FEN_BLACK_PAWN: char = 'p';
pub const FEN_BLACK_KNIGHT: char = 'n';
pub const FEN_BLACK_BISHOP: char = 'b';
pub const FEN_BLACK_ROOK: char = 'r';
pub const FEN_BLACK_QUEEN: char = 'q';
pub const FEN_BLACK_KING: char = 'k';

pub const STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse_board(fen: &str) -> Result<Board, String> {
    // would only need to split at spaces per definition but what gives
    let mut fields = fen.split_whitespace();
    let pos_field = fields
        .next()
        .ok_or_else(|| String::from("non-empty fen string expected"))?;
    let turn_field = fields
        .next()
        .ok_or_else(|| String::from("fen string expected to have at least first two fields"))?;
    let mut pieces = [Piece::from(Piece::NONE); 64];
    let mut index: u32 = 0;
    for c in pos_field.chars() {
        pieces[index as usize] = Piece::from(match c {
            FEN_WHITE_PAWN => Piece::WHITE | Piece::PAWN,
            FEN_WHITE_KNIGHT => Piece::WHITE | Piece::KNIGHT,
            FEN_WHITE_BISHOP => Piece::WHITE | Piece::BISHOP,
            FEN_WHITE_ROOK => Piece::WHITE | Piece::ROOK,
            FEN_WHITE_QUEEN => Piece::WHITE | Piece::QUEEN,
            FEN_WHITE_KING => Piece::WHITE | Piece::KING,
            FEN_BLACK_PAWN => Piece::BLACK | Piece::PAWN,
            FEN_BLACK_KNIGHT => Piece::BLACK | Piece::KNIGHT,
            FEN_BLACK_BISHOP => Piece::BLACK | Piece::BISHOP,
            FEN_BLACK_ROOK => Piece::BLACK | Piece::ROOK,
            FEN_BLACK_QUEEN => Piece::BLACK | Piece::QUEEN,
            FEN_BLACK_KING => Piece::BLACK | Piece::KING,
            d if d.is_digit(Board::NUM_FILES + 1) => {
                let n = c.to_digit(Board::NUM_FILES + 1).unwrap();
                if n > Board::NUM_FILES - index % Board::NUM_FILES {
                    return Err(format!(
                        "there aren't {n} empty fields left in the current rank!"
                    ));
                } else {
                    index += n;
                    continue;
                }
            }
            FEN_NEW_RANK => {
                if index % 8 != 0 {
                    return Err(format!("rank not yet done at rank delimiter {c}"));
                }
                continue;
            }
            _ => return Err(format!("unexpected character {c} in position field!")),
        });
        index += 1;
    }
    if turn_field.len() != 1 {
        return Err(format!(
            "illegal second (turn) field '{turn_field}'; one of {} or {} expected",
            FEN_WHITE, FEN_BLACK
        ));
    }
    let turn = match turn_field.chars().next().unwrap() {
        FEN_WHITE => Piece::WHITE,
        FEN_BLACK => Piece::BLACK,
        _ => {
            return Err(format!(
                "expected {} or {} as second field!",
                FEN_WHITE, FEN_BLACK
            ))
        }
    };

    Ok(Board::new(pieces, turn))
}

pub fn fen_char(p: &Piece) -> char {
    let mut piece = match p.inner() & !(Piece::WHITE | Piece::BLACK) /*"removes" the color bits*/ {
        Piece::PAWN => FEN_BLACK_PAWN,
        Piece::KNIGHT => FEN_BLACK_KNIGHT,
        Piece::BISHOP => FEN_BLACK_BISHOP,
        Piece::ROOK => FEN_BLACK_ROOK,
        Piece::QUEEN => FEN_BLACK_QUEEN,
        Piece::KING => FEN_BLACK_KING,
        Piece::NONE => ' ',
        _ => panic!("double flagged piece {}", p.inner()),
    };
    if p.is(Piece::WHITE) {
        piece.make_ascii_uppercase()
    }
    piece
}

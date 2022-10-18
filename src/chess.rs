use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Piece(u16);

pub struct Board {
    /// the pieces on the board, stored using big endian for the ranks
    /// (0-7 = rank 8) and little endian for the files (0 = 1, 7 = 8)
    pub pieces: [Piece; 64],
    /// who's turn is it?
    /// Piece::BLACK or Piece::WHITE
    pub turn: u16,
    /// the target square for en passant
    pub en_passant_target: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum CastleKind {
    WhiteKingSide,
    WhiteQueenSide,
    BlackKingSide,
    BlackQueenSide,
}

/// A move is an always assumed to be valid modification
/// of the board.
/// I. e. A sliding move is always playd as an AB move
/// without prior checking if the move "slides" through pieces
/// or leaves its "lane" entirely.
#[derive(Debug, Clone)]
pub enum Move {
    /// A "normal" from square a to square b
    AB { a: usize, b: usize },
    /// A castle move with the given kind
    Castle { kind: CastleKind },
    /// An en passant capture where the pawn moves from a to b
    /// and captures the piece on the given square
    EnPassant { a: usize, b: usize, capture: usize },
    /// A promotion move from a to b into the given target piece
    Promotion { a: usize, b: usize, target: Piece },
}

impl Board {
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

    pub const STARTING_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub const NUM_FILES: u32 = 8; // = "width"
    pub const NUM_RANKS: u32 = 8; // = "height"
    pub const BLACK_BACK_RANK: RangeInclusive<usize> = 0..=7;
    pub const BLACK_PAWN_RANK: RangeInclusive<usize> = 8..=15;
    pub const WHITE_PAWN_RANK: RangeInclusive<usize> = 48..=55;
    pub const WHITE_BACK_RANK: RangeInclusive<usize> = 56..=63;

    /// plays the given move and sets self.turn to the opposite color
    pub fn play(&mut self, m: &Move) {
        m.play(self);
        self.turn = match self.turn {
            Piece::BLACK => Piece::WHITE,
            Piece::WHITE => Piece::BLACK,
            no_color => panic!("illegal board state {no_color} is not a color"),
        }
    }

    pub fn rank_of(pos: usize) -> usize {
        pos / 8
    }

    pub fn file_of(pos: usize) -> usize {
        pos % 8
    }

    pub fn square_index(file: usize, rank: usize) -> usize {
        rank * 8 + file
    }

    pub fn parse_fen(fen: &str) -> Result<Self, String> {
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
                Self::FEN_WHITE_PAWN => Piece::WHITE | Piece::PAWN,
                Self::FEN_WHITE_KNIGHT => Piece::WHITE | Piece::KNIGHT,
                Self::FEN_WHITE_BISHOP => Piece::WHITE | Piece::BISHOP,
                Self::FEN_WHITE_ROOK => Piece::WHITE | Piece::ROOK,
                Self::FEN_WHITE_QUEEN => Piece::WHITE | Piece::QUEEN,
                Self::FEN_WHITE_KING => Piece::WHITE | Piece::KING,
                Self::FEN_BLACK_PAWN => Piece::BLACK | Piece::PAWN,
                Self::FEN_BLACK_KNIGHT => Piece::BLACK | Piece::KNIGHT,
                Self::FEN_BLACK_BISHOP => Piece::BLACK | Piece::BISHOP,
                Self::FEN_BLACK_ROOK => Piece::BLACK | Piece::ROOK,
                Self::FEN_BLACK_QUEEN => Piece::BLACK | Piece::QUEEN,
                Self::FEN_BLACK_KING => Piece::BLACK | Piece::KING,
                d if d.is_digit(Self::NUM_FILES + 1) => {
                    let n = c.to_digit(Self::NUM_FILES + 1).unwrap();
                    if n > Self::NUM_FILES - index % Self::NUM_FILES {
                        return Err(format!(
                            "there aren't {n} empty fields left in the current rank!"
                        ));
                    } else {
                        index += n;
                        continue;
                    }
                }
                Self::FEN_NEW_RANK => {
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
                Self::FEN_WHITE,
                Self::FEN_BLACK
            ));
        }
        let turn = match turn_field.chars().next().unwrap() {
            Self::FEN_WHITE => Piece::WHITE,
            Self::FEN_BLACK => Piece::BLACK,
            _ => {
                return Err(format!(
                    "expected {} or {} as second field!",
                    Self::FEN_WHITE,
                    Self::FEN_BLACK
                ))
            }
        };

        Ok(Self {
            pieces,
            turn,
            en_passant_target: None,
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let mut rank = 8;
        for (i, p) in self.pieces.iter().enumerate() {
            if i % 8 == 0 {
                writeln!(f)?;
                write!(f, "{} ", rank)?;
                rank -= 1;
            }
            if p.has(Piece::NONE) {
                write!(f, " . ")?;
            } else {
                write!(f, " {} ", char::from(p))?;
            }
        }
        write!(f, "\n   a  b  c  d  e  f  g  h")?;
        Ok(())
    }
}

impl std::ops::Index<usize> for Board {
    type Output = Piece;
    fn index(&self, i: usize) -> &Piece {
        &self.pieces[i]
    }
}

impl std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.pieces[i]
    }
}

impl From<&Piece> for char {
    fn from(p: &Piece) -> Self {
        let mut piece = match p.0 & !(Piece::WHITE | Piece::BLACK) /*"removes" the color bits*/ {
            Piece::PAWN => Board::FEN_BLACK_PAWN,
            Piece::KNIGHT => Board::FEN_BLACK_KNIGHT,
            Piece::BISHOP => Board::FEN_BLACK_BISHOP,
            Piece::ROOK => Board::FEN_BLACK_ROOK,
            Piece::QUEEN => Board::FEN_BLACK_QUEEN,
            Piece::KING => Board::FEN_BLACK_KING,
            Piece::NONE => ' ',
            _ => panic!("double flagged piece {}", p.0),
        };
        if p.has(Piece::WHITE) {
            piece.make_ascii_uppercase()
        }
        piece
    }
}

impl Piece {
    pub const NONE: u16 = 1 << 0;
    pub const WHITE: u16 = 1 << 1;
    pub const BLACK: u16 = 1 << 2;
    pub const COLOR_MASK: u16 = Self::WHITE | Self::BLACK;
    pub const PAWN: u16 = 1 << 3;
    pub const KNIGHT: u16 = 1 << 4;
    pub const BISHOP: u16 = 1 << 5;
    pub const ROOK: u16 = 1 << 6;
    pub const QUEEN: u16 = 1 << 7;
    pub const KING: u16 = 1 << 8;
    pub const KIND_MASK: u16 = !Self::COLOR_MASK;
    pub const NO_PIECE: Piece = Piece(Piece::NONE);

    pub fn new_unchecked(color: u16, kind: u16) -> Self {
        Self(color | kind)
    }

    pub fn none() -> Self {
        Self(Self::NONE)
    }

    pub fn has(&self, flag: u16) -> bool {
        self.0 & flag != 0
    }

    pub fn is(&self, color: u16, kind: u16) -> bool {
        self.0 & (color | kind) != 0
    }

    pub fn color(&self) -> u16 {
        self.0 & Self::COLOR_MASK
    }

    pub fn kind(&self) -> u16 {
        self.0 & Self::KIND_MASK
    }
}

impl Move {
    pub fn new_ab(a: usize, b: usize) -> Self {
        Self::AB { a, b }
    }

    pub fn new_castle(kind: CastleKind) -> Self {
        Self::Castle { kind }
    }

    pub fn new_en_passant(a: usize, b: usize, capture: usize) -> Self {
        Self::EnPassant { a, b, capture }
    }

    pub fn play(&self, board: &mut Board) {
        macro_rules! ab_move {
            ($a:ident, $b:ident) => {
                let a = *$a;
                board[*$b] = board[a];
                board[a] = Piece::NO_PIECE;
            };
        }
        match self {
            Self::AB { a, b } => {
                ab_move!(a, b);
            }
            Self::Castle { kind } => kind.play(board),
            Self::EnPassant { a, b, capture } => {
                ab_move!(a, b);
                board[*capture] = Piece::NO_PIECE;
            }
            Self::Promotion { a, b, target } => {
                board[*a] = Piece::NO_PIECE;
                board[*b] = *target;
            }
        }
    }

    pub fn capture(&self, board: &Board) -> Option<Piece> {
        match self {
            Self::AB { a: _, b } | Self::Promotion { a: _, b, target: _ } => {
                let cap = board[*b];
                if cap.has(Piece::NONE) {
                    None
                } else {
                    Some(cap)
                }
            }
            Self::Castle { kind: _ } => None,
            Self::EnPassant {
                a: _,
                b: _,
                capture,
            } => Some(board[*capture]),
        }
    }
}

impl CastleKind {
    const WHITE_KING_KING_MOVE: Move = Move::AB { a: 60, b: 62 };
    const WHITE_KING_ROOK_MOVE: Move = Move::AB { a: 63, b: 61 };

    const WHITE_QUEEN_KING_MOVE: Move = Move::AB { a: 60, b: 58 };
    const WHITE_QUEEN_ROOK_MOVE: Move = Move::AB { a: 56, b: 59 };

    const BLACK_KING_KING_MOVE: Move = Move::AB { a: 4, b: 6 };
    const BLACK_KING_ROOK_MOVE: Move = Move::AB { a: 7, b: 5 };

    const BLACK_QUEEN_KING_MOVE: Move = Move::AB { a: 4, b: 2 };
    const BLACK_QUEEN_ROOK_MOVE: Move = Move::AB { a: 0, b: 3 };

    pub fn play(&self, board: &mut Board) {
        match self {
            Self::WhiteKingSide => {
                Self::WHITE_KING_KING_MOVE.play(board);
                Self::WHITE_KING_ROOK_MOVE.play(board);
            }
            Self::WhiteQueenSide => {
                Self::WHITE_QUEEN_KING_MOVE.play(board);
                Self::WHITE_QUEEN_ROOK_MOVE.play(board);
            }
            Self::BlackKingSide => {
                Self::BLACK_KING_KING_MOVE.play(board);
                Self::BLACK_KING_ROOK_MOVE.play(board);
            }
            Self::BlackQueenSide => {
                Self::BLACK_QUEEN_KING_MOVE.play(board);
                Self::BLACK_QUEEN_ROOK_MOVE.play(board);
            }
        }
    }
}

impl From<u16> for Piece {
    fn from(n: u16) -> Self {
        Piece(n)
    }
}

impl From<Piece> for u16 {
    fn from(p: Piece) -> Self {
        p.0
    }
}

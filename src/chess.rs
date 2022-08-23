use std::fmt::{Display, Formatter};

pub struct Board {
    /// the pieces on the board, starting at rank 8 file a, going to rank 8 file h
    /// and ending eventually at rank 1 file h
    pieces: [Piece; 64],
    /// who's turn is it?
    /// Piece::BLACK or Piece::WHITE
    turn: u16,
}

#[derive(Copy, Clone)]
pub struct Piece(u16);

impl Board {
    pub const FEN_WHITE: char = 'w';
    pub const FEN_BLACK: char = 'b';
    pub const FEN_NEW_RANK: char = '/';
    pub const FEN_WHITE_PAWN:   char = 'P';
    pub const FEN_WHITE_KNIGHT: char = 'N';
    pub const FEN_WHITE_BISHOP: char = 'B';
    pub const FEN_WHITE_ROOK:   char = 'R';
    pub const FEN_WHITE_QUEEN:  char = 'Q';
    pub const FEN_WHITE_KING:   char = 'K';
    pub const FEN_BLACK_PAWN:   char = 'p';
    pub const FEN_BLACK_KNIGHT: char = 'n';
    pub const FEN_BLACK_BISHOP: char = 'b';
    pub const FEN_BLACK_ROOK:   char = 'r';
    pub const FEN_BLACK_QUEEN:  char = 'q';
    pub const FEN_BLACK_KING:   char = 'k';

    pub const STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub const NUM_FILES: u32 = 8; // = "width"
    pub const NUM_RANKS: u32 = 8; // = "height"

    pub fn parse_fen(fen: &str) -> Result<Self, String> {
        // would only need to split at spaces per definition but what gives
        let mut fields = fen.split_whitespace();
        let pos_field = fields.next().ok_or_else(|| String::from("non-empty fen string expected"))?;
        let turn_field = fields.next().ok_or_else(|| String::from("fen string expected to have at least first two fields"))?;
        let mut pieces = [Piece::from(Piece::NONE); 64];
        let mut index: u32 = 0;
        for c in pos_field.chars() {
            pieces[index as usize] = Piece::from(match c {
                Self::FEN_WHITE_PAWN      => Piece::WHITE | Piece::PAWN,
                Self::FEN_WHITE_KNIGHT    => Piece::WHITE | Piece::KNIGHT,
                Self::FEN_WHITE_BISHOP    => Piece::WHITE | Piece::BISHOP,
                Self::FEN_WHITE_ROOK      => Piece::WHITE | Piece::ROOK,
                Self::FEN_WHITE_QUEEN     => Piece::WHITE | Piece::QUEEN,
                Self::FEN_WHITE_KING      => Piece::WHITE | Piece::KING,
                Self::FEN_BLACK_PAWN      => Piece::BLACK | Piece::PAWN,
                Self::FEN_BLACK_KNIGHT    => Piece::BLACK | Piece::KNIGHT,
                Self::FEN_BLACK_BISHOP    => Piece::BLACK | Piece::BISHOP,
                Self::FEN_BLACK_ROOK      => Piece::BLACK | Piece::ROOK,
                Self::FEN_BLACK_QUEEN     => Piece::BLACK | Piece::QUEEN,
                Self::FEN_BLACK_KING      => Piece::BLACK | Piece::KING,
                d if d.is_digit(Self::NUM_FILES + 1) => {
                    let n = c.to_digit(Self::NUM_FILES + 1).unwrap();
                    if n > Self::NUM_FILES - index % Self::NUM_FILES {
                        return Err(format!("there aren't {n} empty fields left in the current rank!"));
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
            return Err(format!("illegal second (turn) field '{turn_field}'; one of {} or {} expected", Self::FEN_WHITE, Self::FEN_BLACK));
        }
        let turn = match turn_field.chars().next().unwrap() {
            Self::FEN_WHITE => Piece::WHITE,
            Self::FEN_BLACK => Piece::BLACK,
            _ => return Err(format!("expected {} or {} as second field!", Self::FEN_WHITE, Self::FEN_BLACK)),
        };

        Ok(Self {
            pieces,
            turn,
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<>) -> Result<(), std::fmt::Error> {
        for (i, p) in self.pieces.iter().enumerate() {
            if i % 8 == 0 {
                writeln!(f)?;
            }
            if p.is(Piece::NONE) {
                write!(f, " . ")?;
            } else {
                write!(f, " {} ", char::from(p))?;
            }
        }
        Ok(())
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
        if p.is(Piece::WHITE) {
            piece.make_ascii_uppercase()
        }
        piece
    }
}

impl Piece {
    pub const NONE:     u16 = 1 << 0;
    pub const WHITE:    u16 = 1 << 1;
    pub const BLACK:    u16 = 1 << 2;
    pub const PAWN:     u16 = 1 << 3;
    pub const KNIGHT:   u16 = 1 << 4;
    pub const BISHOP:   u16 = 1 << 5;
    pub const ROOK:     u16 = 1 << 6;
    pub const QUEEN:    u16 = 1 << 7;
    pub const KING:     u16 = 1 << 8;

    pub fn new_unchecked(color: u16, kind: u16) -> Self {
        Self(color | kind)
    }

    pub fn is(&self, flag: u16) -> bool {
        self.0 & flag != 0
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


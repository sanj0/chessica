use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Board {
    /// the pieces on the board, starting at rank 8 file a, going to rank 8 file h
    /// and ending eventually at rank 1 file h
    pieces: [Piece; 64],
    /// who's turn is it?
    /// Piece::BLACK or Piece::WHITE
    turn: u16,
    castle_rights: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece(u16);

impl Board {
    pub const NUM_FILES: u32 = 8; // = "width"
    pub const NUM_RANKS: u32 = 8; // = "height"

    pub fn new(pieces: [Piece; 64], turn: u16, castle_rights: u8) -> Self {
        Self {
            pieces,
            turn,
            castle_rights,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for (i, p) in self.pieces.iter().enumerate() {
            if i % 8 == 0 {
                writeln!(f)?;
            }
            if p.is(Piece::NONE) {
                write!(f, " . ")?;
            } else {
                write!(f, " {} ", crate::fen::fen_char(p))?;
            }
        }
        Ok(())
    }
}

impl Piece {
    pub const NONE: u16 = 1 << 0;
    pub const WHITE: u16 = 1 << 1;
    pub const BLACK: u16 = 1 << 2;
    pub const PAWN: u16 = 1 << 3;
    pub const KNIGHT: u16 = 1 << 4;
    pub const BISHOP: u16 = 1 << 5;
    pub const ROOK: u16 = 1 << 6;
    pub const QUEEN: u16 = 1 << 7;
    pub const KING: u16 = 1 << 8;

    pub fn new_unchecked(color: u16, kind: u16) -> Self {
        Self(color | kind)
    }

    pub fn is(&self, flag: u16) -> bool {
        self.0 & flag != 0
    }

    pub fn inner(&self) -> u16 {
        self.0
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

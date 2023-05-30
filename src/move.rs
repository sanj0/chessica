#[derive(Copy, Clone, Debug)]
pub enum Move {
    EnPassant {
        from: usize,
        to: usize,
        capture: usize,
    },
    AB {
        from: usize,
        to: usize,
    },
    Castle {
        ty: CastleType,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum CastleType {
    BlackLong,
    BlackShort,
    WhiteLong,
    WhiteShort,
}

impl CastleType {
    pub const BIT_BLACK_LONG   : u8 = 0b0001;
    pub const BIT_BLACK_SHORT  : u8 = 0b0010;
    pub const BIT_WHITE_LONG   : u8 = 0b0100;
    pub const BIT_WHITE_SHORT  : u8 = 0b1000;
    pub fn get_bit(self) -> u8 {
        match self {
            Self::BlackLong => Self::BIT_BLACK_LONG,
            Self::BlackShort => Self::BIT_BLACK_SHORT,
            Self::WhiteLong => Self::BIT_WHITE_LONG,
            Self::WhiteShort => Self::BIT_WHITE_SHORT,
        }
    }
}


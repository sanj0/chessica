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

pub enum CastleType {
    BlackLong,
    BlackShort,
    WhiteLong,
    WhiteShort,
}

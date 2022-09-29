use crate::chess::*;

impl Board {
    pub fn rate(&self) -> f32 {
        let mut rating = 0_f32;
        for p in self.pieces {
            if p.has(Piece::NONE) {
                continue;
            }
            if p.color() == Piece::WHITE {
                rating += 1.0;
            } else {
                rating -= 1.0;
            }
        }
        rating
    }
}

use crate::baked_moves::*;
use crate::chess::*;

/// Generates all pseudo legal moves for a given piece on the given board.
/// panics when piece.kind() is neither of eight expected values
fn gen_pseudo_legal_for_piece(
    piece: &Piece,
    color: u16,
    pos: usize,
    board: &Board,
    moves: &mut Vec<Move>,
) {
    /// code to generate pawn moves in both ways
    /// panicks if the supposed pawn is in either back rank of the board
    macro_rules! pawn_gen {
        ($offset_op:tt, $rank:path) => {
            assert!(
                !(Board::WHITE_BACK_RANK.contains(&pos)
                    || Board::BLACK_BACK_RANK.contains(&pos)),
                    "a pawn is in a back rank!"
                );
            // single advance
            let mut target = pos $offset_op Board::NUM_FILES as usize;
            if board[target] == Piece::NO_PIECE {
                moves.push(Move::new_ab(pos, target));
            }
            // double advance
            if $rank.contains(&pos) {
                target = target $offset_op Board::NUM_FILES as usize;
                if board[target] == Piece::NO_PIECE {
                    moves.push(Move::new_ab(pos, target));
                }
            }
            // captures
            // a pawn never exists on the backrank so no need to check for overflow there
            if pos % 8 != 0 {
                let cap = pos - 1 $offset_op Board::NUM_FILES as usize;
                if (board[cap] != Piece::NO_PIECE && board[cap].color() != color)
                    || board.en_passant_target.map_or(false, |eps| cap == eps) {
                        moves.push(Move::new_ab(pos, cap));
                }
            }
            if pos % 8 != 7 {
                let cap = pos + 1 $offset_op Board::NUM_FILES as usize;
                if (board[cap] != Piece::NO_PIECE && board[cap].color() != color)
                    || board.en_passant_target.map_or(false, |eps| cap == eps) {
                        moves.push(Move::new_ab(pos, cap));
                }
            }
            // todo: promotions
        }
    }
    match piece.kind() {
        Piece::PAWN => {
            if color == Piece::WHITE {
                pawn_gen!(-, Board::WHITE_PAWN_RANK);
            } else {
                pawn_gen!(+, Board::BLACK_PAWN_RANK);
            }
        }
        Piece::KNIGHT => {
            for target in &KNIGHT_MOVES[pos] {
                if board[*target].color() != color {
                    moves.push(Move::new_ab(pos, *target));
                }
            }
        }
        Piece::BISHOP => {}
        Piece::ROOK => {
            gen_rook_moves(pos, color, board, moves);
        }
        Piece::QUEEN => {}
        Piece::KING => {}
        kind => {
            panic!("illegal board state: cannot generate moves for unknown piece {kind}");
        }
    }
}

impl Board {
    pub fn gen_pseudo_legal(&self, color: u16) -> Vec<Move> {
        let mut moves = Vec::with_capacity(35);
        for (i, p) in self.pieces.iter().enumerate() {
            if p.color() == color {
                gen_pseudo_legal_for_piece(p, color, i, self, &mut moves);
            }
        }
        moves
    }
}

fn gen_rook_moves(pos: usize, my_color: u16, board: &Board, moves: &mut Vec<Move>) {
    let rank = Board::rank_of(pos);
    let file = Board::file_of(pos);

    // moves going "upwards"
    for i in 0..(Board::NUM_RANKS as usize - rank) {
        let target = Board::square_index(file, rank + i);
        if board[target].color() != my_color {
            moves.push(Move::new_ab(pos, target));
        } else {
            break;
        }
    }
    // moves going "downwards"
    for i in 0..rank {
        let target = Board::square_index(file, rank);
        if board[target].color() != my_color {
            moves.push(Move::new_ab(pos, target));
        }
    }
    // moves going "right"
}

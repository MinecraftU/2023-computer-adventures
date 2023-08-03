use wasm_bindgen::prelude::*;
use counter::Counter;
use chess::{Board, Piece, Color};

#[wasm_bindgen(module="/client/js/output.js")]
extern {
    pub fn my_alert(s: &str);
}

fn weight_of(piece : Piece) -> f32 {
    return match piece {
        Piece::King => 200.0,
        Piece::Queen => 9.0,
        Piece::Rook => 5.0,
        Piece::Bishop => 3.0,
        Piece::Knight => 3.0,
        Piece::Pawn => 1.0,
        _ => 0.0,
    };
}

// https://www.chessprogramming.org/Evaluation
pub fn evaluate(board : &chess::Board) -> f32 { // evaluation is for black because the engine is black
    let mut score : f32 = 0.0; 

    for piece_type in chess::ALL_PIECES {
        let black_pieces = board.pieces(piece_type) & board.color_combined(Color::Black);
        let white_pieces = board.pieces(piece_type) & board.color_combined(Color::White);
        let black_piece_amount = black_pieces.popcnt() as f32;
        let white_piece_amount = white_pieces.popcnt() as f32;
        score += (black_piece_amount - white_piece_amount)*weight_of(piece_type);
    }

    score
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn default_board() {
        assert_eq!(0.0, evaluate(&Board::default()));
    }

    #[test]
    fn king_vs_king() {
        let board : Board = Board::from_str("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        assert_eq!(0.0, evaluate(&board));
    }

    #[test]
    fn endgame_with_queen_down() {
        let board : Board = Board::from_str("5k2/4pr1p/8/8/8/6PP/8/3RK1Q1 w - - 0 1").unwrap();
        assert_eq!(-9.0, evaluate(&board));
    }

    #[test]
    fn endgame_with_queen_up() {
        let board : Board = Board::from_str("5k1q/4pr1p/8/8/8/6PP/8/3RK3 w - - 0 1").unwrap();
        assert_eq!(9.0, evaluate(&board));
    }
}

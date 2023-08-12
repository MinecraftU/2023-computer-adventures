use wasm_bindgen::prelude::*;
use chess::{Piece, Color, Board, MoveGen, BoardStatus};


#[wasm_bindgen(module="/client/js/output.js")]
extern {
    pub fn my_alert(s: &str);
}

fn status(board: &Board) -> f32 {
    let status = match board.status() {
        BoardStatus::Checkmate => 1000.0,
        _ => 0.0
    };
    match board.side_to_move() {
        Color::Black => -status,
        Color::White => status
    }
}

fn weight_of(piece : Piece) -> f32 {
    return match piece {
        Piece::King => 200.0,
        Piece::Queen => 9.0,
        Piece::Rook => 5.0,
        Piece::Bishop => 3.0,
        Piece::Knight => 3.0,
        Piece::Pawn => 1.0,
    };
}

fn material(board : &Board) -> f32 {
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

fn mobility(board : &Board) -> f32 {
    let black_mobility = MoveGen::new_legal(&board).len() as f32;
    let new_board = board.null_move();
    if new_board == None {
        return 0.0;
    }
    let white_mobility = MoveGen::new_legal(&new_board.unwrap()).len() as f32;

    black_mobility - white_mobility
}

// https://www.chessprogramming.org/Evaluation
pub fn evaluate(board : &chess::Board) -> f32 { // evaluation is for black because the engine is black
    material(board) + 0.1 * mobility(board) + status(board)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use chess::Board;

    use super::*;

    #[test]
    fn default_board() {
        assert_eq!(0.0, evaluate(&Board::default().null_move().unwrap()));
    }

    #[test]
    fn king_vs_king() {
        let board : Board = Board::from_str("4k3/8/8/8/8/8/8/4K3 b - - 0 1").unwrap();
        assert_eq!(0.0, evaluate(&board));
    }

    #[test]
    fn endgame_with_queen_down() {
        let board : Board = Board::from_str("5k2/4pr1p/8/8/8/6PP/8/3RK1Q1 b - - 0 1").unwrap();
        assert_eq!(-10.0, evaluate(&board));
    }

    #[test]
    fn endgame_with_queen_up() {
        let board : Board = Board::from_str("5k1q/4pr1p/8/8/8/6PP/8/3RK3 b - - 0 1").unwrap();
        assert_eq!(9.8, evaluate(&board));
    }
}

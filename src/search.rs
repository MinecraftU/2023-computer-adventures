use wasm_bindgen::prelude::*;
use chess::{Board, ChessMove, MoveGen};
use crate::evaluate;

#[wasm_bindgen(module="/client/js/output.js")]
extern {
    pub fn my_alert(s: &str);
}

pub fn alpha_beta_max(mut alpha : f32, beta : f32, depth : i32, board : &Board) -> f32 { // black, engine
    if depth == 0 {
        return evaluate::evaluate(board);
    };
    for chess_move in MoveGen::new_legal(board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);
        let score: f32 = alpha_beta_min(alpha, beta, depth - 1, &result);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}

pub fn alpha_beta_min(alpha : f32, mut beta : f32, depth : i32, board : &Board) -> f32 {
    if depth == 0 {
        return evaluate::evaluate(board);
    };
    for chess_move in MoveGen::new_legal(board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);
        let score: f32 = alpha_beta_max(alpha, beta, depth - 1, &result);
        if score <= alpha {
            return alpha;
        }
        if score < beta {
            beta = score;
        }
    }

    return beta;
}

pub fn search(board : &Board) -> Option<ChessMove> {
    let mut best_move: Option<ChessMove> = None;
    let mut max: f32 = f32::MIN;
    // let mut move_hist = 
    for chess_move in MoveGen::new_legal(board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);
        let score: f32 = alpha_beta_min(f32::MIN, f32::MAX, 3, &result);
        if score >= max {
            max = score;
            best_move = Some(chess_move);
        }
    }
    return best_move;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use chess::Square;

    use crate::make_move;

    use super::*;

    #[test]
    fn takes_hanging_queen() {
        let board : Board = Board::from_str("rnbqkb1r/pppppppp/5n2/8/4P1Q1/8/PPPP1PPP/RNB1KBNR b KQkq - 0 1").unwrap();
        assert_eq!(search(&board).unwrap(), ChessMove::new(Square::F6, Square::G4, None));
    }
    // todo: add a test that makes sure engine doesnt throw a fit when it loses
    #[test]
    fn loses_gracefully() { // (and legally)
        let board = Board::from_str("8/8/8/1Q6/8/3B4/k7/6K1 w - - 0 1").unwrap();
        let white_move = ChessMove::new(Square::from_str("d3").unwrap(), Square::from_str("c4").unwrap(), None);
        let result = make_move(white_move, &board).unwrap();
        assert_ne!(search(&result), None);
    }
}

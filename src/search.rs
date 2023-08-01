use wasm_bindgen::prelude::*;
use chess::{Board, ChessMove, MoveGen};

use crate::evaluate;

#[wasm_bindgen(module="/client/js/output.js")]
extern {
    pub fn my_alert(s: &str);
}

pub fn maxi(depth : i32, board : &Board) -> f32 { // black, engine
    if depth == 0 {
        return evaluate::evaluate(board);
    };
    let mut max: f32 = f32::MIN;
    for chess_move in MoveGen::new_legal(&board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);
        let score: f32 = mini(depth - 1, &result);
        if score > max {
            max = score;
        }
    }
    return max;
}

pub fn mini(depth : i32, board : &Board) -> f32 {
    if depth == 0 {
        return -evaluate::evaluate(board);
    };
    let mut min: f32 = f32::MAX;
    for chess_move in MoveGen::new_legal(&board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);
        let score: f32 = maxi(depth - 1, &result);
        if score < min {
            min = score;
        }
    }
    return min;
}

pub fn search(board : &Board) -> Option<ChessMove> {
    let mut best_move: Option<ChessMove> = None;
    let mut max: f32 = f32::MIN;
    for chess_move in MoveGen::new_legal(&board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);
        let score: f32 = mini(2, &result);
        if score > max {
            max = score;
            best_move = Some(chess_move);
        }
    }
    return best_move;
}

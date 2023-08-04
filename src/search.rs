use chess::{Board, ChessMove, MoveGen};
use std::collections::HashMap;
extern crate queues;
use queues::*;
use wasm_bindgen::prelude::*;

use crate::evaluate;

#[wasm_bindgen(module = "/client/js/output.js")]
extern "C" {
    pub fn my_alert(s: &str);
}

pub fn alpha_beta_max(mut alpha: f32, beta: f32, depth: i32, board: &Board, t_table: &mut HashMap<Board, f32>, t_buf : &mut Buffer<Board>) -> f32 {
    // black, engine
    if depth == 0 {
        return evaluate::evaluate(board);
    };

    for chess_move in MoveGen::new_legal(&board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);

        // generate new score or access cached value
        {}
        let cached = t_table.contains_key(&result);
        let to_insert = if cached {
            None
        } else {
            Some(alpha_beta_min(alpha, beta, depth - 1, &result, t_table, t_buf))
        };
        let score = match to_insert {
            None => &t_table[&result],
            Some(r) => t_table.entry(result).or_insert(r),
        };

        if *score >= beta {
            return beta;
        }
        if *score > alpha {
            alpha = *score;
        }

        if to_insert != None {
            if t_buf.size() == t_buf.capacity() {
                let oldest_board_result = t_buf.remove();
                match oldest_board_result {
                    Ok(_) => {
                        t_table.remove(&oldest_board_result.unwrap());
                    },
                    Err(_) => { }
                };
            }
            match t_buf.add(result) {
                Ok(_) => (),
                Err(_) => my_alert("adding to buffer failed"),
            };
        }
    }
    return alpha;
}

pub fn alpha_beta_min(alpha: f32, mut beta: f32, depth: i32, board: &Board, t_table: &mut HashMap<Board, f32>, t_buf : &mut Buffer<Board> ) -> f32 {
    if depth == 0 {
        return evaluate::evaluate(board);
    };
    for chess_move in MoveGen::new_legal(&board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);

        // generate new score or access cached value
        let cached = t_table.contains_key(&result);
        let to_insert = if cached {
            None
        } else {
            Some(alpha_beta_max(alpha, beta, depth - 1, &result, t_table, t_buf))
        };
        let score = match to_insert {
            None => &t_table[&result],
            Some(r) => t_table.entry(result).or_insert(r),
        };

        if *score <= alpha {
            return alpha;
        }
        if *score < beta {
            beta = *score;
        }

        if to_insert != None {
            if t_buf.size() == t_buf.capacity() {
                let oldest_board_result = t_buf.remove();
                match oldest_board_result {
                    Ok(_) => {
                        t_table.remove(&oldest_board_result.unwrap());
                    },
                    Err(_) => { }
                };
            }
            match t_buf.add(result) {
                Ok(_) => (),
                Err(_) => my_alert("adding to buffer failed"),
            };
        }
    }
    return beta;
}

pub fn maxi(depth: i32, board: &Board) -> f32 {
    // black, engine
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

pub fn mini(depth: i32, board: &Board) -> f32 {
    if depth == 0 {
        return evaluate::evaluate(board);
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

pub fn search(board: &Board) -> Option<ChessMove> {
    let size : usize = 50000;
    let mut t_buf: Buffer<Board> = Buffer::new(size);
    let mut t_table: HashMap<Board, f32> = HashMap::new(); // transposition table: https://www.chessprogramming.org/Transposition_Table

    let mut best_move: Option<ChessMove> = None;
    let mut max: f32 = f32::MIN;
    for chess_move in MoveGen::new_legal(&board) {
        let mut result = *board;
        board.make_move(chess_move, &mut result);
        let score: f32 = alpha_beta_min(f32::MIN, f32::MAX, 7, &result, &mut t_table, &mut t_buf);
        if score > max {
            max = score;
            best_move = Some(chess_move);
        }
    }
    return best_move;
}

#[cfg(test)]
mod tests {
    use chess::Square;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn takes_hanging_queen() {
        let board: Board =
            Board::from_str("rnbqkb1r/pppppppp/5n2/8/4P1Q1/8/PPPP1PPP/RNB1KBNR b KQkq - 0 1")
                .unwrap();
        assert_eq!(
            search(&board).unwrap(),
            ChessMove::new(Square::F6, Square::G4, None)
        );
    }
}

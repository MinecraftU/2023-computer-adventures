//use chess::MoveGen;
//use chess::{Board, Square, Color};
//use std::str::FromStr;
//use chess::EMPTY;
use chess::{Board, ChessMove, Square, MoveGen};
use wasm_bindgen::prelude::*;
use std::str::FromStr;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref BOARD : Mutex<Board> = Mutex::new(Board::default());
}

#[wasm_bindgen(module="/client/js/output.js")]
extern {
    pub fn my_alert(s: &str);
    pub fn update_board(s: &str);
}

#[wasm_bindgen]
pub fn get_engine_move(source_str: &str, target_str: &str) { // TODO: recieves a player move and returns an FEN containing the new position with the engine move as a response
    my_alert(&format!("RUST: source: {source_str}, target: {target_str}!"));
    
    let source : Square = Square::from_str(source_str).unwrap();
    let target : Square = Square::from_str(target_str).unwrap();

    let result = make_move(source, target, &BOARD.lock().unwrap());
    let engine_move = evaluate(&result).unwrap();
    let result2 = make_move(engine_move.get_source(), engine_move.get_dest(), &result);
    *BOARD.lock().unwrap() = result2;
    update_board(&result2.to_string());
}

fn evaluate(board : &Board) -> Option<ChessMove> { // returns the first legal move based on the board (will implement algorithm to return the "best" move later)
    return MoveGen::new_legal(&board).next();
}

fn make_move(from : Square, to : Square, board : &Board) -> Board {
    let mut result = *board;
    let chess_move = ChessMove::new(from, to, None);
    if !board.legal(chess_move) {
        println!("note: illegal move made");
    }
    board.make_move(chess_move, &mut result);
    return result;
}

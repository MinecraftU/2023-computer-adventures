use chess::{Piece, Board, ChessMove, Square};
use wasm_bindgen::prelude::*;
use std::str::FromStr;
use std::sync::Mutex;

mod search;
mod evaluate;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref BOARD : Mutex<Board> = Mutex::new(Board::default());
}

#[wasm_bindgen(module="/client/js/output.js")]
extern {
    pub fn my_alert(s: &str);
}

#[wasm_bindgen]
pub fn get_engine_move(source_str: &str, target_str: &str, promotion_str: &str) -> String { // TODO: recieves a player move and returns an FEN containing the new position with the engine move as a response
    let source : Square = Square::from_str(source_str).unwrap();
    let target : Square = Square::from_str(target_str).unwrap();
    let promotion : Option<Piece> = match promotion_str {
        "n" => Some(Piece::Knight),
        "q" => Some(Piece::Queen),
        "b" => Some(Piece::Bishop),
        "r" => Some(Piece::Rook),
        _ => None,
    };

    let result = make_move(ChessMove::new(source, target, promotion), &BOARD.lock().unwrap());
    match result {
        Ok(_) => (),
        Err(_) => return "illegal move".to_string(),
    }
    let engine_move = search::search(&result.unwrap()).unwrap();
    let engine_result = make_move(engine_move, &result.unwrap());
    match engine_result {
        Ok(_) => (),
        Err(_) => panic!("engine made illegal move"),
    }
    *BOARD.lock().unwrap() = engine_result.unwrap();
    return engine_result.unwrap().to_string();
}

fn make_move(chess_move : ChessMove, board : &Board) -> Result<Board, &'static str> {
    let mut result = *board;
    if !board.legal(chess_move) {
        my_alert("RUST: illegal move made");
        return Err("illegal move made");
    }
    board.make_move(chess_move, &mut result);
    return Ok(result);
}


#[cfg(test)]
mod tests {
    use pprof;
    use std::str::FromStr;
    use pprof::protos::Message;
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn benchmark() {
        let guard = pprof::ProfilerGuard::new(10).unwrap();

        *BOARD.lock().unwrap() = Board::from_str("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 1").unwrap();
        println!("{}", get_engine_move("b1", "c3", ""));

        match guard.report().build() {
            Ok(report) => {
                let mut file = File::create("profile.pb").unwrap();
                let profile = report.pprof().unwrap();

                let mut content = Vec::new();
                profile.encode(&mut content).unwrap();
                file.write_all(&content).unwrap();

                println!("report: {}", &report);
            }
            Err(_) => {}
        }
    }
}

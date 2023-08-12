use chess::{Piece, Board, ChessMove, Square};
use wasm_bindgen::prelude::*;
use std::str::FromStr;

mod search;
mod evaluate;

#[wasm_bindgen(module="/client/js/output.js")]
extern {
    pub fn my_alert(s: &str);
}

#[wasm_bindgen]
pub fn get_engine_move(board_str : &str, source_str: &str, target_str: &str, promotion_str: &str) -> String { // TODO: recieves a player move and returns an FEN containing the new position with the engine move as a response
    let board : Board = Board::from_str(board_str).unwrap();
    let source : Square = Square::from_str(source_str).unwrap();
    let target : Square = Square::from_str(target_str).unwrap();
    let promotion : Option<Piece> = match promotion_str {
        "n" => Some(Piece::Knight),
        "q" => Some(Piece::Queen),
        "b" => Some(Piece::Bishop),
        "r" => Some(Piece::Rook),
        _ => None,
    };

    let result = make_move(ChessMove::new(source, target, promotion), &board);
    match result {
        Ok(_) => (),
        Err(_) => return "illegal move".to_string(),
    }
    let engine_move = search::search(&result.unwrap());
    if engine_move == None {
        return "no legal moves for engine".to_string();
    }
    let engine_result = make_move(engine_move.unwrap(), &result.unwrap());
    match engine_result {
        Ok(_) => (),
        Err(_) => panic!("engine made illegal move"),
    }
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
    use pprof::protos::Message;
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn benchmark() {
        let guard = pprof::ProfilerGuard::new(1000).unwrap();

        let board_fen = "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 1";
        println!("{}", get_engine_move(board_fen, "b1", "c3", ""));

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

    #[test]
    fn no_legal_moves() {
        let board_fen = "8/8/8/1Q6/8/3B4/k7/6K1 w - - 0 1";
        assert_eq!("no legal moves for engine", get_engine_move(board_fen, "d3", "c4", ""));
    }
}

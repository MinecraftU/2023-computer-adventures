//use chess::MoveGen;
//use chess::{Board, Square, Color};
//use std::str::FromStr;
//use chess::EMPTY;
use chess::{Board, BitBoard, ChessMove, Piece, Square, Color};

fn make_move(from : Square, to : Square, board : &Board) -> Board {
    let mut result = *board;
    let chess_move = ChessMove::new(from, to, None);
    if !board.legal(chess_move) {
        println!("illegal move made");
    }
    board.make_move(chess_move, &mut result);
    return result;
}

fn act_on_board(board : &Board) { // prints random stuff, change later
    println!("{}", board);
    println!("{}", board.pieces(Piece::Pawn));
}

fn main() {
    /*
    let fen = "rnbqkbnr/ppp1pppp/8/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 1";
    let board = Board::from_str(fen);
    println!("{}", board.expect("REASON").king_square(Color::White));
    //assert_eq!(board.expect("REASON").king_square(Color::White), Square::E1);
    */

    /*
    let board = Board::default();
    let mut result = Board::default();
    board.make_move(ChessMove::new(Square::D2, Square::D4, None), &mut result);

    let mut result2 = result; 
    result.make_move(ChessMove::new(Square::E2, Square::E4, None), &mut result2);
    //assert_eq!(result.side_to_move(), Color::Black);
    */
    let board = Board::default();
    act_on_board(&make_move(Square::D7, Square::D5, &make_move(Square::E2, Square::E8, &board)));

}

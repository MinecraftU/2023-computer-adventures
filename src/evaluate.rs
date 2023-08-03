use wasm_bindgen::prelude::*;
use counter::Counter;
use chess::{Board};

#[wasm_bindgen(module="/client/js/output.js")]
extern {
    pub fn my_alert(s: &str);
}

// https://www.chessprogramming.org/Evaluation
pub fn evaluate(board : &chess::Board) -> f32 {
    let mut score : f32 = 0.0; 
    let counts = board.to_string().split(" ").next().unwrap().chars().filter(|c| c.is_alphabetic()).collect::<Counter<_>>();
    for (e, count) in counts.iter() {
        let mut weight : f32 = 0.0;
        match e.to_ascii_uppercase() {
            'K' => weight = 200.0,
            'Q' => weight = 9.0,
            'R' => weight = 5.0,
            'B' => weight = 3.0,
            'N' => weight = 3.0,
            'P' => weight = 1.0,
            _ => (),
        }
        if e.is_uppercase() {
            score += weight*(*count as f32);
        } else {
            score += -weight*(*count as f32);
        }
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
    fn endgame_with_queen_up() {
        let board : Board = Board::from_str("5k2/4pr1p/8/8/8/6PP/8/3RK1Q1 w - - 0 1").unwrap();
        assert_eq!(9.0, evaluate(&board));
    }

    #[test]
    fn endgame_with_queen_down() {
        let board : Board = Board::from_str("5k1q/4pr1p/8/8/8/6PP/8/3RK3 w - - 0 1").unwrap();
        assert_eq!(-9.0, evaluate(&board));
    }
}

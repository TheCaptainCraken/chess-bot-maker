use rand::seq::SliceRandom;
use shakmaty::{Chess, Move, Position};

pub fn next_move(position: &Chess) -> Move {
    let legal_moves = position.legal_moves();

    let bot_move = legal_moves.choose(&mut rand::thread_rng()).unwrap();

    bot_move.clone()
}

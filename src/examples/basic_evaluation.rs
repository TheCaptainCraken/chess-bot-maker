use shakmaty::{Chess, Color, Move, Position, Role};

/// Returns the best move for the current position using piece count evaluation.
pub fn next_move(position: &Chess) -> Move {
    let legal_moves = position.legal_moves(); // Get all legal moves

    // Find the move that maximizes the evaluation (piece count)
    let best_move = legal_moves
        .iter()
        .max_by_key(|legal_move| {
            let new_position = position.clone().play(legal_move).expect("Move is legal");
            let evaluation = evaluate(&new_position);
            evaluation
        })
        .expect("No legal moves found");

    best_move.clone()
}

/// Evaluates the position using piece count.
fn evaluate(position: &Chess) -> i64 {
    /*
        This is using iterative folding to calculate the evaluation of the position.
        The evaluation is calculated by iterating over the pieces on the board and summing up the score of each piece.
        The score of each piece is calculated by multiplying the count of the piece by the score of the piece and then taken positively or negatively based on the color of the piece.
    */
    let score = position
        .board()
        .material()
        .zip_color()
        .iter()
        .fold(0, |acc, (color, pieces)| {
            acc + pieces.zip_role().iter().fold(0, |acc, (role, count)| {
                let score = get_score(*role)
                    * (*count as i64)
                    * is_opponent((*color).other(), position.turn()); // we have to invert the color because by playing the move we are changing the turn.
                acc + score
            })
        });

    score
}

/// Returns the score of a piece based on its role. The score is used for evaluation.
fn get_score(role: Role) -> i64 {
    match role {
        Role::Pawn => 1,
        Role::Knight => 3,
        Role::Bishop => 3,
        Role::Rook => 5,
        Role::Queen => 9,
        Role::King => 0,
    }
}

/// Returns 1 if the piece color is the same as the color of the player whose turn it is, otherwise -1.
fn is_opponent(piece_color: Color, our_color: Color) -> i64 {
    if piece_color == our_color {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_evaluate() {
        let position = Chess::default();
        let evaluation = evaluate(&position);
        assert_eq!(evaluation, 0);
    }

    #[test]
    fn test_get_score() {
        assert_eq!(get_score(Role::Pawn), 1);
        assert_eq!(get_score(Role::Knight), 3);
        assert_eq!(get_score(Role::Bishop), 3);
        assert_eq!(get_score(Role::Rook), 5);
        assert_eq!(get_score(Role::Queen), 9);
        assert_eq!(get_score(Role::King), 0);
    }

    #[test]
    fn test_is_opponent() {
        assert_eq!(is_opponent(Color::White, Color::White), 1);
        assert_eq!(is_opponent(Color::Black, Color::White), -1);
        assert_eq!(is_opponent(Color::White, Color::Black), -1);
        assert_eq!(is_opponent(Color::Black, Color::Black), 1);
    }

    #[test]
    fn test_next_move() {
        let position = Chess::default();
        let best_move = next_move(&position);
        assert!(position.legal_moves().contains(&best_move));
    }
}

# Basic evaluation

Now we are really getting started!

Currently we are:

1. Generating all the legal moves
2. Choosing a move at random

This is not exactly a good way to play chess. We can do better. We need a way to choose the best move we have.

We could try to evaluate moves based on wether the game state it creates is more or less favorable to us.
To do that we need to be able to evaluate a game state and decide if we are winning or loosing in that game state.

## Piece counting

A dead simple, easy peasy way to evaluate a position is: counting the number of pieces:

- the more pieces we have, the better
- the less pieces the enemy has, the better

We can bring this a bit further by observing tat not all pieces are created equal: a queen is much much stronger than a single pawn.
We can assign a score to each piece type, then count those pieces and multiplying for the score. We do this for all pieces, then we do the same for the opponent and subtract that number from our number.

These are the most common scores assigned to pieces:

| piece  |  score |
| - | - |
| pawn  | 1  |
| bishop | 3 |
| knight | 3 |
|rook | 5 |
| queen | 9 |
| king | 0 |

This is a very long explanation but the idea is very simple, for example:

```chess
. . . . . ♖ . .
. . . . . . . .
. ♜ . . . . . .
♙ ♙ ♔ . . . ♙ .
. . ♞ . . . ♟ .
♟ . . . . ♗ . .
. ♚ . . . . . .
. . . . . ♝ . .
```

Let's say we are white here, we have:

- 1 king
- 2 pawns
- 1 bishop
- 1 rook
- 1 knight

This makes our score: \\(1  \cdot0 + 2 \cdot 1 + 1  \cdot3 + 1 \cdot 5 + 1  \cdot 3 = 13\\). Black has:

- 1 king
- 3 pawns
- 1 bishop
- 1 rook

This makes their score: \\( 1 \cdot 0 + 3 \cdot 1 + 1  \cdot3 + 1 \cdot 5  = 11 \\). The total score for this position is \\( 13 - 11 = 2 \\) we are winning! Yay!

>**Side note for the curious**: the king has a score of \\(0\\), this is because we are only going to analyze legal positions so there will always be a king thus no score needed.

### Implementation

Let's start with the counting function:

```pseudo
function count_pieces(position){
    scores = {
        pawn = 1
        bishop = knight = 3
        rook = 5
        queen = 9
        king = 0
    }

    white_score = count(position.board.white, king) * scores.king + count(position.board.white, pawn) * scores.pawn ...

    black_score = ...

    return white_score - black_score
}
```

Then we use that function to calculate the best possible move:

```pseudo
function next_move(position){
    legal_moves = get_legal_moves(position)
    best_move = null
    best_score = -infinity

    for(move in legal_moves){
        new_position = position.play(move)
        score = count_pieces(new_position)
        if(score > best_score){
            best_score = score
            best_move = move
        }
    }
}
```

Now let's see some Rust code, we'll start with some helper functions:

```rust
fn is_opponent(piece_color: Color, our_color: Color) -> i64 {
    if piece_color == our_color {
        1
    } else {
        -1
    }
}
```

This function returns \\(1\\) if the piece is ours and \\(-1\\) if it isn't. We'll use this to know if we need to add or subtract a piece's score based on its color.

```rust
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
```

This function just returns for each piece, the score associated with it, the scores are the same as before.

```rust
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
```

Now, we're doing something interesting: this function takes the board, iterates every piece, calculates its value and adds everything together. Our pieces are added while enemy pieces are subtracted. Then we return the score.

```rust
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
```

Here we use `evaluate()` to find the move leading to the most favorable state. Then we return that move.

- If there are more than one move with the best value, we just return the one that happens to be first.

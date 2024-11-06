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

# What is a Chess Engine?

A chess engine is a computer program made to play chess. It looks at a position on the board and figures out the best move.

A chess engine has three main parts:

1. **Board Representation**
2. **Search**
3. **Evaluation**

## Board Representation

To work, the chess engine needs a way to represent the game state. This is what allows it to generate all the legal moves for a position. The goal here is to use as little memory as possible while keeping things efficient, since memory usage can be a bottleneck. On top of that, generating moves quickly is important too.

There are a ton of ways to represent the game state and generate moves, but to keep things simple, we'll use the shakmaty crate to take care of both for us.

If you're curious and want to dive deeper into board representation or move generation algorithms, here's a helpful resource:

- [Board Representation on the Chess Programming Wiki](https://www.chessprogramming.org/Board_Representation)

## Search

Humans play chess based on intuition and memory, but a chess engine uses raw computing power to analyze as many possible positions as it can. This process is called *search*. The search algorithm looks something like this:

```pseudo
function search(position){
    legal_moves = generate_moves(position)
    for(move in legal_moves){
        new_position = play(position, move)
        search(new_position)
    }
}
```

But obviously, this can go on forever, because there are *way* more possible chess positions than there are atoms in the universe. So, we need a way to stop the search at some point, which we'll talk about in the next sections.

## Evaluation

Once the engine has analyzed a position, it needs to figure out how good or bad it is for the player. This is where evaluation comes in: it helps the engine rank moves based on their strength so it can pick the best one. It's not an easy task, though. Since we'll often stop the search in the middle of a game, we need to find a way to evaluate all kinds of positions, not just in endgames.

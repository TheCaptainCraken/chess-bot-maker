# What is a chess engine

>A chess engine is a computer program designed to play chess. It analyzes a position, and generates a move that it regards as the strongest.

Generally speaking, a chess engine is composed of four main parts:

1. Board representation
2. Search
3. Evaluation

## Board representation

A chess engine needs an internal game state representation. This is then used to generate all the legal moves for a position. When talking about board representation, the lesser memory needed, the better. The same goes for move representation. It is also important to have a blazingly fast move generation algorithm.

There are many ways to represent a game state and many ways to generate moves. Since we are on a crunch, like I said earlier, we'll get a little help from shakmaty: this crate will manage the game state and move generation for us.

If you want to dive deeper into this topic, I'll leave you some nice resources:

- [Chess Programming Wiki](https://www.chessprogramming.org/Board_Representation)

## Search


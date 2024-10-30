const config = {
    pieceTheme: 'pieces/{piece}.png',
    position: 'start',
    draggable: true,
    onDragStart: onDragStart,
    onDrop: onDrop,
    onSnapEnd: onSnapEnd
}

const board = Chessboard('chessboard', config)
let game = new Chess()

function onDragStart(source, piece, position, orientation) {
    // do not pick up pieces if the game is over
    if (game.game_over()) return false

    // only pick up pieces for White
    if (piece.search(/^b/) !== -1) return false
}

/*
    Function to call the API to make the bot move

    NOTE: This could be way faster is instead of sending and receiving the FEN string, 
    we could send the move made by the player and the bot could calculate the next move based on that.

*/
async function botMoves() {
    const response = await fetch(`http://localhost:8080/bot-move/${game.fen().replaceAll("/", "%2F")}`, {
        mode: "no-cors",
    })

    const fen = await response.text()

    game = new Chess(fen)
    board.position(fen)
}

async function onDrop(source, target) {
    // see if the move is legal
    var move = game.move({
        from: source,
        to: target,
        promotion: 'q' // NOTE: always promote to a queen for simplicity, if you want to promote to a rook/knight/bishop, fuck you.
    })

    // illegal move
    if (move === null) return 'snapback'

    // Not is the machine's turn (HA!)
    await botMoves()
}

// update the board position after the piece snap
// for castling, en passant, pawn promotion
function onSnapEnd() {
    board.position(game.fen())
}
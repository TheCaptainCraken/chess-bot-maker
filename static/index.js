const config = {
    pieceTheme: 'pieces/{piece}.png',
    position: 'start',
    draggable: true,
    snapbackSpeed: 500,
    snapSpeed: 100,
    onDragStart: onDragStart,
    onDrop: onDrop,
    onSnapEnd: onSnapEnd
}

const board = Chessboard('chessboard', config)
let game = new Chess()

// fetch('http://127.0.0.1:8080/gameOver', { mode: "no-cors" }).then(response => response.json()).then(data => {
//     console.log(data)
// })

// fetch('http://127.0.0.1:8080/gimmeMoves', { mode: "no-cors" }).then(response => response.json()).then(data => {
//     console.log(data)
// })

function onDragStart(source, piece, position, orientation) {
    // do not pick up pieces if the game is over
    if (game.game_over()) return false

    // only pick up pieces for White
    if (piece.search(/^b/) !== -1) return false
}

async function botMoves() {
    // CALLING API TO GET BOT MOVE EXPECTING A FEN STRING

    const response = await fetch(`http://127.0.0.1:8080/bot-move/${game.fen().replaceAll("/", "%2F")}`, {
        mode: "no-cors",
    })

    const fen = await response.text()

    console.log(fen)

    game = new Chess(fen)
    board.position(fen)
}

async function onDrop(source, target) {
    // see if the move is legal
    var move = game.move({
        from: source,
        to: target,
        promotion: 'q' // NOTE: always promote to a queen for simplicity
    })

    // illegal move
    if (move === null) return 'snapback'

    // CALL API TO MAKE BOT MOVE
    await botMoves()
}

// update the board position after the piece snap
// for castling, en passant, pawn promotion
function onSnapEnd() {
    board.position(game.fen())
}
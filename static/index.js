const ruyLopez = 'r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R'

const config = {
    pieceTheme: 'pieces/{piece}.png',
    position: ruyLopez,
    draggable: true,
    moveSpeed: 'slow',
    snapbackSpeed: 500,
    snapSpeed: 100,
}

const board = Chessboard('chessboard', config)

fetch('http://127.0.0.1:8080/gameOver', { mode: "no-cors" }).then(response => response.json()).then(data => {
    console.log(data)
})
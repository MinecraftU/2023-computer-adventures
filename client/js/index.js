import init, { get_engine_move } from "../pkg/chess_engine.js";

function onDrop(source, target, piece, newPos, oldPos, orientation) {
    console.log('JS: Source: ' + source, 'Target: ' + target);

    init().then(() => {
        let engine_move = get_engine_move(source, target);
        console.log("JS: returned engine move:", engine_move);
        if (engine_move === "illegal move") {
            window.board.position(oldPos);
        } else {
            window.board.move(engine_move);
        }
    });
}

let config = {
    draggable: true,
    position: 'start',
    onDrop: onDrop,
    sparepieces: true
}
window.board = Chessboard('myBoard', config);

import init, { get_engine_move } from "../pkg/chess_engine.js";

function onDrop(source, target, piece, newPos, oldPos, orientation) {
    console.log('JS: Source: ' + source, 'Target: ' + target);

    handle_move(source, target);
}

function handle_move(source, target) {
    init().then(() => {
        get_engine_move(source, target);
    });
}

let config = {
    draggable: true,
    position: 'start',
    onDrop: onDrop,
    sparepieces: true
}
window.board = Chessboard('myBoard', config);

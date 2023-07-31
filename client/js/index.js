import init, { get_engine_move } from "../pkg/chess_engine.js";

function wait_until(source, target, oldPos) { // https://stackoverflow.com/a/64947598
    if (window.promotion === undefined) {
        window.setTimeout(wait_until, 100, source, target, oldPos);
    } else {
        console.log("JS: done waiting for player to choose promotion piece");
        make_move(source, target, oldPos);
        window.promotion = undefined;
    }
}

function make_move(source, target, oldPos) {
    let promotion = window.promotion === undefined ? "" : window.promotion;
    let new_position = get_engine_move(source, target, promotion);;
    console.log("js: returned engine move:", new_position);
    if (new_position === "illegal move") {
        window.board.position(oldPos);
    } else {
        window.board.position(new_position);
    }
}

function onDrop(source, target, piece, newPos, oldPos, orientation) {
    console.log('JS: Source: ' + source, 'Target: ' + target);

    init().then(() => {
        if (target[1] === "8" && piece === "wP") { // player must promote
            let promotion_choice = document.getElementById("promotion-choice");
            promotion_choice.style.display = "block";
            wait_until(source, target, oldPos);
            return;
        }

        make_move(source, target, oldPos);
    });
}

let config = {
    draggable: true,
    position: 'start',
    onDrop: onDrop,
    sparepieces: true
}
window.board = Chessboard('myBoard', config);

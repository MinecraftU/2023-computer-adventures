import init, { get_engine_move } from "../pkg/chess_engine.js";
import { openGameOverModal } from "./game_over.js";

document.getElementById("queenPromoButton").addEventListener("click", () => {
    handlePromotion('q')
}, false);
document.getElementById("rookPromoButton").addEventListener("click", () => {
    handlePromotion('r')
}, false);
document.getElementById("bishopPromoButton").addEventListener("click", () => {
    handlePromotion('b')
}, false);
document.getElementById("knightPromoButton").addEventListener("click", () => {
    handlePromotion('n')
}, false);

let promotionData;

function handlePromotion(piece) {
    closePromotionModal();
    let { source, target, oldPos } = promotionData;
    makeMove(source, target, oldPos, piece);
}

function openPromotionModal() {
    console.log("JS: open promotion modal");
    document.getElementById("promotionModal").style.display = "block";
}

function closePromotionModal() {
    document.getElementById("promotionModal").style.display = "none";
}

function onDrop(source, target, piece, newPos, oldPos, orientation) {
    console.log('JS: Source: ' + source, 'Target: ' + target);
    console.log("piece: ", piece);
    if (source[1] == 7 && piece == "wP") { // promotion
        promotionData = { source: source, target: target, oldPos: oldPos };
        openPromotionModal();
    }
    else { // not a promotion
        makeMove(source, target, oldPos, "");
    }
}

let config = {
    draggable: true,
    position: 'start',
    onDrop: onDrop,
    sparepieces: true
}
let board = Chessboard('myBoard', config);
let state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

const ending_states = ["stalemate after engine move", "stalemate after player move", "checkmate, engine won", "checkmate, player won"];

function makeMove(source, target, oldPos, promo) {
    init().then(() => {
        // not sure why this is necessary for calling non-wasm function to work
        if (ending_states.includes(state.split(";")[0])) {
            console.log("game over");
            board.position(oldPos);
            return;
        }
        let engine_output = get_engine_move(state, source, target, promo); // pass current state to engine, + user move + promotion if any
        console.log("JS: returned engine move:", engine_output);
        switch (engine_output.split(";")[0]) {
            case "illegal move":
                board.position(oldPos);
                break;
            case "stalemate after engine move":
                board.position(engine_output.split(";")[1]);
                state = engine_output;
                console.log("stalemate");
                openGameOverModal("Draw by stalemate");
                break;
            case "stalemate after player move":
                state = engine_output;
                console.log("stalemate");
                openGameOverModal("Draw by stalemate");
                break;
            case "checkmate, engine won":
                board.position(engine_output.split(";")[1]);
                state = engine_output;
                console.log("checkmate, engine won");
                openGameOverModal("Engine won by checkmate");
                break;
            case "checkmate, player won":
                state = engine_output;
                console.log("checkmate, player won");
                openGameOverModal("Player won by checkmate");
                break;
            default:
                board.position(engine_output);
                state = engine_output; // prevent extra FEN information from being thrown away
        }
    });
}
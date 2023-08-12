import init, { get_engine_move } from "../pkg/chess_engine.js";

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
    console.log("JS: open modal");
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
window.board = Chessboard('myBoard', config);
window.state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

function makeMove(source, target, oldPos, promo) {
    init().then(() => {
        let engine_output = get_engine_move(window.state, source, target, promo); // pass current state to engine, + user move + promotion if any
        console.log("JS: returned engine move:", engine_output);
        if (engine_output === "illegal move") {
            window.board.position(oldPos);
        } else {
            window.board.position(engine_output);
            window.state = engine_output; // prevent extra FEN information from being thrown away
        }
    });
}

export function my_alert(message) {
    console.log(message);
}

export function update_board(fen) {
    console.log("JS: updating board to ", fen);
    console.log("--------------------------------------");
    window.board.position(fen.split(" ")[0]);
}

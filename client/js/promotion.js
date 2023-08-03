function promote_to(piece) {
    console.log("JS: promoting to", piece);
    window.promotion = piece;

    document.getElementById("promotion-choice").style.display = "none";
}

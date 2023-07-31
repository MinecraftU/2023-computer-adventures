function promote_to(piece) {
    console.log("JS: promoting to", piece);
    window.promotion = piece;

    let promotion_choice = document.getElementById("promotion-choice");
    promotion_choice.style.display = "none";
}

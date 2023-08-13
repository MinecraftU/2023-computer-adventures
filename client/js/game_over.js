export function openGameOverModal(message) {
    let modal = document.getElementById("gameOverModal");
    modal.style.display = "block";
    modal.innerHTML = `<p>${message}</p>`;
}

/*
var ruyLopez = 'r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R'
var board = Chessboard('myBoard')
*/
function onDrop (source, target, piece, newPos, oldPos, orientation) {
  console.log('Source: ' + source);
  console.log('Target: ' + target);
  //console.log('Piece: ' + piece);
  //console.log('New position: ' + Chessboard.objToFen(newPos));
  //console.log('Old position: ' + Chessboard.objToFen(oldPos));
  //console.log('Orientation: ' + orientation);
  //console.log('~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~');
}

var config = {
  draggable: true,
  position: 'start',
  onDrop: onDrop,
  sparePieces: true
}
var board = Chessboard('myBoard', config);


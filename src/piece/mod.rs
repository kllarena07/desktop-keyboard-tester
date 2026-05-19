pub mod vertex;

const WHITE_PAWN_BYTES: &[u8] = include_bytes!("../../pieces/white/pawn.png");
const WHITE_CASTLE_BYTES: &[u8] = include_bytes!("../../pieces/white/castle.png");
const WHITE_KNIGHT_BYTES: &[u8] = include_bytes!("../../pieces/white/knight.png");
const WHITE_BISHOP_BYTES: &[u8] = include_bytes!("../../pieces/white/bishop.png");
const WHITE_KING_BYTES: &[u8] = include_bytes!("../../pieces/white/king.png");
const WHITE_QUEEN_BYTES: &[u8]= include_bytes!("../../pieces/white/queen.png");
const BLACK_PAWN_BYTES: &[u8] = include_bytes!("../../pieces/black/pawn.png");
const BLACK_CASTLE_BYTES: &[u8] = include_bytes!("../../pieces/black/castle.png");
const BLACK_KNIGHT_BYTES: &[u8] = include_bytes!("../../pieces/black/knight.png");
const BLACK_BISHOP_BYTES: &[u8] = include_bytes!("../../pieces/black/bishop.png");
const BLACK_KING_BYTES: &[u8] = include_bytes!("../../pieces/black/king.png");
const BLACK_QUEEN_BYTES: &[u8]= include_bytes!("../../pieces/black/queen.png");

#[derive(Debug, Clone)]
pub enum PieceType {
    Pawn,
    Castle,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(PartialEq, Debug, Clone)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Debug, Clone)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Self {
            piece_type,
            piece_color
        }
    }
    pub fn get_color(&self) -> &PieceColor {
        &self.piece_color
    }
    pub fn get_bytes(&self) -> &[u8] {
        match (&self.piece_type, &self.piece_color) {
            (PieceType::Pawn, PieceColor::White) => WHITE_PAWN_BYTES,
            (PieceType::Castle, PieceColor::White) => WHITE_CASTLE_BYTES,
            (PieceType::Knight, PieceColor::White) => WHITE_KNIGHT_BYTES,
            (PieceType::Bishop, PieceColor::White) => WHITE_BISHOP_BYTES,
            (PieceType::King, PieceColor::White) => WHITE_KING_BYTES,
            (PieceType::Queen, PieceColor::White) => WHITE_QUEEN_BYTES,
            (PieceType::Pawn, PieceColor::Black) => BLACK_PAWN_BYTES,
            (PieceType::Castle, PieceColor::Black) => BLACK_CASTLE_BYTES,
            (PieceType::Knight, PieceColor::Black) => BLACK_KNIGHT_BYTES,
            (PieceType::Bishop, PieceColor::Black) => BLACK_BISHOP_BYTES,
            (PieceType::King, PieceColor::Black) => BLACK_KING_BYTES,
            (PieceType::Queen, PieceColor::Black) => BLACK_QUEEN_BYTES
        }
    }
}


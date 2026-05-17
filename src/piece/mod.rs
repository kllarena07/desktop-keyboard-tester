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
    WhitePawn,
    WhiteCastle,
    WhiteKnight,
    WhiteBishop,
    WhiteKing,
    WhiteQueen,
    BlackPawn,
    BlackCastle,
    BlackKnight,
    BlackBishop,
    BlackKing,
    BlackQueen,
}

impl PieceType {
    pub fn get_bytes(&self) -> &[u8] {
        match self {
            PieceType::WhitePawn => WHITE_PAWN_BYTES,
            PieceType::WhiteCastle => WHITE_CASTLE_BYTES,
            PieceType::WhiteKnight => WHITE_KNIGHT_BYTES,
            PieceType::WhiteBishop => WHITE_BISHOP_BYTES,
            PieceType::WhiteKing => WHITE_KING_BYTES,
            PieceType::WhiteQueen => WHITE_QUEEN_BYTES,
            PieceType::BlackPawn => BLACK_PAWN_BYTES,
            PieceType::BlackCastle => BLACK_CASTLE_BYTES,
            PieceType::BlackKnight => BLACK_KNIGHT_BYTES,
            PieceType::BlackBishop => BLACK_BISHOP_BYTES,
            PieceType::BlackKing => BLACK_KING_BYTES,
            PieceType::BlackQueen => BLACK_QUEEN_BYTES
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        Self {
            piece_type,
        }
    }
}


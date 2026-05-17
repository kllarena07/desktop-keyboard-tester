use crate::piece::{Piece, PieceType};

pub mod vertex;

pub struct Chessboard {
    board_state: [Option<Piece>; 64]
}

impl Chessboard {
    pub fn new() -> Self {
        let board_state: [Option<Piece>; 64] = [
            Some(Piece::new(PieceType::BlackCastle, 0, 0)),
            Some(Piece::new(PieceType::BlackKnight, 1, 0)),
            Some(Piece::new(PieceType::BlackBishop, 2, 0)),
            Some(Piece::new(PieceType::BlackQueen, 3, 0)),
            Some(Piece::new(PieceType::BlackKing, 4, 0)),
            Some(Piece::new(PieceType::BlackBishop, 5, 0)),
            Some(Piece::new(PieceType::BlackKnight, 6, 0)),
            Some(Piece::new(PieceType::BlackCastle, 7, 0)),
            Some(Piece::new(PieceType::BlackPawn, 0, 1)),
            Some(Piece::new(PieceType::BlackPawn, 1, 1)),
            Some(Piece::new(PieceType::BlackPawn, 2, 1)),
            Some(Piece::new(PieceType::BlackPawn, 3, 1)),
            Some(Piece::new(PieceType::BlackPawn, 4, 1)),
            Some(Piece::new(PieceType::BlackPawn, 5, 1)),
            Some(Piece::new(PieceType::BlackPawn, 6, 1)),
            Some(Piece::new(PieceType::BlackPawn, 7, 1)),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            Some(Piece::new(PieceType::WhitePawn, 0, 6)),
            Some(Piece::new(PieceType::WhitePawn, 1, 6)),
            Some(Piece::new(PieceType::WhitePawn, 2, 6)),
            Some(Piece::new(PieceType::WhitePawn, 3, 6)),
            Some(Piece::new(PieceType::WhitePawn, 4, 6)),
            Some(Piece::new(PieceType::WhitePawn, 5, 6)),
            Some(Piece::new(PieceType::WhitePawn, 6, 6)),
            Some(Piece::new(PieceType::WhitePawn, 7, 6)),
            Some(Piece::new(PieceType::WhiteCastle, 0, 7)),
            Some(Piece::new(PieceType::WhiteKnight, 1, 7)),
            Some(Piece::new(PieceType::WhiteBishop, 2, 7)),
            Some(Piece::new(PieceType::WhiteQueen, 3, 7)),
            Some(Piece::new(PieceType::WhiteKing, 4, 7)),
            Some(Piece::new(PieceType::WhiteBishop, 5, 7)),
            Some(Piece::new(PieceType::WhiteKnight, 6, 7)),
            Some(Piece::new(PieceType::WhiteCastle, 7, 7)),
        ];

        Self {
            board_state
        }
    }
    pub fn get_board_state(&self) -> &[Option<Piece>] {
        &self.board_state
    }
}


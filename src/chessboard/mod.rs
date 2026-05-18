use crate::piece::{Piece, PieceType};

pub mod vertex;

pub struct Chessboard {
    board_state: [Option<Piece>; 64]
}

impl Chessboard {
    pub fn new() -> Self {
        let board_state: [Option<Piece>; 64] = [
            Some(Piece::new(PieceType::BlackCastle)),
            Some(Piece::new(PieceType::BlackKnight)),
            Some(Piece::new(PieceType::BlackBishop)),
            Some(Piece::new(PieceType::BlackQueen)),
            Some(Piece::new(PieceType::BlackKing)),
            Some(Piece::new(PieceType::BlackBishop)),
            Some(Piece::new(PieceType::BlackKnight)),
            Some(Piece::new(PieceType::BlackCastle)),
            Some(Piece::new(PieceType::BlackPawn)),
            Some(Piece::new(PieceType::BlackPawn)),
            Some(Piece::new(PieceType::BlackPawn)),
            Some(Piece::new(PieceType::BlackPawn)),
            Some(Piece::new(PieceType::BlackPawn)),
            Some(Piece::new(PieceType::BlackPawn)),
            Some(Piece::new(PieceType::BlackPawn)),
            Some(Piece::new(PieceType::BlackPawn)),
            Some(Piece::new(PieceType::WhitePawn)), None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            Some(Piece::new(PieceType::WhitePawn)),
            Some(Piece::new(PieceType::WhitePawn)),
            Some(Piece::new(PieceType::WhitePawn)),
            Some(Piece::new(PieceType::WhitePawn)),
            Some(Piece::new(PieceType::WhitePawn)),
            Some(Piece::new(PieceType::WhitePawn)),
            Some(Piece::new(PieceType::WhitePawn)),
            Some(Piece::new(PieceType::WhitePawn)),
            Some(Piece::new(PieceType::WhiteCastle)),
            Some(Piece::new(PieceType::WhiteKnight)),
            Some(Piece::new(PieceType::WhiteBishop)),
            Some(Piece::new(PieceType::WhiteQueen)),
            Some(Piece::new(PieceType::WhiteKing)),
            Some(Piece::new(PieceType::WhiteBishop)),
            Some(Piece::new(PieceType::WhiteKnight)),
            Some(Piece::new(PieceType::WhiteCastle)),
        ];

        Self {
            board_state
        }
    }
    pub fn get_board_state(&self) -> &[Option<Piece>] {
        &self.board_state
    }
    pub fn move_piece(&mut self, selected_index: usize, new_position: (u32, u32)) {
        let (board_x, board_y) = new_position;
        let new_board_pos = (board_x + (board_y * 8)) as usize;

        if let Some(existing_index) = self.board_state.get(selected_index) && let Some(piece) = existing_index {
            println!("({}, {}) ... {} {:?}", board_x, board_y, selected_index, piece);
            self.board_state[new_board_pos] = Some(piece.clone());
            self.board_state[selected_index] = None;
            println!("{:?}", self.board_state);
        }
    }
}


use crate::piece::{Piece, PieceColor, PieceType};

pub mod vertex;

pub struct Chessboard {
    board_state: [Option<Piece>; 64]
}

impl Chessboard {
    pub fn new() -> Self {
        let board_state: [Option<Piece>; 64] = [
            Some(Piece::new(PieceType::Castle, PieceColor::Black)),
            Some(Piece::new(PieceType::Knight, PieceColor::Black)),
            Some(Piece::new(PieceType::Bishop, PieceColor::Black)),
            Some(Piece::new(PieceType::Queen, PieceColor::Black)),
            Some(Piece::new(PieceType::King, PieceColor::Black)),
            Some(Piece::new(PieceType::Bishop, PieceColor::Black)),
            Some(Piece::new(PieceType::Knight, PieceColor::Black)),
            Some(Piece::new(PieceType::Castle, PieceColor::Black)),
            Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
            Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
            Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
            Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
            Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
            Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
            Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
            Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            Some(Piece::new(PieceType::Pawn, PieceColor::White)),
            Some(Piece::new(PieceType::Pawn, PieceColor::White)),
            Some(Piece::new(PieceType::Pawn, PieceColor::White)),
            Some(Piece::new(PieceType::Pawn, PieceColor::White)),
            Some(Piece::new(PieceType::Pawn, PieceColor::White)),
            Some(Piece::new(PieceType::Pawn, PieceColor::White)),
            Some(Piece::new(PieceType::Pawn, PieceColor::White)),
            Some(Piece::new(PieceType::Pawn, PieceColor::White)),
            Some(Piece::new(PieceType::Castle, PieceColor::White)),
            Some(Piece::new(PieceType::Knight, PieceColor::White)),
            Some(Piece::new(PieceType::Bishop, PieceColor::White)),
            Some(Piece::new(PieceType::Queen, PieceColor::White)),
            Some(Piece::new(PieceType::King, PieceColor::White)),
            Some(Piece::new(PieceType::Bishop, PieceColor::White)),
            Some(Piece::new(PieceType::Knight, PieceColor::White)),
            Some(Piece::new(PieceType::Castle, PieceColor::White)),
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


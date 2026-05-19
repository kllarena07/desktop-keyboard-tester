use crate::piece::{Piece, PieceColor, PieceType};
use crate::movement::would_leave_king_in_check;

pub mod vertex;

#[derive(Debug)]
pub enum MoveError {
    NoPieceAtSource,
    InvalidMove(String),
    PathBlocked,
    WouldLeaveKingInCheck,
    NotYourTurn,
}

pub struct Chessboard {
    board_state: [Option<Piece>; 64],
    last_pawn_double_push: Option<(usize, usize)>,
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
            board_state,
            last_pawn_double_push: None,
        }
    }
    
    pub fn get_board_state(&self) -> &[Option<Piece>] {
        &self.board_state
    }
    
    pub fn get_piece_mut(&mut self, index: usize) -> Option<&mut Piece> {
        self.board_state[index].as_mut()
    }
    
    pub fn get_en_passant_target(&self) -> Option<usize> {
        if let Some((from, to)) = self.last_pawn_double_push {
            let _direction = if to < 32 { 1 } else { -1 };
            let (_, from_y) = (from / 8, from / 8);
            let (_, to_y) = (to / 8, to / 8);
            if from_y == 6 && to_y == 4 {
                Some(to + 8)
            } else if from_y == 1 && to_y == 3 {
                Some(to - 8)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn is_valid_move(&mut self, from: usize, to: usize, current_turn: PieceColor) -> Result<(), MoveError> {
        if from >= 64 || to >= 64 {
            return Err(MoveError::InvalidMove("Invalid square index".to_string()));
        }
        
        let piece = self.board_state[from].as_ref().ok_or(MoveError::NoPieceAtSource)?;
        
        if piece.get_color() != &current_turn {
            return Err(MoveError::NotYourTurn);
        }
        
        let en_passant_target = self.get_en_passant_target();
        
        if !piece.is_valid_move(from, to, &self.board_state, en_passant_target) {
            return Err(MoveError::InvalidMove("Invalid move for piece type".to_string()));
        }
        
        if would_leave_king_in_check(from, to, current_turn, &self.board_state) {
            return Err(MoveError::WouldLeaveKingInCheck);
        }
        
        Ok(())
    }
    
    pub fn move_piece(&mut self, selected_index: usize, new_position: (u32, u32), current_turn: PieceColor) -> Result<(), MoveError> {
        let (board_x, board_y) = new_position;
        let new_board_pos = (board_x + (board_y * 8)) as usize;
        
        self.is_valid_move(selected_index, new_board_pos, current_turn)?;
        
        self.execute_move(selected_index, new_board_pos)
    }
    
    fn execute_move(&mut self, from: usize, to: usize) -> Result<(), MoveError> {
        let piece = self.board_state[from].clone().ok_or(MoveError::NoPieceAtSource)?;
        let piece_type = piece.get_type();
        let piece_color = piece.get_color().clone();
        
        let is_pawn_double_push = piece_type == PieceType::Pawn && {
            let from_y = from / 8;
            let to_y = to / 8;
            (piece_color == PieceColor::White && from_y == 6 && to_y == 4) ||
            (piece_color == PieceColor::Black && from_y == 1 && to_y == 3)
        };
        
        if let Some(mut moved_piece) = self.board_state[from].take() {
            moved_piece.mark_as_moved();
            
            if piece_type == PieceType::Pawn {
                let (_, to_y) = (to / 8, to / 8);
                let promotion_row = if piece_color == PieceColor::White { 0 } else { 7 };
                
                if to_y == promotion_row {
                    let promoted_piece = Piece::new(PieceType::Queen, piece_color);
                    self.board_state[to] = Some(promoted_piece);
                } else {
                    self.board_state[to] = Some(moved_piece);
                }
            } else if piece_type == PieceType::King && (to as i32 - from as i32).abs() == 2 {
                let (_, from_y) = (from / 8, from / 8);
                let (to_x, _) = (to % 8, to / 8);
                
                if to_x == 6 {
                    let rook_from = if from_y == 7 { 63 } else { 7 };
                    let rook_to = if from_y == 7 { 61 } else { 5 };
                    if let Some(mut rook) = self.board_state[rook_from].take() {
                        rook.mark_as_moved();
                        self.board_state[rook_to] = Some(rook);
                    }
                } else if to_x == 2 {
                    let rook_from = if from_y == 7 { 56 } else { 0 };
                    let rook_to = if from_y == 7 { 59 } else { 3 };
                    if let Some(mut rook) = self.board_state[rook_from].take() {
                        rook.mark_as_moved();
                        self.board_state[rook_to] = Some(rook);
                    }
                }
                
                self.board_state[to] = Some(moved_piece);
            } else {
                self.board_state[to] = Some(moved_piece);
            }
        }
        
        if let Some(en_passant_target) = self.get_en_passant_target() {
            if to == en_passant_target && piece_type == PieceType::Pawn {
                let capture_square = if piece_color == PieceColor::White {
                    to + 8
                } else {
                    to - 8
                };
                self.board_state[capture_square] = None;
            }
        }
        
        if is_pawn_double_push {
            self.last_pawn_double_push = Some((from, to));
        } else {
            self.last_pawn_double_push = None;
        }
        
        println!("Move executed: {:?} {} -> {}", piece_type, from, to);
        println!("{:?}", self.board_state);
        
        Ok(())
    }
}

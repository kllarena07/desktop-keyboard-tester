use crate::piece::movement::{coordinates_from_index, index_from_coordinates, is_path_clear};

pub mod movement;
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

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Castle,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Debug, Clone)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
    has_moved: bool
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Self {
            piece_type,
            piece_color,
            has_moved: false
        }
    }
    pub fn get_color(&self) -> &PieceColor {
        &self.piece_color
    }
    pub fn get_type(&self) -> PieceType {
        self.piece_type.clone()
    }
    pub fn has_moved(&self) -> bool {
        self.has_moved
    }
    pub fn mark_as_moved(&mut self) {
        self.has_moved = true;
    }
    
    pub fn is_valid_move(&self, from: usize, to: usize, board: &[Option<Piece>], en_passant_target: Option<usize>) -> bool {
        if from == to {
            return false;
        }
        
        let (from_x, from_y) = coordinates_from_index(from);
        let (to_x, to_y) = coordinates_from_index(to);
        let dx = to_x as i32 - from_x as i32;
        let dy = to_y as i32 - from_y as i32;
        
        let target_piece = board[to].as_ref();
        
        if let Some(target) = target_piece {
            if target.get_color() == self.get_color() {
                return false;
            }
        }
        
        match self.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from_x, from_y, to_x, to_y, dx, dy, target_piece, board, en_passant_target),
            PieceType::Castle => self.is_valid_castle_move(from, to, dx, dy, target_piece, board),
            PieceType::Knight => self.is_valid_knight_move(dx, dy),
            PieceType::Bishop => self.is_valid_bishop_move(from, to, dx, dy, board),
            PieceType::King => self.is_valid_king_move(from, to, from_x, from_y, to_x, to_y, dx, dy, board),
            PieceType::Queen => self.is_valid_queen_move(from, to, dx, dy, board),
        }
    }
    
    pub fn get_legal_moves(&self, from: usize, board: &[Option<Piece>], en_passant_target: Option<usize>) -> Vec<usize> {
        let mut legal_moves = Vec::new();
        
        for to in 0..64 {
            if self.is_valid_move(from, to, board, en_passant_target) {
                legal_moves.push(to);
            }
        }
        
        legal_moves
    }
    
    fn is_valid_pawn_move(&self, from_x: u32, from_y: u32, to_x: u32, to_y: u32, dx: i32, dy: i32, target_piece: Option<&Piece>, board: &[Option<Piece>], en_passant_target: Option<usize>) -> bool {
        let direction = if self.piece_color == PieceColor::White { -1 } else { 1 };
        let start_row = if self.piece_color == PieceColor::White { 6 } else { 1 };
        
        let to_index = index_from_coordinates(to_x, to_y);
        
        if dx == 0 {
            if dy == direction {
                if target_piece.is_none() {
                    return true;
                }
            } else if dy == direction * 2 && from_y == start_row {
                let intermediate_y = from_y as i32 + direction;
                let intermediate_index = index_from_coordinates(from_x, intermediate_y as u32);
                if target_piece.is_none() && board[intermediate_index].is_none() {
                    return true;
                }
            }
        } else if dx.abs() == 1 && dy == direction {
            if target_piece.is_some() && target_piece.unwrap().get_color() != self.get_color() {
                return true;
            }
            
            if let Some(en_passant_square) = en_passant_target {
                if to_index == en_passant_square {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn is_valid_castle_move(&self, from: usize, to: usize, dx: i32, dy: i32, target_piece: Option<&Piece>, board: &[Option<Piece>]) -> bool {
        if dx == 0 || dy == 0 {
            if is_path_clear(from, to, board) {
                return true;
            }
        }
        false
    }
    
    fn is_valid_knight_move(&self, dx: i32, dy: i32) -> bool {
        (dx.abs() == 2 && dy.abs() == 1) || (dx.abs() == 1 && dy.abs() == 2)
    }
    
    fn is_valid_bishop_move(&self, from: usize, to: usize, dx: i32, dy: i32, board: &[Option<Piece>]) -> bool {
        if dx.abs() == dy.abs() && dx != 0 {
            if is_path_clear(from, to, board) {
                return true;
            }
        }
        false
    }
    
    fn is_valid_king_move(&self, from: usize, to: usize, from_x: u32, from_y: u32, to_x: u32, to_y: u32, dx: i32, dy: i32, board: &[Option<Piece>]) -> bool {
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return true;
        }
        
        if !self.has_moved && from_y == 7 && to_y == 7 && self.piece_color == PieceColor::White {
            if to_x == 6 {
                if board[index_from_coordinates(5, 7)].is_none() && 
                   board[index_from_coordinates(6, 7)].is_none() &&
                   is_path_clear(from, to, board) {
                    if let Some(ref rook) = board[index_from_coordinates(7, 7)] {
                        let rook: &Piece = rook;
                        if rook.get_type() == PieceType::Castle && !rook.has_moved() {
                            return true;
                        }
                    }
                }
            } else if to_x == 2 {
                if board[index_from_coordinates(3, 7)].is_none() && 
                   board[index_from_coordinates(2, 7)].is_none() &&
                   board[index_from_coordinates(1, 7)].is_none() &&
                   is_path_clear(from, to, board) {
                    if let Some(ref rook) = board[index_from_coordinates(0, 7)] {
                        let rook: &Piece = rook;
                        if rook.get_type() == PieceType::Castle && !rook.has_moved() {
                            return true;
                        }
                    }
                }
            }
        }
        
        if !self.has_moved && from_y == 0 && to_y == 0 && self.piece_color == PieceColor::Black {
            if to_x == 6 {
                if board[index_from_coordinates(5, 0)].is_none() && 
                   board[index_from_coordinates(6, 0)].is_none() &&
                   is_path_clear(from, to, board) {
                    if let Some(ref rook) = board[index_from_coordinates(7, 0)] {
                        let rook: &Piece = rook;
                        if rook.get_type() == PieceType::Castle && !rook.has_moved() {
                            return true;
                        }
                    }
                }
            } else if to_x == 2 {
                if board[index_from_coordinates(3, 0)].is_none() && 
                   board[index_from_coordinates(2, 0)].is_none() &&
                   board[index_from_coordinates(1, 0)].is_none() &&
                   is_path_clear(from, to, board) {
                    if let Some(ref rook) = board[index_from_coordinates(0, 0)] {
                        let rook: &Piece = rook;
                        if rook.get_type() == PieceType::Castle && !rook.has_moved() {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
    
    fn is_valid_queen_move(&self, from: usize, to: usize, dx: i32, dy: i32, board: &[Option<Piece>]) -> bool {
        if dx == 0 || dy == 0 {
            if is_path_clear(from, to, board) {
                return true;
            }
        } else if dx.abs() == dy.abs() {
            if is_path_clear(from, to, board) {
                return true;
            }
        }
        false
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
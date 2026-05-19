use crate::piece::{Piece, PieceColor, PieceType};

pub fn coordinates_from_index(index: usize) -> (u32, u32) {
    (index as u32 % 8, index as u32 / 8)
}

pub fn index_from_coordinates(x: u32, y: u32) -> usize {
    (x + y * 8) as usize
}

pub fn get_square_direction(from: usize, to: usize) -> Option<(i32, i32)> {
    let (from_x, from_y) = coordinates_from_index(from);
    let (to_x, to_y) = coordinates_from_index(to);
    
    let dx = to_x as i32 - from_x as i32;
    let dy = to_y as i32 - from_y as i32;
    
    if dx == 0 && dy == 0 {
        return None;
    }
    
    let gcd_val = gcd(dx.abs() as u32, dy.abs() as u32);
    Some((dx / gcd_val as i32, dy / gcd_val as i32))
}

pub fn is_path_clear(from: usize, to: usize, board: &[Option<Piece>]) -> bool {
    let direction = match get_square_direction(from, to) {
        Some(dir) => dir,
        None => return true,
    };
    
    let mut current = from;
    loop {
        let (x, y) = coordinates_from_index(current);
        let new_x = x as i32 + direction.0;
        let new_y = y as i32 + direction.1;
        
        if new_x < 0 || new_x >= 8 || new_y < 0 || new_y >= 8 {
            return true;
        }
        
        current = index_from_coordinates(new_x as u32, new_y as u32);
        
        if current == to {
            return true;
        }
        
        if board[current].is_some() {
            return false;
        }
    }
}

pub fn get_king_position(color: PieceColor, board: &[Option<Piece>]) -> Option<usize> {
    board.iter()
        .position(|p| p.as_ref().map(|piece| piece.get_type() == PieceType::King && *piece.get_color() == color).unwrap_or(false))
}

pub fn is_square_attacked(square: usize, attacker_color: PieceColor, board: &[Option<Piece>]) -> bool {
    let (target_x, target_y) = coordinates_from_index(square);
    
    for (index, piece_option) in board.iter().enumerate() {
        if let Some(piece) = piece_option {
            if *piece.get_color() == attacker_color {
                let (from_x, from_y) = coordinates_from_index(index);
                
                match piece.get_type() {
                    PieceType::Pawn => {
                        let direction = if attacker_color == PieceColor::White { 1 } else { -1 };
                        let pawn_y = from_y as i32 + direction;
                        if pawn_y >= 0 && pawn_y < 8 {
                            if (from_x as i32 - 1).abs_diff(target_x as i32) == 1 && pawn_y as u32 == target_y {
                                return true;
                            }
                            if (from_x as i32 + 1).abs_diff(target_x as i32) == 1 && pawn_y as u32 == target_y {
                                return true;
                            }
                        }
                    }
                    PieceType::Knight => {
                        let dx = (from_x as i32 - target_x as i32).abs();
                        let dy = (from_y as i32 - target_y as i32).abs();
                        if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
                            return true;
                        }
                    }
                    PieceType::King => {
                        let dx = (from_x as i32 - target_x as i32).abs();
                        let dy = (from_y as i32 - target_y as i32).abs();
                        if dx <= 1 && dy <= 1 {
                            return true;
                        }
                    }
                    PieceType::Castle | PieceType::Queen => {
                        let dx = from_x as i32 - target_x as i32;
                        let dy = from_y as i32 - target_y as i32;
                        if dx == 0 || dy == 0 {
                            if is_path_clear(index, square, board) {
                                return true;
                            }
                        }
                    }
                    PieceType::Bishop => {
                        let dx = (from_x as i32 - target_x as i32).abs();
                        let dy = (from_y as i32 - target_y as i32).abs();
                        if dx == dy {
                            if is_path_clear(index, square, board) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn is_king_in_check(color: PieceColor, board: &[Option<Piece>]) -> bool {
    if let Some(king_pos) = get_king_position(color, board) {
        let opponent_color = match color {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
        return is_square_attacked(king_pos, opponent_color, board);
    }
    false
}

pub fn would_leave_king_in_check(from: usize, to: usize, piece_color: PieceColor, board: &[Option<Piece>]) -> bool {
    let mut temp_board = board.to_vec();
    let piece = temp_board[from].clone();
    temp_board[to] = piece;
    temp_board[from] = None;
    
    is_king_in_check(piece_color, &temp_board)
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
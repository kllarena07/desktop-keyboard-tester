use crate::renderer::Renderer;

use std::{sync::Arc};
use winit::{
    application::ApplicationHandler, dpi::PhysicalPosition, event::*, event_loop::ActiveEventLoop, keyboard::PhysicalKey, window::Window
};

pub struct App {
    pub state: Option<Renderer>,
    mouse_position: Option<PhysicalPosition<f64>>,
    grabbed_piece: Option<usize>
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
            mouse_position: None,
            grabbed_piece: None,
        }
    }
}

impl ApplicationHandler<Renderer> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes().with_inner_size(winit::dpi::LogicalSize::new(600, 600)).with_resizable(false);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.state = Some(pollster::block_on(Renderer::new(window)).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: Renderer) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(renderer) => renderer,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        // Log the error and exit gracefully
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::CursorMoved { device_id: _, position } => {
                self.mouse_position = Some(position);

                // if let (Some(grabbed_piece), Some(s)) = (self.grabbed_piece, self.state.as_mut()) {
                //     let mouse_pixel = self.mouse_position.unwrap();
                //     let (mx, my): (f32, f32) = mouse_pixel.into();
                //
                //     let piece_space_x = ((mx - 35.0) / 600.0) * 2.0;
                //     let piece_space_y = ((my - 35.0) / 600.0) * 2.0;
                //
                //     println!("Tried moving {:?}", grabbed_piece);
                // }
            },
            WindowEvent::MouseInput { state, button, .. } => match (button, state.is_pressed()) {
                (MouseButton::Left, true) => {
                    if let Some(mouse_position) = self.mouse_position {
                        let (board_x, board_y) = ((mouse_position.x / 75.0) as usize, (mouse_position.y / 75.0) as usize);
                        // this is effectively row number + column number 
                        let selected_board_state_index = board_x + (board_y * 8);
                        self.grabbed_piece = Some(selected_board_state_index);
                        println!("({}, {}) = {}. Grabbed {:?}", board_x, board_y, board_x + (board_y*8), self.grabbed_piece);
                    }
                }
                (MouseButton::Left, false) => {
                    if let Some(mouse_position) = self.mouse_position {
                        let (board_x, board_y) = ((mouse_position.x / 75.0) as usize, (mouse_position.y / 75.0) as usize);
                        let new_board_pos = board_x + (board_y * 8);

                        if let Some(grabbed_piece) = self.grabbed_piece {
                            if let Some(state) = self.state.as_mut() {
                                state.chessboard.move_piece(grabbed_piece, (board_x as u32, board_y as u32));
                                state.update_piece_identity(grabbed_piece, new_board_pos);
                            }
                        }
                    }
                }
                _ => {}
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

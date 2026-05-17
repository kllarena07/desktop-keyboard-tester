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

                if let (Some(grabbed_piece), Some(s)) = (self.grabbed_piece, self.state.as_mut()) {
                    let mouse_pixel = self.mouse_position.unwrap();
                    let (mx, my): (f32, f32) = mouse_pixel.into();

                    let piece_space_x = ((mx - 35.0) / 600.0) * 2.0;
                    let piece_space_y = ((my - 35.0) / 600.0) * 2.0;

                    println!("Tried moving {:?}", grabbed_piece);
                }
            },
            WindowEvent::MouseInput { state, button, .. } => match (button, state.is_pressed()) {
                (MouseButton::Left, true) => {
                    if let Some(s) = &self.state {
                        let mouse_pixel = self.mouse_position.unwrap();
                        let (mx, my): (f32, f32) = mouse_pixel.into();
                        
                        let piece_space_x = (mx / 600.0) * 2.0;
                        let piece_space_y = (my / 600.0) * 2.0;
                        
                        let board_state = s.chessboard.get_board_state();
                        for (i, piece) in board_state.iter().enumerate() {
                            if let Some(p) = &piece {
                                let px = p.x as f32 * 0.25;
                                let py = p.y as f32 * 0.25;
                                if px <= piece_space_x && piece_space_x <= px + 0.25 && py <= piece_space_y && piece_space_y <= py + 0.25 {
                                    self.grabbed_piece = Some(i);
                                    println!("Grabbed {:?}", p.piece_type);
                                }
                            }
                        }
                    };
                }
                (MouseButton::Left, false) => {
                    if let (Some(grabbed_piece), Some(s)) = (self.grabbed_piece, self.state.as_mut()) {
                        let mouse_pixel = self.mouse_position.unwrap();
                        let (mx, my): (f32, f32) = mouse_pixel.into();

                        let piece_space_x = ((mx - 35.0) / 600.0) * 2.0;
                        let piece_space_y = ((my - 35.0) / 600.0) * 2.0;

                        // s.chessboard.move_piece(grabbed_piece, (piece_space_x, piece_space_y));
                        println!("Tried moving {:?}", grabbed_piece);
                    }
                    self.grabbed_piece = None;
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

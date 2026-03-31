use std::collections::HashMap;
use std::sync::Arc;

use pixels::{SurfaceTexture, Pixels};
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::Key;
use winit::window::{Window, WindowId};

struct App {
    window: Option<Arc<Window>>,
    activated: HashMap<Key, bool>,
    pixels: Option<Pixels<'static>>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());

        self.window = Some(window);

        let window_clone = self.window.as_ref().unwrap().clone();
        let window_width = window_clone.inner_size().width;
        let window_height = window_clone.inner_size().height;
        let surface_texture = SurfaceTexture::new(window_width, window_height, window_clone);
        let pixels = Pixels::new(window_width, window_height, surface_texture);

        self.pixels = Some(pixels.unwrap());

        let mut hm: HashMap<Key, bool> = HashMap::new();
        for ch in 'a'..='z' {
            hm.insert(Key::Character(ch.to_string().into()), false);
        }
        self.activated = hm;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let window_width = window.inner_size().width as usize;
                let window_height = window.inner_size().height as usize;
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    for (i, spot) in frame.chunks_exact_mut(4).enumerate() {
                        if i % window_height == 0 {
                            spot[0] = 0xFF; // R
                            spot[1] = 0x00; // G
                            spot[2] = 0x00; // B
                            spot[3] = 0xFF; // A
                        } else {
                            spot[0] = 0x00; // R
                            spot[1] = 0x00; // G
                            spot[2] = 0xFF; // B
                            spot[3] = 0xFF; // A
                        }
                    }

                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                    }
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                // self.activated.entry(event.logical_key).and_modify(|val| *val = event.state.is_pressed());
                // self.activated.entry(event.logical_key).and_modify(|val| *val = true);
                // println!("{:?}", self.activated);
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App { window: None, activated: HashMap::new(), pixels: None };
    let _ = event_loop.run_app(&mut app);
}

use std::{iter, sync::Arc};

use wgpu::Backends;
use winit::{
    event_loop::{ActiveEventLoop},
    keyboard::KeyCode,
    window::Window,
};

use crate::{chessboard::{Chessboard}, piece::Piece};

pub struct State {
    pub surface: wgpu::Surface<'static>,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub config: Arc<wgpu::SurfaceConfiguration>,
    pub window: Arc<Window>,
    chessboard: Chessboard,
    pieces: Vec<Piece>
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::all(),
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let device = Arc::new(device);
        let config = Arc::new(config);

        let chessboard = Chessboard::new(Arc::clone(&device), Arc::clone(&config));

        let black_pawn = include_bytes!("../pieces/black/pawn.png");
        let black_castle = include_bytes!("../pieces/black/castle.png");
        let black_knight = include_bytes!("../pieces/black/knight.png");
        let black_bishop = include_bytes!("../pieces/black/bishop.png");
        let black_king = include_bytes!("../pieces/black/king.png");
        let black_queen = include_bytes!("../pieces/black/queen.png");

        let white_pawn = include_bytes!("../pieces/white/pawn.png");
        let white_castle = include_bytes!("../pieces/white/castle.png");
        let white_knight = include_bytes!("../pieces/white/knight.png");
        let white_bishop = include_bytes!("../pieces/white/bishop.png");
        let white_king = include_bytes!("../pieces/white/king.png");
        let white_queen = include_bytes!("../pieces/white/queen.png");

        let pieces: Vec<Piece> = vec![
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_castle, 0.00, 0.0),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_knight, 0.25, 0.0),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_bishop, 0.50, 0.0),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_king, 0.75, 0.0),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_queen, 1.00, 0.0),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_bishop, 1.25, 0.0),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_knight, 1.50, 0.0),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_castle, 1.75, 0.0),

            Piece::new(Arc::clone(&device), Arc::clone(&config), black_pawn, 0.00, 0.25),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_pawn, 0.25, 0.25),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_pawn, 0.50, 0.25),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_pawn, 0.75, 0.25),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_pawn, 1.00, 0.25),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_pawn, 1.25, 0.25),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_pawn, 1.50, 0.25),
            Piece::new(Arc::clone(&device), Arc::clone(&config), black_pawn, 1.75, 0.25),

            Piece::new(Arc::clone(&device), Arc::clone(&config), white_pawn, 0.00, 1.5),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_pawn, 0.25, 1.5),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_pawn, 0.50, 1.5),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_pawn, 0.75, 1.5),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_pawn, 1.00, 1.5),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_pawn, 1.25, 1.5),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_pawn, 1.50, 1.5),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_pawn, 1.75, 1.5),

            Piece::new(Arc::clone(&device), Arc::clone(&config), white_castle, 0.00, 1.75),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_knight, 0.25, 1.75),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_bishop, 0.50, 1.75),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_king, 0.75, 1.75),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_queen, 1.00, 1.75),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_bishop, 1.25, 1.75),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_knight, 1.50, 1.75),
            Piece::new(Arc::clone(&device), Arc::clone(&config), white_castle, 1.75, 1.75)
        ];

        for piece in &pieces {
            queue.write_texture(
                // Tells wgpu where to copy the pixel data
                wgpu::TexelCopyTextureInfo {
                    texture: &piece.diffuse_texture(),
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                // The actual pixel data
                &piece.diffuse_rgba(),
                // The layout of the texture
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * piece.dimensions().0),
                    rows_per_image: Some(piece.dimensions().1),
                },
                piece.texture_size(),
            );
        }

        Ok(Self {
            surface,
            device,
            queue,
            config,
            window,
            chessboard,
            pieces
        })
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.surface.configure(&self.device, &self.config);
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // Skip this frame
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // You could recreate the devices and all resources
                // created with it here, but we'll just bail
                anyhow::bail!("Lost device");
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })
                ],
                depth_stencil_attachment: None,
                multiview_mask: None,
                occlusion_query_set: None,
                timestamp_writes: None
            });

            render_pass.set_pipeline(&self.chessboard.render_pipeline());
            render_pass.set_vertex_buffer(0, self.chessboard.vertex_buffer().slice(..));
            render_pass.draw(0..self.chessboard.num_vertices(), 0..1);

            for piece in &self.pieces {
                render_pass.set_pipeline(&piece.render_pipeline());
                render_pass.set_bind_group(0, &piece.diffuse_bind_group(), &[]);
                render_pass.set_vertex_buffer(0, piece.vertex_buffer().slice(..));
                render_pass.draw(0..piece.num_vertices(), 0..1);
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }
}


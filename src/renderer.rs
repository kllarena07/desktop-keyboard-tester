use std::{iter, sync::Arc};

use wgpu::{util::DeviceExt, Backends};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};
use crate::chessboard::{Chessboard, vertex::ChessboardVertex};
use crate::piece::{Piece, vertex::PieceVertex};

struct PieceGpuResources {
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
}

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub config: Arc<wgpu::SurfaceConfiguration>,
    pub window: Arc<Window>,
    
    pub chessboard: Chessboard,
    chessboard_render_pipeline: wgpu::RenderPipeline,
    chessboard_vertex_buffer: wgpu::Buffer,
    chessboard_num_vertices: u32,

    piece_render_pipeline: wgpu::RenderPipeline,
    piece_bind_groups: [Option<wgpu::BindGroup>; 64],
    piece_vertex_buffers: [Option<wgpu::Buffer>; 64],
    piece_num_vertices: [Option<u32>; 64],
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Renderer> {
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

        let chessboard = Chessboard::new();
        let pieces = chessboard.get_board_state();

        let chessboard_render_pipeline = Self::create_chessboard_pipeline(&device, &config);
        let (chessboard_vertex_buffer, chessboard_num_vertices) = Self::create_chessboard_vertex_buffer(&device);

        let piece_texture_bind_group_layout = Self::create_piece_bind_group_layout(&device);
        let piece_render_pipeline = Self::create_piece_render_pipeline(&device, &config, &piece_texture_bind_group_layout);

        let mut piece_bind_groups: [Option<wgpu::BindGroup>; 64] = std::array::from_fn(|_| None);
        let mut piece_vertex_buffers: [Option<wgpu::Buffer>; 64] = std::array::from_fn(|_| None);
        let mut piece_num_vertices: [Option<u32>; 64] = std::array::from_fn(|_| None);

        for (board_index, piece) in pieces.iter().enumerate() {
            if let Some(piece) = piece {
                let resources = Self::create_piece_resources(&device, &queue, piece, board_index, &piece_texture_bind_group_layout);
                piece_bind_groups[board_index] = Some(resources.bind_group);
                piece_vertex_buffers[board_index] = Some(resources.vertex_buffer);
                piece_num_vertices[board_index] = Some(resources.num_vertices);
            }
        }

        Ok(Self {
            surface,
            device: Arc::new(device),
            queue,
            config: Arc::new(config),
            window,
            chessboard,
            chessboard_render_pipeline,
            chessboard_vertex_buffer,
            chessboard_num_vertices,
            piece_render_pipeline,
            piece_bind_groups,
            piece_vertex_buffers,
            piece_num_vertices,
        })
    }

    fn create_chessboard_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("chessboard_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("chessboard/chessboard.wgsl").into())
        });

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Chessboard Render Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Chessboard Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    ChessboardVertex::desc()
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        })
    }

    fn create_chessboard_vertex_buffer(device: &wgpu::Device) -> (wgpu::Buffer, u32) {
        const WHITE: [f32; 3] = [0.925, 0.925, 0.800];
        const GREEN: [f32; 3] = [0.450, 0.667, 0.290];

        let mut vertices: Vec<ChessboardVertex> = vec![];

        for i in 0..64 {
            let col = i % 8;
            let row = i / 8;
            let color = if (row + col) % 2 == 0 { WHITE } else { GREEN };

            let x_offset = ((i % 8) as f32) * 0.25;
            let y_offset = ((i / 8) as f32) * 0.25;

            let mut new_vertices: Vec<ChessboardVertex> = vec![
                ChessboardVertex { position: [-1.0 + x_offset, 1.0 - y_offset, 0.0], color },
                ChessboardVertex { position: [-1.0 + x_offset, 0.75 - y_offset, 0.0], color },
                ChessboardVertex { position: [-0.75 + x_offset, 1.0 - y_offset, 0.0], color },
                ChessboardVertex { position: [-0.75 + x_offset, 0.75 - y_offset, 0.0], color },
            ];

            vertices.append(&mut new_vertices);
        }

        let num_vertices = vertices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chessboard Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        });

        (vertex_buffer, num_vertices)
    }

    fn create_piece_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("piece_texture_bind_group_layout"),
        })
    }

    fn create_piece_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layout: &wgpu::BindGroupLayout
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Piece Render Pipeline Layout"),
                bind_group_layouts: &[Some(bind_group_layout)],
                immediate_size: 0,
            }
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("piece_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("piece/piece.wgsl").into())
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Piece Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    PieceVertex::desc()
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        })
    }

    fn create_piece_resources(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        piece: &Piece,
        board_index: usize,
        bind_group_layout: &wgpu::BindGroupLayout
    ) -> PieceGpuResources {
        let diffuse_image = image::load_from_memory(piece.piece_type.get_bytes()).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();

        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("piece_diffuse_texture"),
                view_formats: &[],
            }
        );

        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("piece_diffuse_bind_group"),
            }
        );

        let col = (board_index % 8) as f32;
        let row = (board_index / 8) as f32;
        let x = col * 0.25;
        let y = row * 0.25;

        let vertices: [PieceVertex; 4] = [
            PieceVertex { position: [-1.0 + x, 1.0 - y, 0.0], tex_coords: [0.0, 0.0] },
            PieceVertex { position: [-1.0 + x, 0.75 - y, 0.0], tex_coords: [0.0, 1.0] },
            PieceVertex { position: [-0.75 + x, 1.0 - y, 0.0], tex_coords: [1.0, 0.0] },
            PieceVertex { position: [-0.75 + x, 0.75 - y, 0.0], tex_coords: [1.0, 1.0] },
        ];

        let num_vertices = vertices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Piece Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        PieceGpuResources {
            bind_group: diffuse_bind_group,
            vertex_buffer,
            num_vertices,
        }
    }

    // pub fn update_piece_identity(&mut self, from_index: usize, to_index: usize) {
    //     if let Some(piece_identity) = self.piece_identity_map[from_index] {
    //         self.piece_identity_map[from_index] = None;
    //         self.piece_identity_map[to_index] = Some(piece_identity);
    //     }
    // }

    pub fn rebuild_all_piece_resources(&mut self) {
        let pieces = self.chessboard.get_board_state();
        let piece_texture_bind_group_layout = Self::create_piece_bind_group_layout(&self.device);

        self.piece_bind_groups = std::array::from_fn(|_| None);
        self.piece_vertex_buffers = std::array::from_fn(|_| None);
        self.piece_num_vertices = std::array::from_fn(|_| None);

        for (board_index, piece) in pieces.iter().enumerate() {
            if let Some(piece) = piece {
                let resources = Self::create_piece_resources(
                    &self.device,
                    &self.queue,
                    piece,
                    board_index,
                    &piece_texture_bind_group_layout
                );
                self.piece_bind_groups[board_index] = Some(resources.bind_group);
                self.piece_vertex_buffers[board_index] = Some(resources.vertex_buffer);
                self.piece_num_vertices[board_index] = Some(resources.num_vertices);
            }
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.rebuild_all_piece_resources();

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
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
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

            render_pass.set_pipeline(&self.chessboard_render_pipeline);
            render_pass.set_vertex_buffer(0, self.chessboard_vertex_buffer.slice(..));
            render_pass.draw(0..self.chessboard_num_vertices, 0..1);

            let pieces = self.chessboard.get_board_state();

            for (board_index, piece) in pieces.iter().enumerate() {
                if piece.is_some() {
                    let board_x = (board_index % 8) as f32;
                    let board_y = (board_index / 8) as f32;

                    let x = board_x * 0.25;
                    let y = board_y * 0.25;

                    let vertices: [PieceVertex; 4] = [
                        PieceVertex { position: [-1.0 + x, 1.0 - y, 0.0], tex_coords: [0.0, 0.0] },
                        PieceVertex { position: [-1.0 + x, 0.75 - y, 0.0], tex_coords: [0.0, 1.0] },
                        PieceVertex { position: [-0.75 + x, 1.0 - y, 0.0], tex_coords: [1.0, 0.0] },
                        PieceVertex { position: [-0.75 + x, 0.75 - y, 0.0], tex_coords: [1.0, 1.0] },
                    ];

                    let vertex_buffer = self.piece_vertex_buffers[board_index].as_ref();
                    let bind_group = self.piece_bind_groups[board_index].as_ref();
                    let num_vertices = self.piece_num_vertices[board_index];

                    if let (Some(vertex_buffer), Some(bind_group), Some(num_vertices)) = (vertex_buffer, bind_group, num_vertices) {
                        self.queue.write_buffer(vertex_buffer, 0, bytemuck::cast_slice(&vertices));
                        render_pass.set_pipeline(&self.piece_render_pipeline);
                        render_pass.set_bind_group(0, bind_group, &[]);
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass.draw(0..num_vertices, 0..1);
                    }
                }
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) { event_loop.exit(); }
    }
}


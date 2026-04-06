use std::sync::Arc;

use std::io::Cursor;
use image::ImageReader;
use winit::window::{Window};

use crate::object;

pub struct GpuHandler {
    pub device:           Option<wgpu::Device>,
    pub queue:            Option<wgpu::Queue>,
    pub surface:          Option<wgpu::Surface<'static>>,
    pub surface_config:   Option<wgpu::SurfaceConfiguration>,
    pub compute_pipeline: Option<wgpu::ComputePipeline>,
    pub render_pipeline:  Option<wgpu::RenderPipeline>,
    pub bind_group:       Option<wgpu::BindGroup>,
    pub uniform_buf:      Option<wgpu::Buffer>,
    pub output_buf:       Option<wgpu::Buffer>,
    pub spheres_buf:      Option<wgpu::Buffer>,
    pub quads_buf:        Option<wgpu::Buffer>,
    pub output_buf_size:  Option<u64>,
}

#[allow(unused)]
pub struct GpuState<'a> { // makes my life infinitely easier
    pub device:           &'a wgpu::Device,
    pub queue:            &'a wgpu::Queue,
    pub surface:          &'a wgpu::Surface<'static>,
    pub surface_config:   &'a wgpu::SurfaceConfiguration,
    pub compute_pipeline: &'a wgpu::ComputePipeline,
    pub render_pipeline:  &'a wgpu::RenderPipeline,
    pub bind_group:       &'a wgpu::BindGroup,
    pub uniform_buf:      &'a wgpu::Buffer,
    pub output_buf:       &'a wgpu::Buffer,
    pub spheres_buf:      &'a wgpu::Buffer,
    pub quads_buf:        &'a wgpu::Buffer,
    pub output_buf_size:  &'a u64
}

#[allow(unused)]
pub struct GpuStateMut<'a> { // makes my life infinitely easier
    pub device:           &'a mut wgpu::Device,
    pub queue:            &'a mut wgpu::Queue,
    pub surface:          &'a mut wgpu::Surface<'static>,
    pub surface_config:   &'a mut wgpu::SurfaceConfiguration,
    pub compute_pipeline: &'a mut wgpu::ComputePipeline,
    pub render_pipeline:  &'a mut wgpu::RenderPipeline,
    pub bind_group:       &'a mut wgpu::BindGroup,
    pub uniform_buf:      &'a mut wgpu::Buffer,
    pub output_buf:       &'a mut wgpu::Buffer,
    pub spheres_buf:      &'a mut wgpu::Buffer,
    pub quads_buf:        &'a mut wgpu::Buffer,
    pub output_buf_size:  &'a mut u64,
}

impl GpuHandler {

    // makes it easier to not have to wrap everything in some()
    fn get_state_mut(&mut self) -> Option<GpuStateMut<'_>> {
        Some(
            GpuStateMut {
                device:           self.device.as_mut()?,
                queue:            self.queue.as_mut()?,
                surface:          self.surface.as_mut()?,
                surface_config:   self.surface_config.as_mut()?,
                compute_pipeline: self.compute_pipeline.as_mut()?,
                render_pipeline:  self.render_pipeline.as_mut()?,
                bind_group:       self.bind_group.as_mut()?,
                uniform_buf:      self.uniform_buf.as_mut()?,
                output_buf:       self.output_buf.as_mut()?,
                spheres_buf:      self.spheres_buf.as_mut()?,
                quads_buf:        self.quads_buf.as_mut()?,
                output_buf_size:  self.output_buf_size.as_mut()?,
            }
        )
    }

    pub fn get_state(&self) -> Option<GpuState<'_>> {
        Some(
            GpuState {
                device:           self.device.as_ref()?,
                queue:            self.queue.as_ref()?,
                surface:          self.surface.as_ref()?,
                surface_config:   self.surface_config.as_ref()?,
                compute_pipeline: self.compute_pipeline.as_ref()?,
                render_pipeline:  self.render_pipeline.as_ref()?,
                bind_group:       self.bind_group.as_ref()?,
                uniform_buf:      self.uniform_buf.as_ref()?,
                output_buf:       self.output_buf.as_ref()?,
                spheres_buf:      self.spheres_buf.as_ref()?,
                quads_buf:        self.quads_buf.as_ref()?,
                output_buf_size:  self.output_buf_size.as_ref()?,
            }
        )
    }

    pub fn change_resolution(&mut self, width: u32, height: u32) {
        let gpu = match self.get_state_mut() {
            Some(t) => t,
            None => return
        };

        gpu.surface_config.width = width;
        gpu.surface_config.height = height;

        gpu.surface.configure(&gpu.device, &gpu.surface_config);

        if (width*height) as u64 > *gpu.output_buf_size {
            *gpu.output_buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("output"),
                size: (width * height * 4) as u64, // 4 bytes per pixel
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            
            *gpu.bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &gpu.compute_pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: gpu.uniform_buf.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: gpu.output_buf.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 2, resource: gpu.spheres_buf.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 3, resource: gpu.quads_buf.as_entire_binding() },
                ],
            });

            self.output_buf_size = Some((width * height) as u64);
        }
    }

    // vibecoded but man thats a lot
    pub async fn init(&mut self, window: Arc<Window>, width: u32, height: u32) {
        // --- get a handle to the graphics card ---
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
        }, None).await.unwrap();

        // --- create the surface ---

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: width,
            height: height,
            present_mode: wgpu::PresentMode::Mailbox,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };

        surface.configure(&device, &surface_config);

        // --- textures ---
        let texture_size = wgpu::Extent3d {
            width: 16,
            height: 16,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Dirt"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let rgba_data = ImageReader::open("textures/dirt.png").expect("No image").decode().expect("Bad decode").to_rgba8().into_raw();

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_data, // Your pixel bytes
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * 16), // 4 bytes per pixel * width
                rows_per_image: Some(16),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // --- buffers ---

        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform"),
            size: std::mem::size_of::<object::camera::GpuUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // output buffer — one u32 per pixel
        let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("output"),
            size: (width * height * 4) as u64, // 4 bytes per pixel
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let spheres_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("spheres"),
            size: (std::mem::size_of::<object::sphere::GpuSphere>() * 512) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let quads_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("quads"),
            size: (std::mem::size_of::<object::quad::GpuQuad>() * 512) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // --- pipelines ---

        let source = format!(
            "{}{}{}{}",
            include_str!("./shaders/types.wgsl"),
            include_str!("./shaders/intersection.wgsl"),
            include_str!("./shaders/material.wgsl"),
            include_str!("./shaders/main.wgsl")
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("raytracer"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // camera
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // output
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // spheres
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<object::sphere::GpuSphere>() as u64),
                    },
                    count: None,
                },
                // quads
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<object::quad::GpuQuad>() as u64),
                    },
                    count: None,
                },
                // textures
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE, // Usually read in fragment shaders
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                }
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: output_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: spheres_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: quads_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&view)},
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[]
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        // minimal render pipeline — just blits the output buffer to screen
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blit"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multiview: None,
            multisample: wgpu::MultisampleState::default(),
            cache: None,
        });

        // i opted to set them all at the end so im not wrapping everything in Some() and .unwrap()
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
        self.compute_pipeline = Some(compute_pipeline);
        self.render_pipeline = Some(render_pipeline);
        self.bind_group = Some(bind_group);
        self.uniform_buf = Some(uniform_buf);
        self.output_buf = Some(output_buf);
        self.spheres_buf = Some(spheres_buf);
        self.quads_buf = Some(quads_buf);
        self.output_buf_size = Some((width as u64)*(height as u64));
    }
}

impl Default for GpuHandler {
    fn default() -> Self {
        Self {
            device:           None,
            queue:            None,
            surface:          None,
            surface_config:   None,
            compute_pipeline: None,
            render_pipeline:  None,
            bind_group:       None,
            uniform_buf:      None,
            output_buf:       None,
            spheres_buf:      None,
            quads_buf:        None,
            output_buf_size:  None,
        }
    }
}

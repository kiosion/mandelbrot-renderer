use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use wgpu::{util::DeviceExt, ShaderModuleDescriptor, ShaderSource};

const SHADER: &'static [u8] = include_bytes!("shader/shader.spv");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Date)]
    pub fn now() -> f64;
}

#[wasm_bindgen]
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
    output_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Renderer, JsValue> {
        // TODO: Way more performant here to grab reference to canvas
        // and use that later, rather than finding element each render pass

        // let window = window().unwrap();
        // let document = window.document().unwrap();
        // let canvas = document.get_element_by_id("canvas").unwrap();
        // let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        // let context = canvas.get_context("gpupresent")?.unwrap();
        // let context = context.dyn_into::<wgpu::Surface>().unwrap();
        // let context = canvas
        //   .get_context("webgpu")
        //   .unwrap()
        //   .unwrap()
        //   .dyn_into::<web_sys::WebGl2RenderingContext>()
        //   .unwrap();

        // Establish connection to the GPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
        }).await.unwrap();

        // Request a connection to the device (GPU)
        let (device, queue) = adapter.request_device(
          &wgpu::DeviceDescriptor {
              label: Some("Device"),
              features: wgpu::Features::empty(),
              limits: wgpu::Limits::default(),
          },
          None, // change from None to use specific queue
        ).await.unwrap();

        let shader_module = device.create_shader_module(&ShaderModuleDescriptor {
          label: Some("Shader Module"),
          source: ShaderSource::SpirV(std::borrow::Cow::Borrowed(unsafe {
              std::slice::from_raw_parts(SHADER.as_ptr() as *const u32, SHADER.len() / 4)
          })),
          flags: wgpu::ShaderFlags::VALIDATION,
       });

        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader_module,
            entry_point: "main",
        });

        // let input_data = vec![0u32; 8];
        // let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Input Buffer"),
        //     contents: bytemuck::cast_slice(&input_data),
        //     usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
        // });

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: (4*1024*1024) as u64,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_SRC,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(4),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(4),
                    },
                    count: None,
                },
            ],
            label: Some("bind group layout"),
        });

        // let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     label: Some("Bind Group"),
        //     layout: &bind_group_layout,
        //     entries: &[
        //         wgpu::BindGroupEntry {
        //             binding: 0,
        //             resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
        //                 buffer: &input_buffer,
        //                 offset: 0,
        //                 size: wgpu::BufferSize::new(32),
        //             }),
        //         },
        //         wgpu::BindGroupEntry {
        //             binding: 1,
        //             resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
        //                 buffer: &output_buffer,
        //                 offset: 0,
        //                 size: wgpu::BufferSize::new(4 * 1024 * 1024),
        //             }),
        //         },
        //     ],
        // });

        Ok(Self {
            device,
            queue,
            compute_pipeline,
            bind_group_layout,
            output_buffer,
        })
    }

    #[wasm_bindgen]
    pub async fn compute(&self, width: u32, height: u32, max_iter: u32, center_x: f64, center_y: f64, zoom: f64) -> Result<(), JsValue> {
        // Create an input buffer and write data into it.
        let input_data: [u32; 6] = [
            width,
            height,
            max_iter,
            center_x.to_bits() as u32,
            center_y.to_bits() as u32,
            zoom.to_bits() as u32,
        ];
        let input_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Input Buffer"),
            contents: bytemuck::cast_slice(&input_data),
            usage: wgpu::BufferUsage::STORAGE,
        });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &input_buffer,
                        offset: 0,
                        size: wgpu::BufferSize::new(24),
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.output_buffer,
                        offset: 0,
                        size: wgpu::BufferSize::new((4 * width * height).into()),
                    }),
                },
            ],
        });

        // new command encoder.
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        });

        // compute pass in its own block to limit scope
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch(width, height, 1);
        }

        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = self.output_buffer.slice(..);
        // let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);
        self.device.poll(wgpu::Maintain::Wait);
        let data = buffer_slice.get_mapped_range();

        // assume the data format in buffer is RGBA8
        let rgba_data: Vec<u8> = unsafe {
            std::slice::from_raw_parts(data.as_ptr(), data.len())
                .chunks_exact(4)
                .map(|bgra| vec![bgra[2], bgra[1], bgra[0], bgra[3]])  // Swap color channels
                .flatten()
                .collect()
        };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
        let context = canvas.get_context("2d")?.unwrap();
        let context = context.dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&rgba_data),
            width,
            height,
        ).unwrap();

        context.put_image_data(&image_data, 0.0, 0.0)?;

        // unmap the buffer before dropping the data.
        self.output_buffer.unmap();

        Ok(())
    }
}

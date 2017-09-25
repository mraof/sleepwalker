#[macro_use]
extern crate gfx;
extern crate gfx_support;
extern crate rlua;
extern crate image;

use gfx::{Bundle, Device, Frame, GraphicsCommandPool, GraphicsPoolExt};
use gfx::queue::GraphicsQueue;
use gfx_support::{Application, BackbufferView, ColorFormat, SyncPrimitives, WindowTargets};
use gfx_support::shade::{Backend, Source};

use std::time::Instant;

#[test]
fn graphics_test() {
    App::launch_simple("Meowing time");
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }
    
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
        time: gfx::Global<f32> = "f_Time",
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    }
}

struct App<B: gfx::Backend> {
    views: Vec<BackbufferView<B::Resources>>,
    bundle: Bundle<B, pipe::Data<B::Resources>>,
    start_time: Instant,
}

impl<B: gfx::Backend> Application<B> for App<B> {
    fn new(device: &mut B::Device, _: &mut GraphicsQueue<B>, backend: Backend, window_targets: WindowTargets<B::Resources>) -> Self {
        use gfx::traits::DeviceExt;
        let pso = {
            let vs = Source {
                glsl_130: include_bytes!("assets/shaders/rect_150.vert"),
                ..Source::empty()
            };
            let ps = Source {
                glsl_130: include_bytes!("assets/shaders/rect_150.frag"),
                ..Source::empty()
            };
            
            device.create_pipeline_simple(vs.select(backend).unwrap(),
                                          ps.select(backend).unwrap(),
                                          pipe::new())
                .unwrap()
        };
        
        let (vertex_buffer, slice) = {
            let triangle = [
                Vertex {
                    pos: [0.0, 0.0],
                    uv: [0.0, 1.0],
                },
                Vertex {
                    pos: [0.457, 0.0],
                    uv: [1.0, 1.0],
                },
                Vertex {
                    pos: [0.457, 0.69],
                    uv: [1.0, 0.0],
                },
                Vertex {
                    pos: [0.0, 0.0],
                    uv: [0.0, 1.0],
                },
                Vertex {
                    pos: [0.0, 0.69],
                    uv: [0.0, 0.0],
                },
                Vertex {
                    pos: [0.457, 0.69],
                    uv: [1.0, 0.0],
                },
            ];
            device.create_vertex_buffer_with_slice(&triangle, ())
        };
        let texture = {
            let bytes = include_bytes!("../assets/sprites/sbnkalny.png");
            let img = image::load_from_memory(bytes).unwrap().to_rgba();
            let (width, height) = img.dimensions();
            let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
            let (_, view) = device.create_texture_immutable_u8::<gfx::format::Srgba8>(kind, &[&img]).unwrap();
            view
        };
        let sampler_info = gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale, gfx::texture::WrapMode::Clamp);
        let sampler = device.create_sampler(sampler_info);
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: window_targets.views[0].0.clone(),
            time: 0.0,
            texture: (texture, sampler),
        };
        
        App {
            views: window_targets.views,
            bundle: Bundle::new(slice, pso, data),
            start_time: Instant::now(),
        }
    }

    fn render(&mut self,
              (_, sync): (Frame, &SyncPrimitives<B::Resources>),
              pool: &mut GraphicsCommandPool<B>,
              queue: &mut GraphicsQueue<B>) {
        let time = self.start_time.elapsed();
        
        let mut encoder = pool.acquire_graphics_encoder();
        self.bundle.data.time = (time.as_secs() as f32) + (time.subsec_nanos() / 1000000) as f32 / 1000.0;
        const CLEAR_COLOR: [f32; 4] = [0.0, 0.005, 0.005, 1.0];
        
        
        encoder.clear(&self.bundle.data.out, CLEAR_COLOR);
        self.bundle.encode(&mut encoder);
        encoder.synced_flush(queue, &[&sync.rendering], &[], Some(&sync.frame_fence))
            .expect("Could not flush encoder");
    }

    fn on_resize(&mut self, window_targets: WindowTargets<B::Resources>) {
        self.views = window_targets.views;
    }
}

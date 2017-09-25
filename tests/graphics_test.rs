#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin as gfx_glutin;
extern crate glutin;
extern crate rlua;

use rlua::{Lua, UserData, UserDataMethods};

use gfx::traits::FactoryExt;
use gfx::Device;
use glutin::GlContext;

use std::rc::Rc;
use std::cell::RefCell;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

#[derive(Debug, Copy, Clone)]
struct Square {
    pub pos: (f32, f32),
    pub size: f32,
    pub color: [f32; 3],
}

#[derive(Debug)]
struct Psuedocube {
    squares: Vec<Square>,
    ratio: f32,
}

impl Psuedocube {
    pub fn new() -> Psuedocube {
        Psuedocube {
            squares: vec![],
            ratio: 1.0,
        }
    }

    pub fn add_square(&mut self, x: f32, y: f32, size: f32, color: [f32; 3]) {
        let square = Square {
            pos: (x, y),
            size,
            color,
        };
        self.squares.push(square);
    }

    pub fn get_vertices_indices(&self) -> (Vec<Vertex>, Vec<u16>) {
        let mut vs = Vec::new();
        let mut is = Vec::new();
        for (i, sq) in self.squares.iter().enumerate() {
            let pos = sq.pos;
            let half = 0.5 * sq.size;
            let i = i as u16;
            let (hx, hy) = if self.ratio > 1.0 {
                (half / self.ratio, half)
            } else {
                (half, half * self.ratio)
            };

            vs.extend(
                &[
                    Vertex {
                        pos: [pos.0 + hx, pos.1 - hy],
                        color: sq.color,
                    },
                    Vertex {
                        pos: [pos.0 - hx, pos.1 - hy],
                        color: sq.color,
                    },
                    Vertex {
                        pos: [pos.0 - hx, pos.1 + hy],
                        color: sq.color,
                    },
                    Vertex {
                        pos: [pos.0 + hx, pos.1 + hy],
                        color: sq.color,
                    },
                ],
            );
            is.extend(&[4 * i, 4 * i + 1, 4 * i + 2, 4 * i + 2, 4 * i + 3, 4 * i]);
        }
        (vs, is)
    }

    pub fn update_ratio(&mut self, ratio: f32) {
        self.ratio = ratio;
    }
}

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

#[test]
fn graphics_test() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Graphics Test".to_string())
        .with_dimensions(800, 800);
    let context_builder = glutin::ContextBuilder::new().with_vsync(true);
    let (window, mut device, mut factory, main_color, mut main_depth) = gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory
        .create_pipeline_simple(
            include_bytes!("assets/shaders/rect_150.vert"),
            include_bytes!("assets/shaders/rect_150.frag"),
            pipe::new(),
        )
        .unwrap();

    let cube = Psuedocube::new();
    let (vertices, indices) = cube.get_vertices_indices();
    let (vertex_buffer, mut slice) = factory.create_vertex_buffer_with_slice(&vertices, &*indices);
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };
    let cube = CubeWrapper { cube: Rc::new(RefCell::new(cube)) };
    {
        let mut cube = cube.cube.borrow_mut();
        cube.add_square(0.0, 0.0, 1.0, WHITE);
    }

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("cube", cube.clone()).unwrap();

    let mut running = true;
    let mut needs_update = true;
    let mut ticks = 0;
    while running {
        globals.set("ticks", ticks).unwrap();
        needs_update |= lua.exec::<bool>(
            r#"
if ticks % 50 == 0 then
    local i = (ticks / 50) % 10 + 1
    cube:add_square(0.075 * i, 0.05 * i - (ticks / 5000), 0.1 * (11 - i), 0.1 * i, 0.05 * i, 0.1 * i)
    return true
end
return false
            "#,
            None,
        ).unwrap();
        if needs_update {
            let cube = cube.cube.borrow();
            let (vs, is) = cube.get_vertices_indices();
            let (vbuf, sl) = factory.create_vertex_buffer_with_slice(&vs, &*is);
            data.vbuf = vbuf;
            slice = sl;

            needs_update = false
        }
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { window_id, event } => {
                use glutin::WindowEvent::*;
                match event {
                    Closed => running = false,
                    Resized(w, h) => {
                        gfx_glutin::update_views(&window, &mut data.out, &mut main_depth);
                        let mut cube = cube.cube.borrow_mut();
                        cube.update_ratio(w as f32 / h as f32);
                        needs_update = true;
                    }
                    _ => (),
                }
            }
            _ => (),
        });
        encoder.clear(&data.out, BLACK);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
        ticks += 1;
    }
}

#[derive(Clone)]
struct CubeWrapper {
    cube: Rc<RefCell<Psuedocube>>,
}

impl UserData for CubeWrapper {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method("add_square", |_,
         cube: &CubeWrapper,
         (x, y, size, r, g, b): (f32,
                                 f32,
                                 f32,
                                 f32,
                                 f32,
                                 f32)| {
            let mut cube = cube.cube.borrow_mut();
            cube.add_square(x, y, size, [r, g, b]);
            Ok(())
        });
    }
}

use std::fs::File;
use std::io::Read;
use std::borrow::Cow;


use glium;
use glium::{ Surface, Display, VertexBuffer, Rect };
use glium::texture::{ Texture2d, ClientFormat, RawImage2d };
use glium::backend::Facade;

use tile_net::TileNet;
use world::World;


pub struct Graphics<'a> {
    display: &'a Display,
    world: &'a World,

    // OpenGL 
    shader_prg: glium::Program,
    quad_vbo: VertexBuffer<Vertex>,
    texture: Texture2d,
}

impl<'a> Graphics<'a> {
    pub fn new(display: &'a Display, world: &'a World) -> Graphics<'a> {
        let shader_prg = create_program(display, "xyuv_tex");
        let fullscreen_quad = vec![ Vertex { pos: [-1.0, -1.0], texpos: [0.0, 0.0]},
                                    Vertex { pos: [1.0, -1.0],  texpos: [1.0, 0.0]},
                                    Vertex { pos: [1.0, 1.0],   texpos: [1.0, 1.0]},

                                    Vertex { pos: [1.0, 1.0],   texpos: [1.0, 1.0]},
                                    Vertex { pos: [-1.0, 1.0],  texpos: [0.0, 1.0]},
                                    Vertex { pos: [-1.0, -1.0], texpos: [0.0, 0.0]}];

        let quad_vbo = glium::VertexBuffer::new(display, &fullscreen_quad).unwrap();
        let texture_data: Vec<Vec<u8>> = vec!(vec!(0; world.get_width()); world.get_height());
        let texture = glium::texture::Texture2d::new(display, texture_data).unwrap();

        let mut new = Graphics {
            display: display,
            world: world,

            shader_prg: shader_prg,
            quad_vbo: quad_vbo,
            texture: texture,
        };
        new.upload_world();
        new
    }

    pub fn render(&mut self, center_x: u32, center_y: u32, width: u32, height: u32) {
        let mut target = self.display.draw();        // target: glium::Frame
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // RENDER 

        let uniforms = uniform! (
            sampler: &self.texture,
        );
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.draw(self.quad_vbo.slice(0..6).unwrap(), indices, &self.shader_prg, &uniforms, &Default::default()).unwrap();


        // END

        target.finish().unwrap(); 
    }

    fn upload_world(&mut self) {
        let upload_area = Rect { left: 0, bottom: 0, width: self.world.get_width() as u32, height: self.world.get_height() as u32};
        let upload_data = RawImage2d {
            data: Cow::Borrowed(self.world.get_tiles().get_raw()),
            width: self.world.get_width() as u32,
            height: self.world.get_height() as u32,
            format: ClientFormat::U32,
        };


        self.texture.write(upload_area, upload_data);
    }

}

// For rendering
#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    texpos: [f32; 2],
}

implement_vertex!(Vertex, pos, texpos);


//// Helpers ////
pub fn create_program<'a, F>(display: &'a F, name: &'static str) -> glium::Program
    where F: glium::backend::Facade
{
    let mut f = File::open("shaders/".to_string() + name + ".vert").unwrap();
    let mut vert_src = String::new();
    f.read_to_string(&mut vert_src);
    f = File::open("shaders/".to_string() + name + ".frag").unwrap();
    let mut frag_src = String::new();
    f.read_to_string(&mut frag_src);

    glium::Program::from_source(display, vert_src.as_str(), frag_src.as_str(), None).unwrap()
}


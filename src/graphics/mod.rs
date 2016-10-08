
use glium;
use glium::{ Surface, Display, VertexBuffer };
use tile_net::TileNet;
use world::World;
use glium::backend::Facade;

pub struct Graphics<'a> {
    display: &'a Display,
    world: &'a World,
    fullscreen_quad: Vec<Vertex>,

    // OpenGL 
    quad_vbo: VertexBuffer<Vertex>,
}

impl<'a> Graphics<'a> {
    pub fn new(display: &'a Display, world: &'a World) -> Graphics<'a> {
        let vertex1 = Vertex { position: [-1.0, -1.0]};
        let fullscreen_quad = vec![ Vertex { position: [-1.0, -1.0]},
                                    Vertex { position: [1.0, -1.0]},
                                    Vertex { position: [1.0, 1.0]},

                                    Vertex { position: [1.0, 1.0]},
                                    Vertex { position: [-1.0, 1.0]},
                                    Vertex { position: [-1.0, -1.0]}];

        let vertex_buffer = glium::VertexBuffer::new(&display as &glium::backend::Facade, &fullscreen_quad).unwrap();
        Graphics {
            display: display,
            world: world,
            fullscreen_quad: fullscreen_quad,
            quad_vbo: vertex_buffer,
        }
    }

    pub fn render(&mut self, center_x: u32, center_y: u32, width: u32, height: u32) {
        let mut target = self.display.draw();        // target: glium::Frame
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // RENDER 

        // END

        target.finish().unwrap(); 
    }

    fn upload_world(&mut self) {

    }
}

// For rendering
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

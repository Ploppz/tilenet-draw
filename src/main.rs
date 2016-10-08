extern crate tile_net;
#[macro_use]
extern crate glium;

use tile_net::TileNet;

use glium::{ DisplayBuild, glutin };
use glium::glutin::{Event, MouseScrollDelta, ElementState, MouseButton, WindowBuilder};
use glium::backend::glutin_backend::GlutinFacade;

use graphics::Graphics;
use world::World;

pub mod graphics;
pub mod world;


fn main() {
    let mut world = World::new(30, 30);
    let display = glutin::WindowBuilder::new().build_glium().unwrap();
    let mut graphics = Graphics::new(&display, &world);
    loop {
        let window_size = display.get_window().unwrap().get_inner_size().unwrap();
        graphics.render(0, 0, window_size.0, window_size.1);
    }
}

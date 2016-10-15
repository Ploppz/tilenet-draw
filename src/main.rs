#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate tile_net;
#[macro_use]
extern crate glium;

use tile_net::TileNet;

use glium::{ DisplayBuild, glutin };

use graphics::Graphics;
use world::World;

pub mod graphics;
pub mod world;


fn main() {
    let mut world = World::new(300, 300);
    // world.get_tiles_mut().set_row(&255, 2);
    xor_pattern(world.get_tiles_mut());

    let display = glutin::WindowBuilder::new().build_glium().unwrap();
    let mut graphics = Graphics::new(&display, &world);
    loop {
        let window_size = display.get_window().unwrap().get_inner_size().unwrap();
        graphics.render(-30.0, -30.0, window_size.0, window_size.1);
    }
}

fn xor_pattern(tiles: &mut TileNet<u8>) {
    let tile_size = tiles.get_size();
    for x in 0..tile_size.0 {
        for y in 0..tile_size.1 {
            tiles.set(&( (x ^ y) as u8 ), (x, y));
        }
    }
}

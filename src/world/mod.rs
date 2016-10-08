use tile_net::TileNet;


#[derive(Clone, Debug, Default)]
struct Tile(bool);

pub struct World {
    tiles: TileNet<Tile>,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        World {
            tiles: TileNet::<Tile>::new((width, height)),
        }
    }
    pub fn print(&self) {
        println!("{:?}", self.tiles);
    }
}

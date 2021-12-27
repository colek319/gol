use gol::{Cell, World};

fn main() {
    println!("Hello, world!");
    let mut world = World::new(640, 480);
    world.start();
}
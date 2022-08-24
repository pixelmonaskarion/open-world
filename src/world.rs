use crate::object::*;
#[derive(Debug)]
#[derive(Clone)]
pub struct World {
    pub heights: Vec<f32>,
    pub size: i32,
}

#[derive(Debug)]
pub struct WorldView {
    pub cubes: Vec<Cube>,
    pub spheres: Vec<Sphere>,
    pub world: World,
}

impl WorldView {
    pub fn new(cubes: Vec<Cube>, spheres: Vec<Sphere>, world: World) -> Self{
        Self {
            cubes,
            spheres,
            world,
        }
    }
}
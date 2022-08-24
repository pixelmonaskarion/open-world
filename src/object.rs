#[derive(Debug)]
pub struct Sphere {
    pub pos: [f32; 3],
    pub radius: f32,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(pos: [f32; 3], radius: f32) -> Self {
        Self {
            pos,
            radius,
        }
    }

    pub fn colliding_point(&self, point: [f32; 3]) -> bool {
        ((point[0]-self.pos[0]).powf(2.0)+(point[1]-self.pos[1]).powf(2.0)+(point[2]-self.pos[2]).powf(2.0)).sqrt() < self.radius
    }
}

#[derive(Debug)]
pub struct Cube {
    pub pos: [f32; 3],
    pub size: [f32; 3],
}

impl Cube {
    pub fn new(pos: [f32; 3], size: [f32; 3]) -> Self {
        Self {
            pos,
            size,
        }
    }

    #[allow(dead_code)]
    pub fn colliding_point(&self, point: [f32; 3], mask: [f32; 3]) -> (bool, [f32; 3]) {
        let colliding = point[0] > self.pos[0]-self.size[0]/2.0 && point[0] < self.pos[0]+self.size[0]/2.0 && point[1] > self.pos[1]-self.size[1]/2.0 && point[1] < self.pos[1]+self.size[1]/2.0 && point[2] > self.pos[2]-self.size[2]/2.0 && point[2] < self.pos[2]+self.size[2]/2.0 ;
        if colliding {
            let delta = [point[0]-self.pos[0], point[1]-self.pos[1], point[2]-self.pos[2]];
            let x = ((self.size[0]/2.0)*(delta[0]/delta[0].abs()))-delta[0];
            let y = ((self.size[1]/2.0)*(delta[1]/delta[1].abs()))-delta[1];
            let z = ((self.size[2]/2.0)*(delta[2]/delta[2].abs()))-delta[2];
            return (colliding, [x*mask[0], y*mask[1], z*mask[2]]);
        } else {
            return (colliding, [0.0,0.0,0.0]);
        }
    }

    pub fn colliding_box(&self, other: Cube, mask: [f32; 3]) -> (bool, [f32; 3]){
        let colliding = other.pos[0]+other.size[0]/2.0 > self.pos[0]-self.size[0]/2.0 && other.pos[0]-other.size[0]/2.0 < self.pos[0]+self.size[0]/2.0 &&
        other.pos[1]+other.size[1]/2.0 > self.pos[1]-self.size[1]/2.0 && other.pos[1]-other.size[1]/2.0 < self.pos[1]+self.size[1]/2.0 && 
        other.pos[2]+other.size[2]/2.0 > self.pos[2]-self.size[2]/2.0 && other.pos[2]-other.size[2]/2.0 < self.pos[2]+self.size[2]/2.0;
        if colliding {
            let abs_things = [(self.pos[0]-other.pos[0])/(self.pos[0]-other.pos[0]).abs(), 
            (self.pos[1]-other.pos[1])/(self.pos[1]-other.pos[1]).abs(),
            (self.pos[2]-other.pos[2])/(self.pos[2]-other.pos[2]).abs()];
            let delta = [self.pos[0]+abs_things[0]*self.size[0]/2.0-abs_things[0]*other.size[0]/2.0,
            self.pos[1]+abs_things[1]*self.size[1]/2.0-abs_things[1]*other.size[1]/2.0,
            self.pos[2]+abs_things[2]*self.size[2]/2.0-abs_things[2]*other.size[2]/2.0];
            let x = ((self.size[0]/2.0)*(delta[0]/delta[0].abs()))-delta[0];
            let y = ((self.size[1]/2.0)*(delta[1]/delta[1].abs()))-delta[1];
            let z = ((self.size[2]/2.0)*(delta[2]/delta[2].abs()))-delta[2];
            return (colliding, [x*mask[0], y*mask[1], z*mask[2]]);
        } else {
            return (colliding, [0.0,0.0,0.0]);
        }
    }
}
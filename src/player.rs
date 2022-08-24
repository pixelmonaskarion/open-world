use std::time::Duration;
use crate::camera::*;
use crate::world::*;
use cgmath::*;
use crate::object::*;
use std::f32::consts::FRAC_PI_2;
// use winit::dpi::PhysicalPosition;
use winit::event::*;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[allow(dead_code)]
pub struct Player {
    pub camera: Camera,
    pub camera_controller: PlayerController,
}

#[deprecated(since="0.5.0", note="please use PlayerController instead")]
impl Player {
    #[allow(dead_code)]
    pub fn new(_position: [f32; 3]) {
        /*let camera = Camera::new(position, cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let camera_controller = PlayerController::new(camera.copy(), 4.0, 1.0);
        Self {
            camera,
            camera_controller,
        }*/
    }
    #[allow(dead_code)]
    pub fn update(&mut self, dt: Duration) {
        //self.camera_controller.update_camera(&mut self.camera, dt);
        let _dt = dt.as_secs_f32();
    }
}

pub fn new_player_camera(position: [f32; 3]) -> Camera {
    Camera::new(position, cgmath::Deg(-90.0), cgmath::Deg(-20.0))
}

pub const SPECTATOR: bool = false;

#[derive(Debug)]
pub struct PlayerController {
    pub camera: Camera,
    move_amounts: [f32; 6],
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
    delta: [f32; 3],
    in_air: bool,
    gravity: f32,
    world: Option<WorldView>,

}

const MAX_SLOPE: f32 = 0.5;

impl PlayerController {
    pub fn new(camera: Camera, speed: f32, sensitivity: f32, world: Option<WorldView>) -> Self {
        Self {
            camera,
            move_amounts: [0.0,0.0,0.0,0.0,0.0,0.0],
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
            delta: [0.0,0.0,0.0],
            in_air: true,
            gravity: 0.01,
            world,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.move_amounts[0] = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.move_amounts[1] = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.move_amounts[2] = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.move_amounts[3] = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.move_amounts[4] = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.move_amounts[5] = amount;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, _delta: &MouseScrollDelta) {
        /*self.scroll = match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => -scroll * 0.5,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => -*scroll as f32,
        };*/
    }

    fn move_x(&mut self, dx: f32) {
        if dx != 0.0 {
            self.camera.position.x += dx;
            /*while self.colliding([1.0,0.0,0.0]) {
                self.delta[0] = 0.0;
                self.camera.position.x -= (dx/dx.abs())*0.001;
                self.move_up_slope();
            }*/
            let result = self.colliding([1.0,0.0,0.0]);
            if result.0 {
                self.delta[0] = 0.0;
                self.camera.position.x += result.1[0];
                self.move_up_slope();
            }
        }
    }

    fn move_y(&mut self, dy: f32) {
        if dy != 0.0 {
            self.camera.position.y += dy;
            let result = self.colliding([0.0,1.0,0.0]);
            //println!("{} {} {} {}", result.0, result.1[0], result.1[1], result.1[2]);
            //println!("player pos is {} {} {}", self.camera.position.x, self.camera.position.y, self.camera.position.z);
            if result.0 {
                self.in_air = false;
                self.delta[1] = 0.0;
                self.camera.position.y += result.1[1];
            } else {
                self.in_air = true;
            }
        }
    }

    fn move_z(&mut self, dz: f32) {
        if dz != 0.0 {
            self.camera.position.z += dz;
            let result = self.colliding([0.0,0.0,1.0]);
            if result.0 {
                self.delta[2] = 0.0;
                self.camera.position.z += result.1[2];
                self.move_up_slope();
            }
        }
    }

    fn move_up_slope(&mut self) {
        /*let start_y = self.camera.position.y;
        while self.camera.position.y-start_y <= MAX_SLOPE {
            self.camera.position.y += 0.01;
            if !self.colliding() {
                break;
            }
        }
        if self.colliding() {
            self.camera.position.y = start_y;
        }*/
    }

    fn move_all(&mut self, delta: [f32; 3]) {
        self.move_x(delta[0]);
        self.move_y(delta[1]);
        self.move_z(delta[2]);
    }

    fn move_vec(&mut self, delta: Vector3<f32>) {
        self.move_x(delta[0]);
        self.move_y(delta[1]);
        self.move_z(delta[2]);
    }

    pub fn set_world_view(&mut self, world: WorldView) {
        self.world = Some(world);
    } 

    fn colliding(&self, mask: [f32; 3]) -> (bool, [f32; 3]) {
        if SPECTATOR {
            return (false, [0.0,0.0,0.0]);
        }
        if self.world.is_some() {
            for cube in &self.world.as_ref().unwrap().cubes {
                let result = cube.colliding_box(Cube {
                    pos: self.camera.position.into(),
                    size: [3.0, 3.0, 3.0],
                    }, mask);
                if result.0 {
                    return result;
                }
            }
            for sphere in &self.world.as_ref().unwrap().spheres {
                if sphere.colliding_point(self.camera.position.into()) {
                    return (true, [0.0,0.0,0.0]);
                }
            }
        }
        let world_ref = &self.world.as_ref().unwrap().world;
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    let real_x = self.camera.position.x+x as f32;
                    let real_y = self.camera.position.y+y as f32;
                    let real_z = self.camera.position.z+z as f32;
                    if real_x >= 0.0 && real_x < (world_ref.size-1) as f32 && real_z >= 0.0 && real_z < (world_ref.size-1) as f32 {
                        let sub_x = real_x-((real_x as i32) as f32);
                        let sub_z = real_z-((real_z as i32) as f32);
                        let height_here = world_ref.heights[(world_ref.size*real_x as i32 + real_z as i32) as usize];
                        let next_height_x1 = world_ref.heights[(world_ref.size*real_x as i32 + real_z as i32 + 1) as usize];
                        let next_height_z = world_ref.heights[(world_ref.size*(1+real_x as i32) + real_z as i32) as usize];
                        let next_height_x2 = world_ref.heights[(world_ref.size*(1+real_x as i32) + real_z as i32 + 1) as usize];
                        let height_delta_x1 = next_height_x1-height_here;
                        let height_delta_x2 = next_height_x2-next_height_z;
                        let exact_height_x1 = height_here+sub_x*height_delta_x1;
                        let exact_height_x2 = next_height_z+sub_x*height_delta_x2;
                        let exact_delta = exact_height_x2-exact_height_x1;
                        let height = exact_height_x1+sub_z*exact_delta;
                        if real_y < height {
                            return (true, [0.0,1.0,0.0]);
                        }
                    }
                }
            }
        }
        return (false,[0.0,0.0,0.0]);
    }

    fn get_jump_velocity(&self, height: f32) -> f32 {
        let val = (self.gravity*height).sqrt();
        return val;
    }

    pub fn update(&mut self, dt: Duration) {
        let dt = dt.as_secs_f32();
        if !SPECTATOR {
            self.delta[1] -= self.gravity;
            if !self.in_air && self.move_amounts[4] == 1.0 {
                self.delta[1] = self.get_jump_velocity(5.0);//0.1;
            }
        }
        self.move_all(self.delta);
        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = self.camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        //self.camera.position += forward * (self.move_amounts[0] - self.move_amounts[1]) * self.speed * dt;
        //self.camera.position += right * (self.move_amounts[3] - self.move_amounts[2]) * self.speed * dt;
        self.move_vec(forward * (self.move_amounts[0] - self.move_amounts[1]) * self.speed * dt);
        self.move_vec(right * (self.move_amounts[3] - self.move_amounts[2]) * self.speed * dt);
        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = self.camera.pitch.0.sin_cos();
        let scrollward =
            Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        self.camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        if SPECTATOR {
            self.camera.position.y += (self.move_amounts[4] - self.move_amounts[5]) * self.speed * dt;
        }
        //self.move_y((self.move_amounts[4] - self.move_amounts[5]) * self.speed * dt);

        // Rotate
        self.camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        self.camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if self.camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            self.camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if self.camera.pitch > Rad(SAFE_FRAC_PI_2) {
            self.camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }
}
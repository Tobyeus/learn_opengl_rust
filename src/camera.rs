use cgmath::{Point3, Vector3, Vector2, Rad, Angle, InnerSpace, Matrix4};

#[derive(PartialEq)]
pub enum CameraMovement {
    Forward,
    Backward,
    Right,
    Left
}

pub struct Camera {
    pub position: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub cursor_pos: Vector2<f32>,
    pub pitch: f32,
    pub yaw: f32
}

// default camera settings
const CAMERA_SPEED: f32 = 2.5;
const CAMERA_SENSE: f32 = 0.001;
const CAMERA_PITCH: f32 = 0.0;
const CAMERA_YAW: f32 = -90.0;

impl Camera {
    pub fn new(camera_position: Point3<f32>, front_vector: Vector3<f32>, up_vector: Vector3<f32>, inital_cursor_pos: Vector2<f32>) -> Self {
        Camera { 
            position: camera_position,
            front: front_vector,
            up: up_vector,
            cursor_pos: inital_cursor_pos,
            pitch: CAMERA_PITCH,
            yaw: CAMERA_YAW
        }
    }

    pub fn process_movement(&mut self, movement: CameraMovement, delta_time: f32) {

        if movement == CameraMovement::Forward {
            self.position += CAMERA_SPEED * delta_time * self.front;
        }
        if movement == CameraMovement::Backward {
            self.position -= CAMERA_SPEED * delta_time * self.front;
        }
        if movement == CameraMovement::Right {
            self.position -= CAMERA_SPEED * delta_time * Vector3::cross(self.up, self.front).normalize();
        }
        if movement == CameraMovement::Left {
            self.position += CAMERA_SPEED * delta_time * Vector3::cross(self.up, self.front).normalize();
        }
    }

    pub fn process_cursor(&mut self, x_new: f32, y_new: f32) {
        //offset between old and new cursor position
        let mut x_off_set = x_new - self.cursor_pos.x;
        let mut y_off_set = y_new - self.cursor_pos.y;

        //update new position in camera
        self.cursor_pos.x = x_new;
        self.cursor_pos.y = y_new;

        //multiply offset with sensitivity factor
        x_off_set *= CAMERA_SENSE;
        y_off_set *= CAMERA_SENSE;

        //add offset to yaw and pitch
        self.yaw += x_off_set;
        self.pitch -= y_off_set;

        //make sure we don't have gimbal lock
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        //calculate direction vector
        let direction = Vector3::new(
            Rad(self.yaw).cos() * Rad(self.pitch).cos(),
            Rad(self.pitch).sin(),
            Rad(self.yaw).sin() * Rad(self.pitch).cos()
        );

        self.front = direction.normalize();
    }

    pub fn calculate_view(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }
}

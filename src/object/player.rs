use crate::object;
use crate::ds;

pub struct Player {
    camera: object::Camera,
    rotation: ds::Vector3
}

impl Player {
    pub fn new(camera: object::Camera) -> Self {
        Self {
            camera: camera,
            rotation: ds::Vector3::zero()
        }
    }

    pub fn change_rotation(&mut self, delta: ds::Vector3) {
        self.rotation = self.rotation + delta;

        let pitch_rotation = ds::Vector3::new(
            0.0,
            self.rotation.pitch().sin(),
            self.rotation.pitch().cos()
        );

        let full_rotation = ds::Vector3::new(
            self.rotation.yaw().sin() * pitch_rotation.z,
            pitch_rotation.y,
            self.rotation.yaw().cos() * pitch_rotation.z
        );

        self.camera.set_dir_relative(full_rotation);
    }
    
    pub fn move_player(&mut self, delta: &ds::Vector3) {

        let sin = self.rotation.yaw().sin();
        let cos = self.rotation.yaw().cos();

        self.camera.move_camera(ds::Vector3::new(
            delta.z * sin + delta.x * cos,
            delta.y,
            delta.z * cos - delta.x * sin
        ));
    }

    pub fn get_rotation(&self) -> ds::Vector3 {
        self.rotation
    }

    pub fn get_camera(&self) -> &object::Camera {
        &self.camera
    }

    pub fn get_camera_mut(&mut self) -> &mut object::Camera {
        &mut self.camera
    }

    pub fn update_outputs(&mut self) {
        self.camera.update_outputs();
    }
}

unsafe impl Sync for Player {}
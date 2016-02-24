use std::sync::Arc;
use cgmath::{Point2, Vector2, Vector3, Matrix4, Quaternion, Rotation3, rad};
use glium::texture::Texture2dArray;

pub struct Sprite {
    pub size: Vector2<f32>,
    pub rotation: f32,
    
    pub animation_length: f64,
    pub animation_time: f64,
    
    pub texture: Arc<Texture2dArray>,
}

impl Sprite {
    pub fn animation_frame(&self) -> u32 {
        let frames = self.texture.array_size();
        (self.animation_time / self.animation_length * frames as f64) as u32
    }
    
    pub fn update(&mut self, dt: f64) {
        self.animation_time += dt;
        // I couldn't find an fmod function anywhere :(
        while self.animation_time >= self.animation_length {
            self.animation_time -= self.animation_length;
        }
    }
    
    pub fn matrix(&self, position: &Point2<f32>, cam_matrix: &Matrix4<f32>) -> Matrix4<f32> {
        let scale = Matrix4::from_nonuniform_scale(self.size.x, self.size.y, 1.0);
        let rotate: Matrix4<f32> = Quaternion::from_angle_z(rad(self.rotation)).into();
        let translate = Matrix4::from_translation(Vector3::new(position.x, position.y, 0.0));
        
        cam_matrix * translate * rotate * scale
    }
}

use cgmath::{Deg, Rad, Vector3};
use wgpu::RenderPass;
use crate::camera::Camera;
use crate::driver::Driver;
use crate::materials::{DiffuseMaterial, Material};
use crate::model::{DrawModel, Model};
use super::scene_node::SceneNode;
use crate::transform::{Transform, TransformSpace};

pub struct ModelNode {
    pub model: Model,
    pub transform: Transform,
    pub material: DiffuseMaterial,
}

impl SceneNode for ModelNode {
    fn update(&mut self, dt: f32) {
        self.transform.rotate_around_axis(
            Vector3::unit_z(),
            Rad::from(Deg(45.0 * dt)),
            TransformSpace::Local)
    }

    fn render<'a, 'b>(&'a mut self, driver: &'a Driver, camera: &'a Camera, pass: &mut RenderPass<'b>)
        where 'a: 'b
    {
        self.material.update(driver, camera, &self.transform);
        self.material.apply(pass);
        pass.draw_model(&self.model);
    }
}
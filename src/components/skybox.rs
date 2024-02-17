use bevy_ecs::prelude::*;

use crate::assets::Assets;
use crate::components::transform::Transform;
use crate::components::{MeshRenderer, RenderOrder, ShaderVariant};
use crate::device::Device;
use crate::mesh::Mesh;
use crate::render_tags::RenderTags;
use crate::shaders::{SkyboxShader, SkyboxShaderParams};

#[derive(Component)]
pub struct Skybox;

impl Skybox {
    pub fn spawn(mut commands: Commands, device: Res<Device>, assets: Res<Assets>) {
        let shader = SkyboxShader::new(
            &device,
            SkyboxShaderParams {
                texture: &assets.skybox_tex,
                shader: &assets.skybox_shader,
            },
        );
        let mesh = Mesh::quad(&device);
        let renderer = MeshRenderer::new(mesh, ShaderVariant::Skybox(shader), RenderTags::SCENE);
        let transform = Transform::default();

        commands.spawn((Skybox, RenderOrder(-100), renderer, transform));
    }
}

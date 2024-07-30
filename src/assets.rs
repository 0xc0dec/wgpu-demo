use slotmap::{DefaultKey, SlotMap};

use crate::file;
use crate::graphics::Graphics;
use crate::materials::{
    ColorMaterial, Material, PostProcessMaterial, SkyboxMaterial, TexturedMaterial,
};
use crate::mesh::Mesh;
use crate::texture::Texture;

pub type MeshHandle = DefaultKey;
pub type MaterialHandle = DefaultKey;
pub type ShaderHandle = DefaultKey;
pub type TextureHandle = DefaultKey;

pub struct Assets {
    pub bricks_texture_handle: TextureHandle,
    pub crate_texture_handle: TextureHandle,
    pub skybox_texture_handle: TextureHandle,
    textures: SlotMap<TextureHandle, Texture>,

    pub color_shader_handle: ShaderHandle,
    pub textured_shader_handle: ShaderHandle,
    pub skybox_shader_handle: ShaderHandle,
    pub postprocess_shader_handle: ShaderHandle,
    shaders: SlotMap<ShaderHandle, wgpu::ShaderModule>,

    pub box_mesh_handle: MeshHandle,
    pub quad_mesh_handle: MeshHandle,
    meshes: SlotMap<MeshHandle, Mesh>,

    materials: SlotMap<MaterialHandle, Box<dyn Material>>,
}

impl Assets {
    pub fn load(gfx: &Graphics) -> Self {
        let (
            box_mesh,
            skybox_tex,
            bricks_tex,
            crate_tex,
            color_shader,
            textured_shader,
            postprocess_shader,
            skybox_shader,
        ) = pollster::block_on(async {
            (
                Mesh::from_file("cube.obj", gfx).await,
                Texture::new_cube_from_file("skybox_bgra.dds", gfx)
                    .await
                    .unwrap(),
                Texture::new_2d_from_file("bricks.png", gfx).await.unwrap(),
                Texture::new_2d_from_file("crate.png", gfx).await.unwrap(),
                new_shader_module(gfx, "color.wgsl").await,
                new_shader_module(gfx, "textured.wgsl").await,
                new_shader_module(gfx, "post-process.wgsl").await,
                new_shader_module(gfx, "skybox.wgsl").await,
            )
        });

        let mut meshes = SlotMap::new();
        let box_mesh_handle = meshes.insert(box_mesh);
        let quad_mesh_handle = meshes.insert(Mesh::quad(gfx));

        let mut shaders = SlotMap::new();
        let color_shader_handle = shaders.insert(color_shader);
        let textured_shader_handle = shaders.insert(textured_shader);
        let postprocess_shader_handle = shaders.insert(postprocess_shader);
        let skybox_shader_handle = shaders.insert(skybox_shader);

        let mut textures = SlotMap::new();
        let bricks_texture_handle = textures.insert(bricks_tex);
        let skybox_texture_handle = textures.insert(skybox_tex);
        let crate_texture_handle = textures.insert(crate_tex);

        Self {
            textures,
            bricks_texture_handle,
            crate_texture_handle,
            skybox_texture_handle,
            shaders,
            color_shader_handle,
            textured_shader_handle,
            postprocess_shader_handle,
            skybox_shader_handle,
            meshes,
            box_mesh_handle,
            quad_mesh_handle,
            materials: SlotMap::new(),
        }
    }

    pub fn mesh(&self, handle: MeshHandle) -> &Mesh {
        self.meshes.get(handle).unwrap()
    }

    pub fn shader(&self, handle: ShaderHandle) -> &wgpu::ShaderModule {
        self.shaders.get(handle).unwrap()
    }

    pub fn add_color_material(&mut self, gfx: &Graphics) -> MaterialHandle {
        self.materials
            .insert(Box::new(ColorMaterial::new(gfx, self)))
    }

    pub fn add_skybox_material(
        &mut self,
        gfx: &Graphics,
        texture: TextureHandle,
    ) -> MaterialHandle {
        self.materials.insert(Box::new(SkyboxMaterial::new(
            gfx,
            self,
            &self.textures[texture],
        )))
    }

    pub fn add_textured_material(
        &mut self,
        gfx: &Graphics,
        texture: TextureHandle,
    ) -> MaterialHandle {
        self.materials.insert(Box::new(TexturedMaterial::new(
            gfx,
            self,
            &self.textures[texture],
        )))
    }

    pub fn add_postprocess_material(
        &mut self,
        gfx: &Graphics,
        src_texture: &Texture,
    ) -> MaterialHandle {
        self.materials
            .insert(Box::new(PostProcessMaterial::new(gfx, self, src_texture)))
    }

    pub fn remove_material(&mut self, handle: MaterialHandle) {
        self.materials.remove(handle);
    }

    pub fn material(&self, handle: MaterialHandle) -> &dyn Material {
        self.materials[handle].as_ref()
    }

    pub fn material_mut(&mut self, handle: MaterialHandle) -> &mut dyn Material {
        self.materials[handle].as_mut()
    }
}

async fn new_shader_module(device: &wgpu::Device, src_file_path: &str) -> wgpu::ShaderModule {
    let src = file::read_string_asset(src_file_path).await.unwrap();
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

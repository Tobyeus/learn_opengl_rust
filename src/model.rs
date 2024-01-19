use std::ffi::c_void;
use std::ops::Add;
use std::path::Path;

use cgmath::{Vector3, Vector2};
use image::DynamicImage::{ImageRgb8, ImageRgba8};
use image::GenericImage;
use tobj::{self, Material};

use crate::mesh::{Mesh, Vertex, Texture};
use crate::shader::Shader;

#[derive(Default)]
pub struct Model {
    meshes: Vec<Mesh>,
    materials: Vec<tobj::Material>,
    directory: String,
}

const DIRECTORY: &str = "./resources/obj/";

impl Model {
    pub fn new(path: &str) -> Self {
        let mut model = Model::default();
        model.directory = path.to_string();
        model
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe { mesh.draw(shader); }
        }
    }

    fn load_model(&mut self, file_name: &str) {
        let file_path = DIRECTORY.to_string().add(file_name);
        let object = tobj::load_obj(Path::new(file_path.as_str())).expect("Could not load 3D object");
        let models = object.0;
        self.materials = object.1;
        //self.processModels(models);

        for model in models.iter() {
            self.meshes.push(self.processMesh(&model.mesh));
            
        }
    }

    fn processMesh(&self, mesh: &tobj::Mesh) -> Mesh {
        let num_vertices = mesh.positions.len()/3;
        let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
        //verticies, normals, texcoords
        let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
        for i in 0..num_vertices {
            let position = Vector3::new(p[i*3], p[i*3 +1], p[i*3 +2]);
            let normals = Vector3::new(n[i*3], n[i*3 +1], n[i*3 +2]);
            let tex_coords = Vector2::new(t[i*2], t[i*2 +1]);

            let vertex = Vertex::new(
                position,
                normals,
                tex_coords
            );
            vertices.push(vertex);
        }

        //indices
        let indices = mesh.indices.clone();

        //materials
        let material = self.materials.get(mesh.material_id.unwrap()).unwrap();
        let mut textures = Vec::new();
        if !material.diffuse_texture.is_empty() {
            let diffuse_texture = Texture { id: load_texture(&material.diffuse_texture), tex_type: "texture_diffuse".into()};
            textures.push(diffuse_texture);
        }

        if !material.specular_texture.is_empty() {
            let specular_texture = Texture { id: load_texture(&material.specular_texture), tex_type: "texture_specular".into()};
            textures.push(specular_texture);
        }

        //not yet implemented in mesh
        if !material.normal_texture.is_empty() {
            let normal_texture = Texture { id: load_texture(&material.specular_texture), tex_type: "texture_normal".into()};
        }

        Mesh::new(vertices, indices, textures)
    }

}

fn load_texture(path: &str) -> u32 {
    let mut texture = 0;

    let img = image::open(path).expect("Could not open file").flipv();
    let format = match img {
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
        _ => panic!("Problem with the format")
    };
    let data = img.raw_pixels();

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        //wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        //filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    };

    texture
}
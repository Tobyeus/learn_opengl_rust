use std::ffi::c_void;
use std::ops::Add;
use std::path::Path;

use cgmath::{Vector3, Vector2};
use gl::CopyBufferSubData;
use image::DynamicImage::{ImageRgb8, ImageRgba8};
use image::GenericImage;
use tobj::{self, Material};

use crate::mesh::{Mesh, Vertex, Texture};
use crate::shader::Shader;

pub struct Model {
    meshes: Vec<Mesh>,
    loaded_textures: Vec<Texture>,
    directory: String,
}

const DIRECTORY: &str = "./resources/obj/";

impl Model {
    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe { mesh.draw(shader); }
        }
    }

    pub fn load_model(folder: &str, file_name: &str) -> Model {
        let directory = String::from(DIRECTORY).add(folder);
        let full_path = String::from(&directory).add("/").add(file_name);
        println!("{:?}", full_path);
        let object = tobj::load_obj(Path::new(full_path.as_str())).expect("Could not load 3D object");
        let models = object.0;
        let materials = object.1;

        let mut meshes = Vec::new();
        let mut model_result = Model { meshes: Vec::new(), directory, loaded_textures: Vec::new() };
        
        let mut index = 0;
        for model in models.iter() {
            let material_id = model.mesh.material_id.unwrap();
            let mesh = model_result.processMesh(&model.mesh, materials.get(material_id).unwrap()); 
            meshes.push(mesh);
            index = index + 1;
        }
        model_result.meshes = meshes;
        model_result
    }

    fn processMesh(&mut self, mesh: &tobj::Mesh, material: &tobj::Material) -> Mesh {
        let num_vertices = mesh.positions.len()/3;
        let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
        //verticies, normals, texcoords
        let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
        for i in 0..num_vertices {
            let position = Vector3::new(p[i*3], p[i*3 +1], p[i*3 +2]);
            let normals = Vector3::new(n[i*3], n[i*3 +1], n[i*3 +2]);
            let tex_coords = Vector2::new(t[i*2], t[i*2 +1]);
    
            let vertex = Vertex {
                position,
                normals,
                tex_coords
            };
            vertices.push(vertex);
        }
    
        //indices
        let indices = mesh.indices.clone();
        
        //materials
        let mut textures = Vec::new();

        let mut skip = false;
        println!("Amount of loaded textures: {}", self.loaded_textures.len());
        for t in self.loaded_textures.iter() {
            println!("loaded textures: {:?}", t);
            if 
                t.path == material.diffuse_texture 
                //|| t == &material.diffuse_texture 
                {
                skip = true;
                println!("skipping texture: {}", mesh.material_id.unwrap());
                textures.push(t.clone());
            }
        }

        println!("Skipping? {}", skip);
        println!("material diffuse: {}", material.diffuse_texture);

        if !skip && !material.diffuse_texture.is_empty() {
            let diffuse_texture = Texture { 
                id: self.load_texture(String::from(&self.directory).add("/").add(&material.diffuse_texture).as_str()), 
                tex_type: "texture_diffuse".into(),
                path: material.diffuse_texture.clone()
            };
            self.loaded_textures.push(diffuse_texture.clone());
            textures.push(diffuse_texture);
        }
    
        // if !skip && !material.specular_texture.is_empty() {
        //     let specular_texture = Texture { 
        //         id: self.load_texture(String::from(&self.directory).add("/").add(&material.specular_texture).as_str()),
        //         tex_type: "textureSpecular".into(),
        //         path: material.diffuse_texture.clone()
        //     };
        //     self.loaded_textures.push(material.diffuse_texture.clone());
        //     textures.push(specular_texture);
        // }
    
        //not yet implemented in mesh
        // if !material.normal_texture.is_empty() {
        //     let normal_texture = Texture { 
        //         id: load_texture(String::from(&self.directory).add("/").add(&material.normal_texture).as_str()),
        //         tex_type: "texture_normal".into()
            
        //     };
        //}
    
        Mesh::new(vertices, indices, textures)
    }

    fn load_texture(&self, path: &str) -> u32 {
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
}
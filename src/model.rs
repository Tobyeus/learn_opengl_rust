#![allow(non_snake_case)]
#![allow(dead_code)]

use std::default;
use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use tobj;

use mesh::{ Mesh, Texture, Vertex };
use shader::Shader;
use tobj::Material;

use crate::mesh;
use crate::shader;

pub struct Model {
    /*  Model Data */
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>,   // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    directory: String,
}

impl Model {
    pub fn new(path: &str, file: &str) -> Model {
        let mut model = Model {
            directory: path.into(),
            ..Model::default()
        };
        model.load_model(file);
        model
    }
    
    pub fn Draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe { mesh.Draw(shader); }
        }
    }
    
    pub fn load_model(&mut self, file: &str) {
        let full_dir = &format!("{}/{}", self.directory, file);
        let path = Path::new(full_dir);
        let obj = tobj::load_obj(path).expect("Could not open object file");
        
        let (models, materials) = obj;
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;
            
            // data to fill
            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            //extract indices
            let indices: Vec<u32> = mesh.indices.clone();
            
            //extracting positions, normals, texture coordinates
            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position:  vec3(p[i*3], p[i*3+1], p[i*3+2]),
                    normal:    vec3(n[i*3], n[i*3+1], n[i*3+2]),
                    tex_coords: vec2(t[i*2], t[i*2+1]),
                    ..Vertex::default()
                })
            }
            
            let textures = match mesh.material_id {
                Some(material_id) => self.process_materials(materials.get(material_id).expect("Could not load material from mesh")),
                None => Vec::new()
            };  
            
            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }
    
    fn process_materials(&mut self, material: &Material) -> Vec<Texture> {
        let mut textures = Vec::new();
        // 1. diffuse map
        if !material.diffuse_texture.is_empty() {
            let texture = self.load_material_texture(&format!("{}/{}", self.directory, material.diffuse_texture), "texture_diffuse");
            textures.push(texture);
        }
        // 2. specular map
        if !material.specular_texture.is_empty() {
            let texture = self.load_material_texture(&format!("{}/{}", self.directory, material.diffuse_texture), "texture_specular");
            textures.push(texture);
        }
        // 3. normal map
        if !material.normal_texture.is_empty() {
            let texture = self.load_material_texture(&format!("{}/{}", self.directory, material.diffuse_texture), "texture_normal");
            textures.push(texture);
        }
        textures
    }
    
    fn load_material_texture(&mut self, path: &str, typeName: &str) -> Texture {
        let texture = self.textures_loaded.iter().find(|t| t.path == path);
        if let Some(texture) = texture {
            return texture.clone();
        }
        let texture = Texture {
            id: unsafe { TextureFromFile(path) },
            type_: typeName.into(),
            path: path.into()
        };
        self.textures_loaded.push(texture.clone());
        texture
    }
}

unsafe fn TextureFromFile(path: &str) -> u32 {
    let mut textureID = 0;
    gl::GenTextures(1, &mut textureID);
    
    let img = image::open(&Path::new(&path)).expect("Texture failed to load");
    //image might need to flip
    //let img = img.flipv();
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
    };
    
    let data = img.raw_pixels();
    
    gl::BindTexture(gl::TEXTURE_2D, textureID);
    gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
    0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);
    
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    
    textureID
}
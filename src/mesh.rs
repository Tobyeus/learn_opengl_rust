#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CString;
use std::mem::{self, size_of};
use std::os::raw::c_void;
use std::ptr;

use cgmath::{ Vector3, Vector2 };
use cgmath::prelude::*;
use gl;

use shader::Shader;
use crate::shader;

pub struct Vertex {
    // position
    pub position: Vector3<f32>,
    // normal
    pub normal: Vector3<f32>,
    // texCoords
    pub tex_coords: Vector2<f32>,
    // tangent
    pub tangent: Vector3<f32>,
    // bitangent
    pub bitangent: Vector3<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
            tangent: Vector3::zero(),
            bitangent: Vector3::zero(),
        }
    }
}

#[derive(Clone,Debug)]
pub struct Texture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

pub struct Mesh {
    /*  Mesh Data  */
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub VAO: u32,

    /*  Render data  */
    VBO: u32,
    EBO: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let mut mesh = Mesh {
            vertices, indices, textures,
            VAO: 0, VBO: 0, EBO: 0
        };

        // now that we have all the required data, set the vertex buffers and its attribute pointers.
        unsafe { mesh.setupMesh() }
        mesh
    }

    /// render the mesh
    pub unsafe fn Draw(&self, shader: &Shader) {
        // bind appropriate textures
        let mut diffuseNr  = 0;
        let mut specularNr = 0;
        let mut normalNr = 0;
        let mut heightNr = 0;
        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32); // active proper texture unit before binding
            // retrieve texture number (the N in diffuse_textureN)
            let name = &texture.type_;
            let number = match name.as_str() {
                "texture_diffuse" => {
                    diffuseNr += 1;
                    diffuseNr
                },
                "texture_specular" => {
                    specularNr += 1;
                    specularNr
                }
                "texture_normal" => {
                    normalNr += 1;
                    normalNr
                }
                "texture_height" => {
                    heightNr += 1;
                    heightNr
                }
                _ => panic!("unknown texture type")
            };
            // now set the sampler to the correct texture unit
            let sampler = CString::new(format!("{}{}", name, number)).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(shader.program, sampler.as_ptr()), i as i32);
            // and finally bind the texture
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }

        // draw mesh
        gl::BindVertexArray(self.VAO);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);

        // always good practice to set everything back to defaults once configured.
        gl::ActiveTexture(gl::TEXTURE0);
    }

    unsafe fn setupMesh(&mut self) {
        // create buffers/arrays
        gl::GenVertexArrays(1, &mut self.VAO);
        gl::GenBuffers(1, &mut self.VBO);
        gl::GenBuffers(1, &mut self.EBO);

        gl::BindVertexArray(self.VAO);
        // load data into vertex buffers
        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
        // A great thing about structs with repr(C) is that their memory layout is sequential for all its items.
        // The effect is that we can simply pass a pointer to the struct and it translates perfectly to a glm::vec3/2 array which
        // again translates to 3/2 floats which translates to a byte array.
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // set the vertex attribute pointers
        let size = size_of::<Vertex>() as i32;
        // vertex Positions
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, ptr::null());
        // vertex normals
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, (3 * mem::size_of::<f32>()) as *const c_void);
        // vertex texture coords
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size, (6 * mem::size_of::<f32>()) as *const c_void);
        // // vertex tangent
        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, size, (8 * mem::size_of::<f32>()) as *const c_void);
        // // vertex bitangent
        gl::EnableVertexAttribArray(4);
        gl::VertexAttribPointer(4, 3, gl::FLOAT, gl::FALSE, size, (11 * mem::size_of::<f32>()) as *const c_void);

        gl::BindVertexArray(0);
    }

    pub unsafe fn draw(&self, shader: &Shader) {

        //iterate through textures
        //set uniforms for textures
        //bind textures
        //draw call
        let mut diffuse_index = 0;

        for (i, t) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            let name = &t.tex_type;
            let number = match name.as_str() {
                "texture_diffuse" => {
                    diffuse_index += 1;
                    diffuse_index
                },
                _ => panic!("No implemented texture type..")
            };
            let uniform_name = format!("{}{}", name, number.to_string());
            shader.use_program();
            shader.set_int(&uniform_name, i as i32);
            gl::BindTexture(gl::TEXTURE_2D, t.id);
        }
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());

        gl::BindVertexArray(0);
        gl::ActiveTexture(gl::TEXTURE0);
    }
}

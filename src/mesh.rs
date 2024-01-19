use std::ffi::c_void;
use std::mem;
use std::ptr;

use cgmath::Vector3;
use cgmath::Vector2;
use gl::types::GLfloat;
use gl::types::GLsizei;
use gl::types::GLsizeiptr;

use crate::shader::Shader;

pub struct Vertex {
    position: Vector3<f32>,
    normals: Vector3<f32>,
    tex_coords: Vector2<f32>
}

impl Vertex {
    pub fn new(position: Vector3<f32>, normals: Vector3<f32>, tex_coords: Vector2<f32>) -> Self {
        Vertex {
            position,
            normals,
            tex_coords
        }
    }
}

#[derive(Clone)]
pub struct Texture {
    pub id: u32,
    pub tex_type: String
}

pub struct Mesh {
    //mesh data
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: u32,
    //render data
    vbo: u32,
    ebo: u32
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        
        let vbo = 0;
        let vao = 0;
        let ebo = 0;
        
        Mesh {
            vertices,
            indices,
            textures,
            vbo,
            vao,
            ebo
        }
    }
    
    pub unsafe fn draw(&self, shader: &Shader) {
        let diffuse_number = 0;
        let specular_number = 0;

        for (index, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + texture.id as u32);
            let number: String = match texture.tex_type.as_str() {
                "texture_diffuse" => (diffuse_number + 1).to_string(),
                "texture_specular" => (specular_number + 1).to_string(),
                _ => panic!("Could not handle type of texture")
            };
            let material_name = String::from("material.") + &texture.tex_type + &number;
            shader.set_int(material_name.as_str(), index as i32);
            gl::BindTexture(gl::TEXTURE_2D, texture.id as u32);
        }

        //draw mesh
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        //clean up
        gl::BindVertexArray(0);
        gl::ActiveTexture(gl::TEXTURE0);
    }
    
    unsafe fn setupMesh(&mut self){
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);
        
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &self.vertices[0] as *const Vertex as *const c_void,
            gl::STATIC_DRAW
        );
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (self.indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            self.indices[0] as *const u8 as *const c_void,
            gl::STATIC_DRAW
        );

        // vertex positions
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as GLsizei, ptr::null());
        // vertex normals
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as GLsizei, ptr::null());
        // vertex tex coords
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (2 * mem::size_of::<GLfloat>()) as GLsizei, ptr::null());
        
        gl::BindVertexArray(0);
    }
}
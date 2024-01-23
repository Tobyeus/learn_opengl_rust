use std::ffi::c_void;
use std::mem;
use std::ptr;

use cgmath::Vector3;
use cgmath::Vector2;
use cgmath::Zero;

use crate::shader::Shader;

#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normals: Vector3<f32>,
    pub tex_coords: Vector2<f32>
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normals: Vector3::zero(),
            tex_coords: Vector2::zero(),
        }
    }
}

#[derive(Clone,Debug)]
pub struct Texture {
    pub id: u32,
    pub tex_type: String,
    pub path: String
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
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            vao: 0, vbo: 0, ebo: 0
        };

        unsafe { mesh.setup_mesh(); }
        mesh
    }

    unsafe fn setup_mesh(&mut self) {
        // EBO VBO VAO
        // setups
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (self.vertices.len() * mem::size_of::<Vertex>()) as isize,
            &self.vertices[0] as *const Vertex as *const c_void,
            gl::STATIC_DRAW
        );
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (self.indices.len() * mem::size_of::<u32>()) as isize,
            &self.indices[0] as *const u32 as *const c_void,
            gl::STATIC_DRAW
        );

        let stride = mem::size_of::<Vertex>() as i32;

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<f32>()) as *const c_void);

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<f32>()) as *const c_void);

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

use std::{fs::{self, File}, ffi::CString, ptr};
use gl::types::{GLint, GLchar};

pub struct Shader {
    pub program: u32
}

impl Shader {
    pub fn new(path_vs: &str, path_fs: &str) -> Self {

        let mut shader = Shader { program: 0 };

        let vertex_shader = match File::open(path_vs) {
            Ok(_file) => match fs::read_to_string(path_vs) {
                Ok(source) => source,
                Err(e) => panic!("Could not read the vertex shader. {}", e)
            },
            Err(e) => panic!("Could not open the vertex shader. {}", e)
        };

        let fragment_shader = match File::open(path_fs) {
            Ok(_file) => match fs::read_to_string(path_fs) {
                Ok(source) => source,
                Err(e) => panic!("Could not read the fragment shader. {}", e)
            },
            Err(e) => panic!("Could not open the fragment shader. {}", e)
        };

        // use as CString
        let c_str_vert = CString::new(vertex_shader.as_bytes()).unwrap();
        let c_str_frag = CString::new(fragment_shader.as_bytes()).unwrap();

        // generate shader program
        unsafe {
            // infoLog setup
            let mut success = 0;
            let mut infoLog = Vec::with_capacity(512);
            infoLog.set_len(512 - 1);

            // vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            // check vertex shader
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            }

            // fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            // check fragment shader
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
                if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            }

            // combine shaders with program
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            // clean up
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader.program = shader_program;
        };

        shader
    }
    // use/activate
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.program); }
    }

    // utility uniform functions
    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr()), value as i32); }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr()), value); }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe { gl::Uniform1f(gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr()), value); }
    }

    pub fn set_vector3(&self, name: &str, vector: Vec<f32>) {
        unsafe { gl::Uniform3f(gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr()), vector[0], vector[1], vector[2]) }
    }

}
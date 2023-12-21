extern crate glfw;
extern crate gl;

use std::{ffi::{CString, c_void}, ptr, mem};

use glfw::{Action, Context, Key, PWindow, GlfwReceiver};

use gl::types::*;

// Constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT:u32 = 600;

const vertexShaderSource: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 aPos;

    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 330 core

    out vec4 FragColor;

    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn main() {

    let mut glfw = initialize_glfw();

    let (mut window, events) = create_window(&mut glfw);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (program, VAO) = unsafe {

        // vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
        // arguments: shader object, number of strings, source of the shader
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        // fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(fragmentShaderSource.as_bytes()).unwrap();
        // arguments: shader object, number of strings, source of the shader
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        // shader program
        let shader_program = gl::CreateProgram();
        // link shaders
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // clean up, delete shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let vertices: [f32; 12] = [
            // x    y    z
            -0.5, -0.5, 0.0,
             0.5, -0.5, 0.0,
            -0.5,  0.5, 0.0,
             0.5,  0.5, 0.0
        ];

        let indices = [
            0, 1, 2,
            1, 2, 3
        ];

        // initialize vbo and vao
        // VBO - vertex buffer object
        // this buffer holds the data(vertices), and will be copied to the graphics card
        // VAO - vertex array object
        // object, holds how the data should be used
        let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);

        // set up opengl objects
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);

        // bind VAO first
        gl::BindVertexArray(VAO);

        // bind buffer to opengl state and fill with data
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);

        // bind buffer to opengl state and fill with data
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &indices[0] as *const i32 as *const c_void, gl::STATIC_DRAW);

        //configurate VAO
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());

        // we use only 1 VAO so the index of the location is 0
        gl::EnableVertexAttribArray(0);
        
        // clean up? I think this is not mandatory
        // unbind VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // unbind VAO
        gl::BindVertexArray(0);

        (shader_program, VAO)
    };

    while !window.should_close() {
        // render stuff here
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
    
            gl::UseProgram(program);
            gl::BindVertexArray(VAO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    
        // Swap front and back buffers
        window.swap_buffers();

        // processing events here
        process_events(&mut glfw, &mut window, &events);
    }
}

fn render(window: &mut PWindow, VAO: u32) {

    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindVertexArray(VAO);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }

    // Swap front and back buffers
    window.swap_buffers();
}

fn process_input(window: &mut PWindow, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        _ => {},
    }
}

fn process_events(glfw: &mut glfw::Glfw, window: &mut PWindow, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    // Poll for and process events
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(events) {
        println!("{:?}", event);
        process_input(window, event);
    }
}

fn initialize_glfw() -> glfw::Glfw {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(glfw::fail_on_errors!()).expect("Failed to initialize GLFW");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw
}

fn create_window(glfw: &mut glfw::Glfw) -> (glfw::PWindow, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Learning OpenGL the Rust Way...", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    (window, events)
}
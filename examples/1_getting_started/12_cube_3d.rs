extern crate glfw;
extern crate gl;

use std::{ffi::{c_void, CString}, ptr, mem, path::Path};
use glfw::{Action, Context, Key, PWindow, GlfwReceiver};
use gl::types::*;
use learn_opengl_rust::shader::Shader;
use image::{DynamicImage, GenericImage};
use cgmath::{Matrix4, Vector4, BaseFloat, SquareMatrix, Vector3, Matrix, Rad, perspective, Deg};

// Constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT:u32 = 600;


fn main() {

    let mut glfw = initialize_glfw();

    let (mut window, events) = create_window(&mut glfw);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shaders = Shader::new(
        "./src/shaders/1_getting_started/coord_systems.vs", 
        "./src/shaders/1_getting_started/multiple_tex.fs");

    let (VAO, texture1, texture2) = unsafe {

        // Vertices for a 3d cube
        let vertices_3D: [f32; 180] = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
             0.5, -0.5, -0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,
        
            -0.5, -0.5,  0.5,  0.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
        
            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,
        
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5,  0.5,  0.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
        
            -0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  1.0, 1.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
        
            -0.5,  0.5, -0.5,  0.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ];

        let (mut VBO, mut VAO, mut EBO) = (0,0,0);

        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        //gl::GenBuffers(1, &mut EBO);

        gl::BindVertexArray(VAO);

        // bind buffer will set the buffer as the current OpenGl State
        // then storing the data inside this buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (vertices_3D.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, 
            &vertices_3D[0] as *const f32 as *const c_void, 
            gl::STATIC_DRAW
        );

        //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        //gl::BufferData(
        //    gl::ELEMENT_ARRAY_BUFFER,
        //    (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        //    &indices[0] as *const i32 as *const c_void, 
        //   gl::STATIC_DRAW
        //);

        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

        // configure position attribute aPos
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        // initialize textures as u32
        let (mut texture1, mut texture2): (u32, u32) = (0, 0);
        // generate textures in ogl
        gl::GenTextures(1, &mut texture1);
        // bind texture
        gl::BindTexture(gl::TEXTURE_2D, texture1);

        // settings for texture wrapping 
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // settuings for texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // more info https://learnopengl.com/Getting-started/Textures

        // store image, flip vertically
        let texture_image = image::open(&Path::new("resources/textures/container.jpg")).expect("Could not open the file").flipv();
        // set data as raw_pixels
        let data = texture_image.raw_pixels();

        // configure texture
        gl::TexImage2D(gl::TEXTURE_2D, 
            0, 
            gl::RGB as i32, 
            texture_image.width() as i32, 
            texture_image.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void
        );
        // generate Mipmap
        gl::GenerateMipmap(gl::TEXTURE_2D);

        let texture_image = image::open("resources/textures/awesomeface.png").expect("Could not open file").flipv();
        let data = texture_image.raw_pixels();

        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(gl::TEXTURE_2D, 
            0, 
            gl::RGBA as i32, 
            texture_image.width() as i32, 
            texture_image.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        
        // clean up? I think this is not mandatory
        // unbind VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // unbind VAO
        gl::BindVertexArray(0);

        // make sure to use the program befor setting uniforms
        shaders.use_program();

        // set uniforms for textures
        shaders.set_int("texture1", 0);
        shaders.set_int("texture2", 1);

        (VAO, texture1, texture2)
    };

    let model = Matrix4::<f32>::from_angle_x(Deg(-55.0));
    let view = Matrix4::<f32>::from_translation(Vector3::new(0.0, 0.0, -3.0));
    let projection = perspective(Deg(45.0), WINDOW_WIDTH as f32/WINDOW_HEIGHT as f32, 0.1, 100.0);

    // checking matrices
    println!("Model Matrix: {:?}", model);
    println!("View Matrix: {:?}", view);
    println!("Projection Matrix: {:?}", projection);

    // setting up uniforms
    unsafe {
        shaders.use_program();

        let model_mat_location = gl::GetUniformLocation(shaders.program, CString::new("model").unwrap().as_ptr());
        gl::UniformMatrix4fv(model_mat_location, 1, gl::FALSE, model.as_ptr());

        let view_mat_location = gl::GetUniformLocation(shaders.program, CString::new("view").unwrap().as_ptr());
        gl::UniformMatrix4fv(view_mat_location, 1, gl::FALSE, view.as_ptr());

        let proj_mat_location = gl::GetUniformLocation(shaders.program, CString::new("projection").unwrap().as_ptr());
        gl::UniformMatrix4fv(proj_mat_location, 1, gl::FALSE, projection.as_ptr());

        gl::Enable(gl::DEPTH_TEST);
    }

    while !window.should_close() {
        // render stuff here
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Let the cube rotate
            // recalculate model
            // make sure to update the uniform too
            let model = model * Matrix4::<f32>::from_angle_z(Rad(glfw.get_time() as f32));
            let model_mat_location = gl::GetUniformLocation(shaders.program, CString::new("model").unwrap().as_ptr());
            gl::UniformMatrix4fv(model_mat_location, 1, gl::FALSE, model.as_ptr());

            // set active texture group and bind the texture
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            shaders.use_program();
            gl::BindVertexArray(VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            //gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    
        // Swap front and back buffers
        window.swap_buffers();

        // processing events here
        process_events(&mut glfw, &mut window, &events);
    }
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
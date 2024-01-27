extern crate glfw;
extern crate gl;

use std::{ffi::c_void, mem, ptr};

use glfw::{Action, Context, Key, GlfwReceiver};
//use gl::types::*;
use learn_opengl_rust::{shader::Shader, model::Model};
use cgmath::{perspective, Deg, EuclideanSpace, Matrix4, Point3, SquareMatrix, Vector2, Vector3};
use learn_opengl_rust::camera::{Camera, CameraMovement};

// Constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    // init
    let mut glfw = initialize_glfw();
    let (mut window, events) = create_window(&mut glfw);

    // init camera
    let mut camera = Camera::new(
        Point3::new(0.0, 0.5, 5.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector2::new((WINDOW_WIDTH/2) as f32, (WINDOW_HEIGHT/2) as f32),
    );

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // enable depth perspective
    unsafe { gl::Enable(gl::DEPTH_TEST); };

    let model_shader = Shader::new(
        "./src/shaders/3_model_loading/model_lighting.vs", 
        "./src/shaders/3_model_loading/model_lighting.fs"
    );

    let basic_shader = Shader::new(
        "./src/shaders/2_lighting/basic_lighting.vs", 
        "./src/shaders/2_lighting/light_source.fs"
    );

    // Vertices for a 3d cube
    let vertices_light: [f32; 108] = [
        // positions      
        // vec3           
        -0.5, -0.5, -0.5, 
         0.5, -0.5, -0.5, 
         0.5,  0.5, -0.5, 
         0.5,  0.5, -0.5, 
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        -0.5, -0.5,  0.5,
         0.5, -0.5,  0.5,
         0.5,  0.5,  0.5,
         0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

         0.5,  0.5,  0.5,
         0.5,  0.5, -0.5, 
         0.5, -0.5, -0.5, 
         0.5, -0.5, -0.5,  
         0.5, -0.5,  0.5,  
         0.5,  0.5,  0.5,  

        -0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,  
         0.5, -0.5,  0.5,
         0.5, -0.5,  0.5,  
        -0.5, -0.5,  0.5,  
        -0.5, -0.5, -0.5,  

        -0.5,  0.5, -0.5,  
         0.5,  0.5, -0.5,  
         0.5,  0.5,  0.5,  
         0.5,  0.5,  0.5,  
        -0.5,  0.5,  0.5,  
        -0.5,  0.5, -0.5, 
    ];

    //
    let vao = unsafe {
        let (mut vbo, mut vao) = (0,0);

        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);
        
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_light.len() * mem::size_of::<f32>()) as isize,
            &vertices_light[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<f32>()) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);

        vao
    };

    let model = Model::new("./resources/obj/backpack", "backpack.obj");
    let projection = perspective(Deg(45.0), WINDOW_WIDTH as f32/WINDOW_HEIGHT as f32, 0.1, 100.0);
    
    //model
    model_shader.use_program();
    model_shader.set_mat4("projection", projection);
    model_shader.set_mat4("model", Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from_scale(0.5));
    
    //light uniform
    let ambient = Vector3::new(0.5, 0.5, 0.5);
    let diffuse = Vector3::new(0.8, 0.8, 0.8);
    let specular = Vector3::new(1.0, 1.0, 1.0);
    let constant = 1.0;
    let linear = 0.09;
    let quadratic = 0.032;

    model_shader.set_vector3v("light.ambient",ambient);
    model_shader.set_vector3v("light.diffuse", diffuse);
    model_shader.set_vector3v("light.specular", specular);
    model_shader.set_float("light.constant", constant);
    model_shader.set_float("light.linear", linear);
    model_shader.set_float("light.quadratic", quadratic);

    //cube
    let mut light_position = Vector3::new(2.0, 2.0, 2.0);
    basic_shader.use_program();
    basic_shader.set_mat4("projection", projection);

    //delta time
    let mut last_frame = 0.0;
    let mut delta_time;

    while !window.should_close() {

        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        let light_x = (2.0 * (glfw.get_time() * 2.0).cos()) as f32;
        let light_y = (glfw.get_time() * 1.0).cos() as f32;
        let light_z = (2.0 * (glfw.get_time() * 2.0).sin()) as f32;

        light_position.x = light_x;
        light_position.y = light_y;
        light_position.z = light_z;

        // processing events here
        process_events(&events, &mut camera);
        process_input_keyboard(&mut window, delta_time, &mut camera);

        // render stuff here
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            model_shader.use_program();
            //recalculate stuff
            let view = camera.calculate_view();
            model_shader.set_mat4("view", view);
            model_shader.set_vector3v("light.position", light_position);
            model_shader.set_vector3v("cameraPos", camera.position.to_vec());
            model.Draw(&model_shader);

            //stuff for lighting
            basic_shader.use_program();
            gl::BindVertexArray(vao);
            basic_shader.set_mat4("model", Matrix4::from_translation(light_position) * Matrix4::from_scale(0.1) );
            basic_shader.set_mat4("view", view);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    
        // Swap front and back buffers
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input_keyboard(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
    // quit application
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(Key::W) == Action::Press {
        camera.process_movement(CameraMovement::Forward, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_movement(CameraMovement::Backward, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_movement(CameraMovement::Left, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_movement(CameraMovement::Right, delta_time);
    }
}

fn process_events(events: &GlfwReceiver<(f64, glfw::WindowEvent)>, camera: &mut Camera) {
    for (_, event) in glfw::flush_messages(events) {
        println!("{:?}", event);
        match event {
            glfw::WindowEvent::CursorPos(x_pos, y_pos) => camera.process_cursor(x_pos as f32, y_pos as f32),
            _ => ()
        }
    }
}

fn initialize_glfw() -> glfw::Glfw {
    use glfw::fail_on_errors;
    let mut glfw: glfw::Glfw = glfw::init(glfw::fail_on_errors!()).expect("Failed to initialize GLFW");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw
}

fn create_window(glfw: &mut glfw::Glfw) -> (glfw::PWindow, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Learning OpenGL the rust way...", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    (window, events)
}
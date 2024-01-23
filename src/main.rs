extern crate glfw;
extern crate gl;

use std::{ffi::c_void, ptr, mem};
use glfw::{Action, Context, Key, GlfwReceiver};
use gl::types::*;
use learn_opengl_rust::{shader::Shader, model::{self, Model}};
use image::{GenericImage, DynamicImage::{ImageRgba8, ImageRgb8}};
use cgmath::{Matrix4, Vector3, perspective, Deg, Point3, Vector2, InnerSpace, Rad, Angle, EuclideanSpace, SquareMatrix};
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
        "./src/shaders/3_model_loading/model.vs", 
        "./src/shaders/3_model_loading/model.fs"
    );

    let model = Model::load_model("backpack", "backpack.obj");

    //camera matrices
    let projection = perspective(Deg(45.0), WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32, 0.1, 100.0);
    let model_mat = Matrix4::<f32>::identity();
    //uniforms
    model_shader.use_program();
    model_shader.set_mat4("projection", projection);
    model_shader.set_mat4("model", model_mat);

    //delta time
    let mut last_frame = 0.0;
    let mut delta_time;

    while !window.should_close() {

        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // processing events here
        process_events(&events, &mut camera);
        process_input_keyboard(&mut window, delta_time, &mut camera);

        // render stuff here
        unsafe {
            gl::ClearColor(0.1, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            model_shader.use_program();
            let view = camera.calculate_view();
            model_shader.set_mat4("view", view);    
            model.draw(&model_shader);
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
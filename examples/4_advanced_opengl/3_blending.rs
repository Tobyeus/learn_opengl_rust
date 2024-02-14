extern crate glfw;
extern crate gl;

use std::{ffi::c_void, mem, ptr};

use glfw::{Context, GlfwReceiver};
//use gl::types::*;
use learn_opengl_rust::{shader::Shader, utils};
use cgmath::{perspective, Deg, Matrix4, Point3, SquareMatrix, Vector2, Vector3};
use learn_opengl_rust::camera;

// Constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    // init
    let mut glfw = utils::initialize_glfw();
    let (mut window, events) = utils::create_window(&mut glfw, WINDOW_WIDTH, WINDOW_HEIGHT, "Chapter 4: Blending");

    // init camera
    let mut camera = camera::Camera::new(
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

    let depth_shader = Shader::new(
        "./src/shaders/4_advanced_opengl/depth_testing.vs", 
        "./src/shaders/4_advanced_opengl/depth_testing.fs"
    );

    // Vertices for a 3d cube
    let cube_vertices: [f32; 180] = [
        // positions       // texture Coords
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

    let plane_vertices: [f32; 30] = [
        // positions          // texture Coords (note we set these higher than 1 (together with GL_REPEAT as texture wrapping mode). this will cause the floor texture to repeat)
         5.0, -0.5,  5.0,  2.0, 0.0,
        -5.0, -0.5,  5.0,  0.0, 0.0,
        -5.0, -0.5, -5.0,  0.0, 2.0,

         5.0, -0.5,  5.0,  2.0, 0.0,
        -5.0, -0.5, -5.0,  0.0, 2.0,
         5.0, -0.5, -5.0,  2.0, 2.0
    ];

    let plane_vao = unsafe {
        let (mut vbo, mut vao) = (0,0);

        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);
        
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (plane_vertices.len() * mem::size_of::<f32>()) as isize,
            &plane_vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
        );

        let stride = (5 * mem::size_of::<f32>()) as i32;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        vao
    };

    let cube_vao = unsafe {
        let (mut vbo, mut vao) = (0,0);

        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);
        
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (cube_vertices.len() * mem::size_of::<f32>()) as isize,
            &cube_vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
        );

        let stride = (5 * mem::size_of::<f32>()) as i32;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        vao
    };

    let projection = perspective(Deg(45.0), WINDOW_WIDTH as f32/WINDOW_HEIGHT as f32, 0.1, 100.0);
    
    //plane object
    depth_shader.use_program();
    depth_shader.set_mat4("projection", projection);
    depth_shader.set_mat4("model", Matrix4::identity());

    // //cube object
    // cube_shader.use_program();
    // cube_shader.set_mat4("projection", projection);
    // cube_shader.set_mat4("model", Matrix4::identity());

    //delta time
    let mut last_frame = 0.0;
    let mut delta_time;

    while !window.should_close() {

        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // processing events here
        utils::process_events(&events, &mut camera);
        utils::process_input_keyboard(&mut window, delta_time, &mut camera);

        // render stuff here
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            //draw plane
            depth_shader.use_program();
            //recalculate stuff
            let view = camera.calculate_view();
            depth_shader.set_mat4("view", view);
            depth_shader.set_mat4("model", Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)));

            gl::BindVertexArray(plane_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            depth_shader.set_mat4("model", Matrix4::from_translation(Vector3::new(1.0, 0.0, 2.0)));

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    
        // Swap front and back buffers
        window.swap_buffers();
        glfw.poll_events();
    }
}
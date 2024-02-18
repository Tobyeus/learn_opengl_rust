extern crate glfw;
extern crate gl;

use std::{ffi::c_void, mem, ptr};

use glfw::Context;
//use gl::types::*;
use learn_opengl_rust::{model, shader::Shader, utils};
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

    //set shader files
    let cube_shader = Shader::new(
        "./src/shaders/4_advanced_opengl/blending.vs", 
        "./src/shaders/4_advanced_opengl/blending.fs"
    );

    //vertices for several objects
    let cube_vertices: [f32; 180] = [
        // Back face
        -0.5, -0.5, -0.5,  0.0, 0.0, // Bottom-left
         0.5,  0.5, -0.5,  1.0, 1.0, // top-right
         0.5, -0.5, -0.5,  1.0, 0.0, // bottom-right         
         0.5,  0.5, -0.5,  1.0, 1.0, // top-right
        -0.5, -0.5, -0.5,  0.0, 0.0, // bottom-left
        -0.5,  0.5, -0.5,  0.0, 1.0, // top-left
        // Front face
        -0.5, -0.5,  0.5,  0.0, 0.0, // bottom-left
         0.5, -0.5,  0.5,  1.0, 0.0, // bottom-right
         0.5,  0.5,  0.5,  1.0, 1.0, // top-right
         0.5,  0.5,  0.5,  1.0, 1.0, // top-right
        -0.5,  0.5,  0.5,  0.0, 1.0, // top-left
        -0.5, -0.5,  0.5,  0.0, 0.0, // bottom-left
        // Left face
        -0.5,  0.5,  0.5,  1.0, 0.0, // top-right
        -0.5,  0.5, -0.5,  1.0, 1.0, // top-left
        -0.5, -0.5, -0.5,  0.0, 1.0, // bottom-left
        -0.5, -0.5, -0.5,  0.0, 1.0, // bottom-left
        -0.5, -0.5,  0.5,  0.0, 0.0, // bottom-right
        -0.5,  0.5,  0.5,  1.0, 0.0, // top-right
        // Right face
         0.5,  0.5,  0.5,  1.0, 0.0, // top-left
         0.5, -0.5, -0.5,  0.0, 1.0, // bottom-right
         0.5,  0.5, -0.5,  1.0, 1.0, // top-right         
         0.5, -0.5, -0.5,  0.0, 1.0, // bottom-right
         0.5,  0.5,  0.5,  1.0, 0.0, // top-left
         0.5, -0.5,  0.5,  0.0, 0.0, // bottom-left     
        // Bottom face
        -0.5, -0.5, -0.5,  0.0, 1.0, // top-right
         0.5, -0.5, -0.5,  1.0, 1.0, // top-left
         0.5, -0.5,  0.5,  1.0, 0.0, // bottom-left
         0.5, -0.5,  0.5,  1.0, 0.0, // bottom-left
        -0.5, -0.5,  0.5,  0.0, 0.0, // bottom-right
        -0.5, -0.5, -0.5,  0.0, 1.0, // top-right
        // Top face
        -0.5,  0.5, -0.5,  0.0, 1.0, // top-left
         0.5,  0.5,  0.5,  1.0, 0.0, // bottom-right
         0.5,  0.5, -0.5,  1.0, 1.0, // top-right     
         0.5,  0.5,  0.5,  1.0, 0.0, // bottom-right
        -0.5,  0.5, -0.5,  0.0, 1.0, // top-left
        -0.5,  0.5,  0.5,  0.0, 0.0  // bottom-left        
    ];

    //set vao's
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
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<f32>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        vao
    };

    //load textures
    let cube_tex = unsafe { 
        let texture0 = model::texture_from_file("./resources/textures/marble.jpg"); 
        cube_shader.set_int("texture0", 0);
        texture0
    };

    //set shader model and projection
    let projection = perspective(Deg(45.0), WINDOW_WIDTH as f32/WINDOW_HEIGHT as f32, 0.1, 100.0);
    let model = Matrix4::identity();

    cube_shader.use_program();
    cube_shader.set_mat4("projection", projection);
    cube_shader.set_mat4("model", model);

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
            //reset context
            gl::ClearColor(0.2, 0.2, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 

            //enable cull_face
            gl::Enable(gl::CULL_FACE);
            //cull face function, to specify which face to cull
            gl::CullFace(gl::BACK); //gl::Back is default, so this is not needed
            //other options are gl::Front and gl::Front_And_Back

            //tells opengl which faces are supposed to be front
            //determined by CounterClockWise or ClockWise
            gl::FrontFace(gl::CCW); // CCW -> CounterClockWise
            //vertices are set up in a CCW -> front face way
            //more info in https://learnopengl.com/Advanced-OpenGL/Face-culling

            let view = camera.calculate_view();

            //draw cubes
            cube_shader.use_program();
            cube_shader.set_mat4("view", view);
            cube_shader.set_mat4("model", Matrix4::from_translation(Vector3::new(-1.0, 0.0, 2.0)));

            gl::ActiveTexture(0);
            gl::BindTexture(gl::TEXTURE_2D, cube_tex);

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            //will cull the "front" face of the cube
            gl::FrontFace(gl::CW);
            cube_shader.set_mat4("model", Matrix4::from_translation(Vector3::new(1.0, 0.0, 2.0)));

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

        }
    
        //Swap front and back buffers
        window.swap_buffers();
        glfw.poll_events();
    }
}
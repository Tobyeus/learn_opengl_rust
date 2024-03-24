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

    let plane_shader = Shader::new(
        "./src/shaders/4_advanced_opengl/blending.vs", 
        "./src/shaders/4_advanced_opengl/blending.fs"
    );

    let vegetation_shader = Shader::new(
        "./src/shaders/4_advanced_opengl/blending.vs", 
        "./src/shaders/4_advanced_opengl/blending_object.fs"
    );

    let window_shader = Shader::new(
        "./src/shaders/4_advanced_opengl/blending.vs", 
        "./src/shaders/4_advanced_opengl/blending.fs"
    );

    //vertices for several objects
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

    let transparent_vertices: [f32; 30] = [
        // positions         // texture Coords (swapped y coordinates because texture is flipped upside down)
        0.0,  0.5,  0.0,  0.0,  0.0,
        0.0, -0.5,  0.0,  0.0,  1.0,
        1.0, -0.5,  0.0,  1.0,  1.0,

        0.0,  0.5,  0.0,  0.0,  0.0,
        1.0, -0.5,  0.0,  1.0,  1.0,
        1.0,  0.5,  0.0,  1.0,  0.0
    ];

    //positions for objects
    let vegetation_positions: [Vector3<f32>; 3] = [
        Vector3::new(-1.5,  0.0, -0.48),
        Vector3::new(1.5,  0.0,  0.51),
        Vector3::new(0.0, 0.0, 0.7)
    ];
    
    let window_positions: [Vector3<f32>; 3] = [
        Vector3::new(-1.5, 0.0, -1.0),
        Vector3::new(-0.3, 0.0, -2.3),
        Vector3::new(0.5, 0.0, -0.6),
    ];

    //set vao's
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
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<f32>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

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
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<f32>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        vao
    };

    let vegetation_vao = unsafe {

        let (mut vao, mut vbo) = (0, 0);

        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (transparent_vertices.len() * mem::size_of::<f32>()) as isize,
            &transparent_vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
        );

        let stride = 5 * mem::size_of::<f32>() as i32;
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

    let plane_tex = unsafe {
        let texture1 = model::texture_from_file("./resources/textures/metal.png");
        plane_shader.set_int("texture0", 1);
        texture1
    };

    let vegetation_tex = unsafe {
        let texture2 = model::texture_from_file_transparent("./resources/textures/grass.png");
        vegetation_shader.set_int("texture0", 2);
        texture2
    };
    
    let window_tex = unsafe {
        let texture3 = model::texture_from_file("./resources/textures/blending_transparent_window.png");
        window_shader.set_int("texture0", 3);
        texture3
    };

    //set shader model and projection
    let projection = perspective(Deg(45.0), WINDOW_WIDTH as f32/WINDOW_HEIGHT as f32, 0.1, 100.0);
    let model = Matrix4::identity();

    cube_shader.use_program();
    cube_shader.set_mat4("projection", projection);
    cube_shader.set_mat4("model", model);

    plane_shader.use_program();
    plane_shader.set_mat4("projection", projection);
    plane_shader.set_mat4("model", model);

    vegetation_shader.use_program();
    vegetation_shader.set_mat4("projection", projection);
    vegetation_shader.set_mat4("model", model);

    window_shader.use_program();
    window_shader.set_mat4("projection", projection);
    window_shader.set_mat4("model", model);

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

            //draw plane
            plane_shader.use_program();
            let view = camera.calculate_view();
            plane_shader.set_mat4("view", view);
            plane_shader.set_mat4("model", model);

            gl::ActiveTexture(1);
            gl::BindTexture(gl::TEXTURE_2D, plane_tex);

            gl::BindVertexArray(plane_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            //draw cubes
            cube_shader.use_program();
            cube_shader.set_mat4("view", view);
            cube_shader.set_mat4("model", model);

            gl::ActiveTexture(0);
            gl::BindTexture(gl::TEXTURE_2D, cube_tex);

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            cube_shader.set_mat4("model", Matrix4::from_translation(Vector3::new(1.0, 0.0, 2.0)));

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            //draw vegetation
            vegetation_shader.use_program();
            vegetation_shader.set_mat4("view", view);

            gl::ActiveTexture(2);
            gl::BindTexture(gl::TEXTURE_2D, vegetation_tex);
            gl::BindVertexArray(vegetation_vao);

            for &v in vegetation_positions.iter() {
                vegetation_shader.set_mat4("model", Matrix4::from_translation(v));
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }

            //blending for window texture
            gl::Enable(gl::BLEND);
            //how blending works:
            //result = source color vector ∗ source factor value  +  destination color vector * destination factor value
            //color vectors set by opengl
            //factor values can be set by us

            //glBlendFunc(GLenum sfactor, GLenum dfactor) - source factor and destination factor
            //It is also possible to set different options for the RGB and alpha channel individually using glBlendFuncSeparate
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_CONSTANT_ALPHA);

            window_shader.use_program();
            window_shader.set_mat4("view", view);

            gl::ActiveTexture(3);
            gl::BindTexture(gl::TEXTURE_2D, window_tex);
            gl::BindVertexArray(vegetation_vao);

            for &v in window_positions.iter() {
                window_shader.set_mat4("model", Matrix4::from_translation(v));
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }

            //disable blending
            gl::Disable(gl::BLEND);
        }
    
        //Swap front and back buffers
        window.swap_buffers();
        glfw.poll_events();
    }
}
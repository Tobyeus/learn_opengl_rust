extern crate glfw;
extern crate gl;

use std::{ffi::c_void, mem, ptr};

use glfw::{Action, Context, Key, GlfwReceiver};
//use gl::types::*;
use learn_opengl_rust::{shader::Shader, model::Model};
use cgmath::{perspective, Deg, EuclideanSpace, Matrix4, Point3, SquareMatrix, Vector2, Vector3};
use learn_opengl_rust::camera::{Camera, CameraMovement};
use learn_opengl_rust::model::texture_from_file;

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
    unsafe { gl::Enable(gl::STENCIL_TEST); };

    let plane_shader = Shader::new(
        "./src/shaders/4_advanced_opengl/stencil_testing_plane.vs", 
        "./src/shaders/4_advanced_opengl/stencil_testing_plane.fs"
    );

    let cube_shader = Shader::new(
        "./src/shaders/4_advanced_opengl/stencil_testing_cube.vs", 
        "./src/shaders/4_advanced_opengl/stencil_testing_cube.fs" 
    );

    let border_shader = Shader::new(
        "./src/shaders/4_advanced_opengl/stencil_testing_cube.vs", 
        "./src/shaders/4_advanced_opengl/stencil_testing_border.fs" 
    );

    // let basic_shader = Shader::new(
    //     "./src/shaders/2_lighting/basic_lighting.vs", 
    //     "./src/shaders/2_lighting/light_source.fs"
    // );

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

    let projection = perspective(Deg(45.0), WINDOW_WIDTH as f32/WINDOW_HEIGHT as f32, 0.1, 100.0);
    
    //plane object
    plane_shader.use_program();
    plane_shader.set_mat4("projection", projection);
    plane_shader.set_mat4("model", Matrix4::identity());

    let plane_texture = unsafe {
        let texture = texture_from_file("./resources/metal.png");
        plane_shader.set_int("planeTex", 0);
        texture
    };

    //cube object
    cube_shader.use_program();
    cube_shader.set_mat4("projection", projection);
    cube_shader.set_mat4("model", Matrix4::identity());

    let cube_texture = unsafe {
        let texture = texture_from_file("./resources/marble.jpg");
        cube_shader.set_int("cubeTexture", 1);
        texture
    };

    border_shader.use_program();
    border_shader.set_mat4("projection", projection);
    border_shader.set_mat4("model", Matrix4::identity());

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
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            //set stencilmask to 0 -> 0s written to the buffer
            gl::StencilMask(0x00);
            
            //draw plane
            plane_shader.use_program();
            //recalculate stuff
            let view = camera.calculate_view();
            plane_shader.set_mat4("view", view);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, plane_texture);
            gl::BindVertexArray(plane_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            
            //draw first cube with texture

            // parameters for StencilOp
            // 1. sfail -> action when fails
            // 2. dpfail -> action when stencil passes and depth test fails
            // 3. dppass -> both stencil and depth pass
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            //mask to 255 -> pixel value -> 1
            gl::StencilMask(0xFF);

            cube_shader.use_program();
            cube_shader.set_mat4("view", view);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture);
            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            //second scaled up cube
            gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            gl::StencilMask(0x00);
            gl::Disable(gl::DEPTH_TEST);

            border_shader.use_program();
            border_shader.set_mat4("view", view);
            border_shader.set_mat4("model", Matrix4::from_scale(1.1));

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture);
            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::StencilMask(0xFF);
            gl::StencilFunc(gl::ALWAYS, 0, 0xFF);
            gl::Enable(gl::DEPTH_TEST);
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
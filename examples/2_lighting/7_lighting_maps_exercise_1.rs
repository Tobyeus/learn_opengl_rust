extern crate glfw;
extern crate gl;

use std::{ffi::c_void, ptr, mem};
use glfw::{Action, Context, Key, GlfwReceiver};
use gl::types::*;
use learn_opengl_rust::shader::Shader;
use image::{GenericImage, DynamicImage::{ImageRgba8, ImageRgb8}};
use cgmath::{Matrix4, Vector3, perspective, Deg, Point3, Vector2, SquareMatrix};
use learn_opengl_rust::camera::{Camera, CameraMovement};

// Constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

/*
    Also add something they call an emission map which is a texture that stores emission values per fragment. 
    Emission values are colors an object may emit as if it contains a light source itself; 
    this way an object can glow regardless of the light conditions. Emission maps are often what you see
    when objects in a game glow (like eyes of a robot, or light strips on a container).
*/

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

    let cube_shader = Shader::new(
        "./src/shaders/2_lighting/lighting_maps.vs", 
        "./src/shaders/2_lighting/emission_map.fs"
    );

    let light_source_shader = Shader::new(
        "./src/shaders/2_lighting/basic_lighting.vs", 
        "./src/shaders/2_lighting/light_source.fs"
    );

    // Vertices for a 3d cube
    let vertex_cube: [f32; 288] = [
        // positions       // normals        // texture coords
        // vec3            // vec3           // vec2
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
    ];

        let vao = unsafe {
            let (mut vao, mut vbo) = (0,0);

            gl::GenBuffers(1, &mut vbo);
            gl::GenVertexArrays(1, &mut vao);

            gl::BindVertexArray(vao);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_cube.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertex_cube[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );

            // stride for vertex data -> 3 vertex coords 3 normals 2 tex coords
            let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
            // vertex coords
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            // normals
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
            // texture coords
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2);

            vao
        };
    
    let mut model;
    let projection = perspective(Deg(45.0), WINDOW_WIDTH as f32/WINDOW_HEIGHT as f32, 0.1, 100.0);
    cube_shader.use_program();
    cube_shader.set_mat4("projection", projection);

    //light source setup
    let mut light_pos = Vector3::new(2.0, 1.5, 0.0);
    let light_color = Vector3::new(1.0, 1.0, 1.0);
    cube_shader.set_vector3("light.specular", 1.0, 1.0, 1.0);

    let mut last_frame = 0.0;
    let mut delta_time;

    //texture
    let diffuse_map = load_texture("./resources/textures/container2.png");
    let specular_map = load_texture("./resources/textures/container2_specular.png");
    let emission_map = load_texture("./resources/textures/matrix.jpg");
    
    //material
    cube_shader.set_int("material.diffuseTex", 0);
    cube_shader.set_int("material.specularTex", 1);
    cube_shader.set_int("material.emissionTex", 2);
    cube_shader.set_float("material.shininess", 64.0);

    while !window.should_close() {

        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        //light source position
        light_pos.x = glfw.get_time().cos() as f32 * 2.0;
        light_pos.y = glfw.get_time().cos() as f32 * 1.5;
        light_pos.z = glfw.get_time().sin() as f32 * 2.0;

        //changing light color
        //light_color.x = (glfw.get_time() * 2.0).sin() as f32;
        //light_color.y = (glfw.get_time() * 0.7).sin() as f32;
        //light_color.z = (glfw.get_time() * 1.3).sin() as f32;

        // processing events here
        process_events(&events, &mut camera);
        process_input_keyboard(&mut window, delta_time, &mut camera);

        // render stuff here
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            //bind diffuse map texture
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
            //bind specular map texture
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_map);
            //bind emissionTex
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, emission_map);

            model = Matrix4::<f32>::identity();
            cube_shader.use_program();
            cube_shader.set_mat4("view", camera.calculate_view());
            cube_shader.set_mat4("model", model);
            cube_shader.set_vector3("cameraPos", camera.position.x, camera.position.y, camera.position.z);
            cube_shader.set_vector3("light.position", light_pos.x, light_pos.y, light_pos.z);
            cube_shader.set_vector3("light.diffuse", light_color.x * 0.4, light_color.y * 0.4, light_color.z * 0.4);
            cube_shader.set_vector3("light.ambient", light_color.x * 0.5, light_color.y * 0.5, light_color.z * 0.5);
            
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            light_source_shader.use_program();
            model = Matrix4::<f32>::from_translation(light_pos) * Matrix4::<f32>::from_scale(0.2);
            light_source_shader.set_mat4("projection", projection);
            light_source_shader.set_mat4("view", camera.calculate_view());
            light_source_shader.set_mat4("model", model);

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
    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Learning OpenGL the Rust Way...", glfw::WindowMode::Windowed)
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

fn load_texture(path: &str) -> u32 {
    let mut texture = 0;

    let img = image::open(path).expect("Could not open file").flipv();
    let format = match img {
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
        _ => panic!("Problem with the format")
    };
    let data = img.raw_pixels();

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        //wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        //filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    };

    texture
}
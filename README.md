# Learn-OpenGL-Rust

This repository will show my progress learning OpenGL in Rust

I will be following the famous tutorial [LearnOpenGL](https://learnopengl.com) by Joey de Vries. Since I started learning Rust, I thought it might a great idea to follow the tutorial using Rust. As a starting and reference point, I will be using the [Rust port](https://github.com/bwasty/learn-opengl-rs) of LearnOpenGL by bwasty.

# Structure

As of now, I will mainly code in the main.rs file of the src folder. Finished chapters will be converted into examples, which can be run separately.

# How to use the examples

Make sure to use ```cargo build``` before running anything. The examples can be run via the command ```cargo run --example <example_name>```.

## List of the available examples:

1. Getting started:
- hello_triangle
- hello_indexed
- Exercise_1_1
- Exercise_1_2
- Exercise_1_3
- interpolation
- shaderclass
- textures
- multiple_textures
- transformations
- coordinate_system
- 3D_cube
- cubes
- camera_circle
- camera_move

2. Lighting

The List will grow gradually over time.

# Progress

## Chapter 1

I finished the first chapter. The chapter showed a few things I did not like about the structure of this repository. The example system by Cargo might not be the best option, since it bloats the Cargo.toml file. I might think about a different approach in the future.

## Chapter 2

Right now I am working on Chapter 2.
use std::process::Command;

fn main() {
    println!("Hey, you are running the Learn-OpenGL-Rust Project by Tobyeus.");
    println!("All examples can be run with the command 'cargo run --example [example_name]'.");
    println!("You can find all the example names in the 'Cargo.toml' file or use the command 'cargo run --exmaple' to find a list of example names.");
    println!("Which example would you like to run?");
    let mut example: String = String::new();
    std::io::stdin().read_line(&mut example).unwrap();
    println!("You entered: {}", example);
    println!("Loading the example...");
    runExample(example.trim());
    //output
    //println!("Status: {:?}", result.status);
    //println!("Error: {:?}", result.stderr);
    //println!("Out:: {:?}", result.stdout);
}

fn runExample(example_name: &str) {
    let mut example_command = Command::new("cargo");
    example_command.arg("run");
    example_command.arg("--example");
    example_command.arg(example_name);
    example_command.status();
}
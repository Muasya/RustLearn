
```fn``` keyword is used to define a function.
```
fn function_name() {
    Fuction body
}
```

```println!()``` is used to print a string to the console.


rustc is the compiler for the Rust programming language, provided by the project itself. Compilers take your source code and produce binary code, either as a library or executable. Most Rust programmers don't invoke rustc directly, but instead do it through Cargo.

To compile the code, run ```cargo build``` in the root directory of the project.
or ```rustc the_file.rs``` to compile a specific file.

When you run ```cargo init``` in the root directory of the project, it will create a Cargo.toml file.
All project files are located in the src/ directory. The Cargo.toml file contains the project's dependencies.

public functions are defined in the public module.
``` pub fn function_name() {
    Function body
}```

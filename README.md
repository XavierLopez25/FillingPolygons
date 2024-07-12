# FillingPolygons
Filling Polygons with Rust.

## Description
This project demonstrates how to draw and fill polygons using the Rust programming language. The project includes the implementation of a framebuffer, functions to draw lines and polygons, and an algorithm to fill polygons. Additionally, it supports creating polygons with holes.

## Features
- Draw and fill multiple polygons with different colors.
- Support for creating polygons with holes.
- Output the result as a BMP image.

## Getting Started

### Prerequisites
- Rust programming language installed. You can download and install Rust from [here](https://www.rust-lang.org/).

## Installation

#### Clone the repository:
  ```sh
    git clone git@github.com:XavierLopez25/FillingPolygons.git
  ```
#### Navigate to the project directory:
  ```sh
  cd FillingPolygons
  ```
#### Running the Program
To run the program and generate the output image:

```sh
cargo run
```

This will create an output.bmp file in the project directory containing the rendered polygons.

## Project Structure
```bash
FillingPolygons
├── src
│   ├── colors.rs
│   ├── framebuffer.rs
│   ├── bmp.rs
│   ├── line_impl.rs
│   ├── vertex.rs
│   └── main.rs
├── Cargo.toml
└── README.md
```

`colors.rs`
Defines the Color struct and related functions for handling colors.

`framebuffer.rs`
Defines the FrameBuffer struct and functions for managing the framebuffer, drawing points, and clearing the buffer.

`bmp.rs`
Contains functions to write the framebuffer to a BMP file.

`line_impl.rs`
Implements the Line trait for drawing lines and polygons.

`vertex.rs`
Defines the Vertex type using nalgebra_glm for 3D coordinates.

`main.rs`
The main entry point of the application, setting up the framebuffer, defining polygons, and rendering them.

## Usage
Modify the vertices and colors in main.rs to draw different polygons.

Add new polygons or holes by defining their vertices and calling the draw_polygon function.

## Example
Here is an example of how to define and draw a polygon:

```rust
let vertices = vec![
    glm::vec3(165.0, 380.0, 0.0),
    glm::vec3(185.0, 360.0, 0.0),
    glm::vec3(180.0, 330.0, 0.0),
    glm::vec3(207.0, 345.0, 0.0),
];

draw_polygon(&mut framebuffer, &vertices, 0xFFFFFF, 0xCCCC00);
```

## Acknowledgments
- Rust Programming Language
- nalgebra_glm crate for vector and matrix mathematics

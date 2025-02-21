
# Voxels wgpu


## Features

- **Window Initialization:** Creates a window titled "Voxels wgpu" with `winit`.
- **GPU Setup:** Initializes the WGPU instance, adapter, device, queue, and surface.
- **Shader Pipeline:** Loads vertex and fragment shaders from WGSL files to create a render pipeline.
- **Basic Rendering:** Draws a shape using a set of vertices with position and color attributes.

## Download and Build

To download and build the project, run the following commands:

```bash
git clone https://github.com/Lamoara/voxels_webgpu.git
cd voxels_webgpu
cargo build --release
```

## Run the Application

After building the project, execute:

bash

`cargo run --release` 

This will open a window titled "Voxels wgpu" and render the basic shape defined in the vertex buffer.
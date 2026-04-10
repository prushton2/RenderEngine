# 3D Render Engine

This is a custom render engine written in rust and WGSL.

## Arguments

| Flag | Long | Short | Default
|--|--|--|--|
| Resolution | --resolution | -r | 1280x720
| Mouse sensitivity | --sensitivity | -s | 0.001
| Max framerate | --framelimit | -f | 144
| Camera speed | --movespeed | -m | 1.5
| Field of view | --fov | -v | 60.0

## Features

### Shapes
The render engine provides quads and spheres for rendering.

### Materials
A Material will let you determine either a solid color or a source texture (only on quads), and also define how translucent and reflective the material is.

When using textures, you must include the paths of your textures in `GpuHandler::init`. When referencing the texture, the `texture_id` of any texture is the order they are listed in `GpuHandler::init`.

### GPU Rendering
The `wgpu_handler.rs` file provides heavy abstractions for handling rendering on the gpu.

## Running

In the releases section, there are windows and linux builds for the render engine. 

If you wish to build from source, install rust and type 

```
$ cargo run --release
```
<h6>Although release isnt necessary since most of the work is done on the gpu, i still recommend it.</h6>

### Movement

In the window, you can use WASD to move and your mouse or the arrow keys to look around. Space and LCtrl make you move up and down respectively.
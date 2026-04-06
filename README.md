# 3D Render Engine

This is a custom render engine written in rust and WGSL.

## Features

This render engine provides rendering spheres and quads. These come with a material that takes three parameters: color, reflectivity, and translucency. The material allows you to combine properties in one material. Spheres can be rendered from inside, so feel free to go inside the sphere on the right for a weird experience. Note that a ray can only be recast 4 times before it returns the color of the material it hits instead of casting further.

## Running

In the releases section, there are windows and linux builds for the render engine. 

If you wish to build from source, install rust and type 

```
$ cargo run --release
```
<h6>Although release isnt necessary since most of the work is done on the gpu, i still recommend it.</h6>

### Movement

In the window, you can use WASD to move and your mouse or the arrow keys to look around. Space and LCtrl make you move up and down respectively.

## Expansion

Expansion upon this isnt very well supported currently. I have some refactoring to do to make it more manageable. The main issue is once you start dealing with sending stuff to the gpu, where dynamic dispatching doesnt exist.

If you really want to add a new renderable object, you must implement Renderable and ToGpu for your object. See `src/objects/sphere.rs` for an example. Next, create a buffer in `main.rs` for the GPU version of your object to be sent to the GPU. You must create a field in App for the buffer, instantiate, and bind the buffer. In WGSL, add intersection in `intersection.wgsl`, and in `material.wgsl`, add a dereference for the material.
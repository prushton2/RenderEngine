# 3D Render Engine

This is a custom render engine written in rust and WGSL.

## Features

This render engine provides rendering spheres and quads. These come with a material that takes three parameters: color, reflectivity, and translucency. The material allows you to combine properties in one material. Spheres can be rendered from inside, so feel free to go inside the sphere on the right for a weird experience. Note that a ray can only be recast 4 times before it returns the color of the material it hits instead of casting further.

## Expansion

Expansion upon this isnt very well supported currently. I have some refactoring to do to make it more manageable. The main issue is once you start dealing with sending stuff to the gpu, where dynamic dispatching doesnt exist.

If you really want to add a new renderable object, you must implement Renderable and ToGpu for your object. See `src/objects/sphere.rs` for an example. Next, create a buffer in `main.rs` for the GPU version of your object to be sent to the GPU. You must create a field in App for the buffer, instantiate, and bind the buffer. In WGSL, add intersection in `intersection.wgsl`, and in `material.wgsl`, add a dereference for the material.
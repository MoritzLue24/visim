# visim ![crates.io](https://img.shields.io/crates/v/visim.svg)
A rust library based on opengl and sdl2 used to simulate and visualize algorithms and data structures. I made this for personal purpose so dont expect long time maintenance.A rust library to simulate and visualize algorithms and data structures. I made this for personal purpose so dont expect long time maintenance.

# Features / Todo
- [ ] Idk maybe remove unbinds?
- [X] Error handling
	- [X] Shader errors.
	- [X] Write a macro for errors to simplify the err.rs file.
	- [X] OpenGl runtime errors.
- [ ] Render Engine
	- [X] `Polygon` shape
	- [X] Custom `Color` type
	- [X] Custom `Vector` type
	- [X] `Window` struct for abstraction
	- [X] Customizable `Shader` `Program`s
	- [X] Render struct 
	- [X] Batch rendering
	- [ ] Macro for shader program uniforms
	- [ ] `Rect` shape 
	- [ ] `Circle` shape 
	- [ ] Outline thickness at shapes
	- [ ] Bitmap fonts
    - [ ] Textures
- [ ] Custom UI module
	- [ ] Surfaces holding T: impl Widgets
	- [ ] Label
	- [ ] Button
	- [ ] Slider
	- [ ] Input
	- [ ] Color picker
- [ ] Graphs / Plotting
	- [ ] Zoom in / out
- [ ] Other data visualization tools
	- [ ] Table

# Examples

To try an example, clone the repository and use `cargo run`. You have to enable
a graphics backend feature. For instance, if we want to run an example using
OpenGL, we run:

```
cargo run --example <example> --features opengl
```

__Coffee moves fast and the `master` branch can contain breaking changes!__ If
you want to learn about a specific release, check out [the release list].

[the release list]: https://github.com/hecrj/coffee/releases

## [Particles](particles.rs)

A particle gravity simulator that showcases a loading screen, input handling,
and graphics interpolation with batched drawing and font rendering. Move the
mouse around to attract the particles.

This example renders 50k independent particles every frame. Using the
`--release` flag to run the example is recommended. Additionally, you can
compile it with the `debug` feature if you want to enable the built-in debug
view:

```
cargo run --example particles --features vulkan,debug --release
```

[![Particles][particles]][particles_gfycat]

[particles]: https://github.com/hecrj/coffee/blob/master/images/examples/particles.png?raw=true
[particles_gfycat]: https://gfycat.com/beautifulseparatebeetle


## [User Interface](ui.rs)

A tour showcasing the different built-in widgets available for building
responsive user interfaces in Coffee.

The user must interact with the different widgets in order to reach the end.

```
cargo run --example ui --features opengl,debug --release
```

[![GUI][gui_gif]][gui_gfycat]

[gui_gif]: https://thumbs.gfycat.com/GloomyWeakHammerheadshark-small.gif
[gui_gfycat]: https://gfycat.com/gloomyweakhammerheadshark


## [Mesh](mesh.rs)

A simple mesh viewer showcasing the `Mesh` and `Shape` types.

It renders different shapes and the user is able to tweak some settings using
the user interface.

```
cargo run --example mesh --features opengl,debug --release
```

[![GUI][mesh_gif]][mesh_gfycat]

[mesh_gif]: https://thumbs.gfycat.com/AcademicGlossyKingfisher-small.gif
[mesh_gfycat]: https://gfycat.com/academicglossykingfisher

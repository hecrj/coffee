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

## Particles

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

![Particles example][particles]

[particles]: https://github.com/hecrj/coffee/blob/master/images/examples/particles.png?raw=true

## User Interface

A tour showcasing the different built-in widgets available for building
responsive user interfaces in Coffee.

```
cargo run --example ui --features opengl,debug --release
```

[![GUI](https://thumbs.gfycat.com/LivelyOnlyHypacrosaurus-size_restricted.gif)](https://gfycat.com/livelyonlyhypacrosaurus)

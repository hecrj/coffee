# Examples

To try an example, clone the repository and use `cargo run`. You have to enable
a graphics backend feature. For instance, if we want to run an example using
OpenGL, we run:

```
cargo run --example <example> --features opengl
```

Coffee moves fast and the `master` branch can contain breaking changes! Be sure
to browse the [latest tag] if you want to learn about the latest release.

[latest tag]: https://github.com/hecrj/coffee/tree/0.2.0/examples

## Particles

A particle gravity simulator that showcases a loading screen, input handling,
and graphics interpolation with batched drawing and font rendering. Move the
mouse around to attract the particles.

This example renders 50k independent particles every frame. Using the
`--release` flag to run the example is recommended. Additionally, you can
compile it with the `debug` feature if you want to enable the built-in debug
view:

```
cargo run --example particles --features opengl,debug --release
```

![Particles example][particles]

[particles]: https://github.com/hecrj/coffee/blob/master/images/examples/particles.png?raw=true

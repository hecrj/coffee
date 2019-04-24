# Coffee

[![Build Status](https://travis-ci.org/hecrj/coffee.svg?branch=master)](https://travis-ci.org/hecrj/coffee)
[![Documentation](https://docs.rs/coffee/badge.svg)](https://docs.rs/coffee)
[![Crates.io](https://img.shields.io/crates/v/coffee.svg)](https://crates.io/crates/coffee)
[![License](https://img.shields.io/crates/l/coffee.svg)](https://github.com/hecrj/coffee/blob/master/LICENSE)

An opinionated 2D game engine for Rust focused on simplicity, explicitness, and
type-safety.

__Coffee is in a very early stage of development.__ Active development is
planned during 2019 (and hopefully beyond that!). Many [basic features are still
missing], some [dependencies are experimental], and there are probably _many_
bugs. [Feel free to contribute!]

[basic features are still missing]: https://github.com/hecrj/coffee/issues?q=is%3Aissue+is%3Aopen+label%3Afeature
[dependencies are experimental]: #implementation-details
[Feel free to contribute!]: #contributing--feedback

## Features
  * Declarative, type-safe asset loading
  * Loading screens with progress tracking
  * Built-in [debug view with performance metrics]
  * Fixed timestep
  * Explicit, easy to use, hardware-accelerated 2D graphics API
  * Multiplatform support leveraging OpenGL, Vulkan, Metal, D3D11, and D3D12
  * Texture array support
  * Explicit and efficient batched draws
  * Off-screen rendering
  * TrueType font rendering

[debug view with performance metrics]: https://github.com/hecrj/coffee/blob/master/images/debug.png

## Usage
Add `coffee` as a dependency in your `Cargo.toml` and enable a graphics backend
feature (`opengl`, `vulkan`, `metal`, `dx11`, or `dx12`):

```toml
coffee = { version = "0.1", features = ["opengl"] }
```

Rust is quite slow in debug mode. If you experience performance issues when
drawing hundreds of sprites, enable compiler optimizations in your `Cargo.toml`.
I recommend level 2 optimizations in order to stay closer to `--release`
performance:

```toml
[profile.dev]
opt-level = 2
```

## Overview
Here is a minimal example that will open a window:

```rust
use coffee::{Game, Result, Timer};
use coffee::graphics::{Color, Window, WindowSettings};

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
    })
}

struct MyGame {
    // Your game state goes here...
}

impl Game for MyGame {
    type View = (); // No view data.
    type Input = (); // No input data.

    const TICKS_PER_SECOND: u16 = 60; // Update rate

    fn new(_window: &mut Window) -> Result<(MyGame, Self::View, Self::Input)> {
        // Load your game assets here. Check out the `load` module!
        Ok((MyGame { /* ... */ }, (), ()))
    }

    fn update(&mut self, _view: &Self::View, _window: &Window) {
        // Update your game here
    }

    fn draw(&self, _view: &mut Self::View, window: &mut Window, _timer: &Timer) {
        // Clear the current frame
        let mut frame = window.frame();
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!
    }
}
```

Check out the [documentation] and the [examples] to learn more!

[documentation]: https://docs.rs/coffee
[examples]: https://github.com/hecrj/coffee/tree/master/examples

## Implementation details
Coffee builds upon

  * [`winit`] for windowing and mouse/keyboard events.
  * [`gfx` pre-ll] for OpenGL support, based heavily on the [`ggez`] codebase.
  * [`wgpu`] for _experimental_ Vulkan, Metal, D3D11 and D3D12 support.
  * [`glyph_brush`] for TrueType font rendering.
  * [`nalgebra`] for the `Point`, `Vector`, and `Transformation` types.
  * [`image`] for image loading and texture array building.

[`winit`]: https://github.com/rust-windowing/winit
[`gfx` pre-ll]: https://github.com/gfx-rs/gfx/tree/pre-ll
[`wgpu`]: https://github.com/gfx-rs/wgpu
[`glyph_brush`]: https://github.com/alexheretic/glyph-brush/tree/master/glyph-brush
[`nalgebra`]: https://github.com/rustsim/nalgebra
[`image`]: https://github.com/image-rs/image

## Contributing / Feedback
I am quite new to Rust, systems programming, and computer graphics. I am
learning along the way as I build the engine for a game I am currently
developing. I am always glad to to learn from anyone.

If you want to contribute, you are more than welcome to be a part of the
project! Check out the current [issues] if you want to find something to work
on. Try to share you thoughts first! Feel free to open a new issue if you want
to discuss new ideas.

Any kind of feedback is welcome! You can open an issue or, if you want to talk,
you can find me (and a bunch of awesome folks) over the `#game-dev` channel in
the [Rust Community Discord]. I go by `@lone_scientist` there.

[issues]: https://github.com/hecrj/coffee/issues
[Rust Community Discord]: https://bit.ly/rust-community

## Credits / Thank you
  * [`ggez`], an awesome, easy-to-use, good game engine that introduced me to
    Rust a month ago. Its graphics implementation served me as a guide to
    implement OpenGL support for Coffee.

[`ggez`]: https://github.com/ggez/ggez

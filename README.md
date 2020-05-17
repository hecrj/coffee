# Coffee

[![Integration status](https://github.com/hecrj/coffee/workflows/Integration/badge.svg)](https://github.com/hecrj/coffee/actions)
[![Documentation](https://docs.rs/coffee/badge.svg)](https://docs.rs/coffee)
[![Crates.io](https://img.shields.io/crates/v/coffee.svg)](https://crates.io/crates/coffee)
[![License](https://img.shields.io/crates/l/coffee.svg)](https://github.com/hecrj/coffee/blob/master/LICENSE)
[![Gitter chat](https://badges.gitter.im/hecrj/coffee.png)](https://gitter.im/hecrj/coffee)

An opinionated 2D game engine for Rust focused on simplicity, explicitness, and type-safety.

__Coffee is in a very early stage of development.__ Many [basic features are still missing], some [dependencies are experimental], and there are probably _many_ bugs. [Feel free to contribute!]

[basic features are still missing]: https://github.com/hecrj/coffee/issues?q=is%3Aissue+is%3Aopen+label%3Afeature
[dependencies are experimental]: #implementation-details
[Feel free to contribute!]: #contributing--feedback

## Features
  * [Responsive, customizable GUI]
  * Declarative, type-safe loading screens with progress tracking
  * Built-in [debug view with performance metrics]
  * Fixed, deterministic timestep
  * Explicit, easy to use, hardware-accelerated 2D graphics API
  * Multiplatform support leveraging OpenGL, Vulkan, Metal, D3D11, and D3D12
  * [Explicit and efficient batched draws]
  * [Mesh support]
  * Texture array support
  * Off-screen rendering
  * TrueType font rendering
  * Gamepad support

And more! Check out the [examples] to see them in action.

[Responsive, customizable GUI]: https://gfycat.com/gloomyweakhammerheadshark
[debug view with performance metrics]: https://github.com/hecrj/coffee/blob/master/images/debug.png
[Explicit and efficient batched draws]: https://gfycat.com/beautifulseparatebeetle
[Mesh support]: https://gfycat.com/academicglossykingfisher

## Usage
Add `coffee` as a dependency in your `Cargo.toml` and enable a graphics backend
feature (`opengl`, `vulkan`, `metal`, `dx11`, or `dx12`):

```toml
coffee = { version = "0.4", features = ["opengl"] }
```

Rust is quite slow in debug mode. If you experience performance issues when
drawing hundreds of sprites, enable compiler optimizations in your `Cargo.toml`.
I recommend level 2 optimizations in order to stay closer to `--release`
performance:

```toml
[profile.dev]
opt-level = 2
```

__Coffee moves fast and the `master` branch can contain breaking changes!__ If
you want to learn about a specific release, check out [the release list].

[the release list]: https://github.com/hecrj/coffee/releases

## Overview
Here is a minimal example that will open a window:

```rust
use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct MyGame {
    // Your game state and assets go here...
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| MyGame { /* ... */ })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!
    }
}
```

Browse the [documentation] and the [examples] to learn more!

[documentation]: https://docs.rs/coffee
[examples]: https://github.com/hecrj/coffee/tree/master/examples

## Implementation details
Coffee builds upon

  * [`winit`] for windowing and mouse/keyboard events.
  * [`gfx` pre-ll] for OpenGL support, based heavily on the [`ggez`] codebase.
  * [`wgpu`] for _experimental_ Vulkan, Metal, D3D11 and D3D12 support.
  * [`stretch`] for responsive GUI layouting based on Flexbox.
  * [`glyph_brush`] for TrueType font rendering.
  * [`gilrs`] for gamepad support.
  * [`nalgebra`] for the `Point`, `Vector`, and `Transformation` types.
  * [`image`] for image loading and texture array building.

[`winit`]: https://github.com/rust-windowing/winit
[`gfx` pre-ll]: https://github.com/gfx-rs/gfx/tree/pre-ll
[`wgpu`]: https://github.com/gfx-rs/wgpu
[`stretch`]: https://github.com/vislyhq/stretch
[`glyph_brush`]: https://github.com/alexheretic/glyph-brush/tree/master/glyph-brush
[`gilrs`]: https://gitlab.com/gilrs-project/gilrs
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
you can find me (and a bunch of awesome folks) over the `#games-and-graphics`
channel in the [Rust Community Discord]. I go by `@lone_scientist` there.

[issues]: https://github.com/hecrj/coffee/issues
[Rust Community Discord]: https://bit.ly/rust-community

## Credits / Thank you
  * [`ggez`], an awesome, easy-to-use, good game engine that introduced me to
    Rust. Its graphics implementation served me as a guide to implement OpenGL
    support for Coffee.
  * [Kenney], creators of amazing free game assets with no strings attached. The
    built-in GUI renderer in Coffee uses a modified version of their UI sprites.

[`ggez`]: https://github.com/ggez/ggez
[Kenney]: https://kenney.nl

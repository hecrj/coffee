# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Incorrect buffer sizes in the `Mesh` pipeline. This caused vertices to entirely
  disappear when rendering big meshes, leading to a potential crash.

## [0.3.1] - 2019-06-20
### Added
- Documentation about the default coordinate system of a `Target`.

### Changed
- The built-in `Debug` view now uses `µ` instead of `u` for microseconds.

### Fixed
- Resizing in Wayland. [#58]
- Outdated documentation comment in `graphics` module.
- Documentation typos.

[#58]: https://github.com/hecrj/coffee/pull/58


## [0.3.0] - 2019-06-15
### Added
- __Responsive GUI support!__ The new `ui` module can be used to extend a `Game`
  and build a user interface. [#35]
  - GUI runtime based on [Elm] and [The Elm Architecture].
  - Layouting based on Flexbox and powered by [`stretch`].
  - Built-in GUI widgets. Specifically: buttons, sliders, checkboxes, radio
    buttons, rows, and columns.
  - Built-in GUI renderer. It is capable of rendering all the built-in GUI
    widgets.
  - Customization. The `ui::core` module can be used to implement custom widgets
    and renderers.
- __Gamepad support__. The `input::Event` enum now has a `Gamepad` variant. [#29]
- __Mesh support__. The types `Shape` and `Mesh` have been introduced.
  Rectangles, circles, ellipses, and polylines can now be drawn with ease using
  fill or stroke modes. [#50]
- The `Game::LoadingScreen` associated type. Given that all the `Game`
  associated types implement a trait with a `load` method, wiring a loading
  screen now is as simple as writing its name. Because of this, the `Game::new`
  method is no longer necessary and it is dropped. [#35]
- `Input` trait. It allows to implement reusable input handlers. [#35]
- `KeyboardAndMouse` input handler. Useful to quickstart development and have
  easy access to the keyboard and the mouse from the get-go. [#35]
- `CursorTaken` and `CursorReturned` mouse input events. They are fired when the
  cursor is used/freed by the user interface. [#35]
- Off-screen text rendering support. `Font::draw` now supports any `Target`
  instead of a window `Frame`. [#25]
- `Game::debug` performance tracking. Time spent on this method is now shown in
  the built-in debug view. [#26]
- Implementation of `Default` trait for `Text`. [#25]
- `Transformation::rotate`. Creates a transformation representing a rotation. [#28]
- `Batch::clear`. Clears the batch contents, useful to reuse batches in different
  frames.
- Implementation of `Extend` for `Batch`. [#37]
- Implementation of `ParallelExtend` for `Batch`. A `Batch` can now be populated
  using multiple threads, useful to improve performance when dealing with many
  thousands of quads. [#37]
- `Text` alignment. It can be defined using the new `HorizontalAlignment` and
  `VerticalAlignment` types in the `graphics` module. [#35]
- `Font::measure`. It allows to measure the dimensions of any `Text`. [#35]
- `Rectangle::contains`. It returns whether or not a `Rectangle` contains a
  given `Point`. [#35]
- `Sprite::scale`. It can be used to change the `Sprite` size when drawed.
- `Default` implementation for `Sprite`. [#35]
- `Debug::ui_duration`. It returns the average time spent running the UI runtime.
- A counter example as an introduction to the new UI architecture. [#35]
- A user interface example that introduces the different built-in widgets. [#35]
- A gamepad example that displays the last gamepad event. [#29]
- A mesh example that showcases the different ways to use the new `Mesh` and
  `Shape` types. [#50]
- Multiple gravity centers based on mouse clicks in the particles example. [#30]

### Changed
- The `Game::Input` associated type now has to implement the new `Input` trait.
  This splits code quite nicely, as the `on_input` method moves away from `Game`.
  It also makes `Input` implementors reusable. For instance, a `KeyboardAndMouse`
  type has been implemented that can be used out of the box! [#35]
- `Game::draw` now takes a `Frame` directly instead of a `Window`. [#35]
- `LoadingScreen::on_progress` has been renamed to `LoadingScreen::draw` and it
  now receives a `Frame` instead of a `Window`. [#35]
- `input::Event` is now split into four different variants representing input
  sources: `Keyboard`, `Mouse`, `Gamepad`, and `Window`. Each one of these
  sources has its own module inside `input` with an `Event` type where the old
  variants can be found. [#29]
- `input::KeyCode` has been moved to `input::keyboard::KeyCode`. [#29]
- `input::MouseButton` has been moved to `input::mouse::Button`. [#29]
- `Batch::draw` and `texture_array::Batch::draw` do not take a `position`
  argument anymore. Using `Target::transform` before drawing is preferred. [#53]
- `Font::load` has been renamed to `Font::load_from_bytes` for consistency. [#55]
- The performance of the particles example has been improved considerably on all
  platforms. [#37]
- The `input` example uses the new `ui` module now.

### Removed
- The `Game::View` associated type. Implementors of the `Game` trait are also
  meant to hold the game assets now. This simplifies the API considerably, and
  it helps model your game state-view relationship with precision, avoiding
  inconsistencies. [#35]
- `Game::new`. `Game::load` should be used instead. [#35]
- `Game::on_input`. Input handlers now must be implemented using the new `Input`
  trait. [#35]

[#25]: https://github.com/hecrj/coffee/pull/25
[#26]: https://github.com/hecrj/coffee/pull/26
[#28]: https://github.com/hecrj/coffee/pull/28
[#29]: https://github.com/hecrj/coffee/pull/29
[#30]: https://github.com/hecrj/coffee/pull/30
[#35]: https://github.com/hecrj/coffee/pull/35
[#37]: https://github.com/hecrj/coffee/pull/37
[#50]: https://github.com/hecrj/coffee/pull/50
[#53]: https://github.com/hecrj/coffee/pull/53
[#55]: https://github.com/hecrj/coffee/pull/55
[Elm]: https://elm-lang.org
[The Elm Architecture]: https://guide.elm-lang.org/architecture/
[`stretch`]: https://github.com/vislyhq/stretch


## [0.2.0] - 2019-04-28
### Added
- `Game::on_close_request` to control whether the game should be closed when the
  window receives a close request by the OS. [#14]
- `input::Event::TextInput` event, which triggers on text entry. Contains the
  character typed as a `char`. [#15]
- `input::Event::CursorEntered` and `input::Event::CursorLeft` events, which
  trigger when the mouse cursor enters or leaves the game window, respectively.
  [#15]
- `input::Event::MouseWheel`, which triggers when the mouse wheel is scrolled.
  Contains the number of horizontal and vertical lines scrolled as `f32`. [#15]
- `input::Event::WindowFocused` and `input::Event::WindowUnfocused`, which
  trigger when the game window gains or loses focus, respectively. [#15]
- `input::Event::WindowMoved`, which triggers when the game window is moved.
  Contains the new X and Y coordinates of the window as `f32`. [#15]
- Text rendering for the [`wgpu`] graphics backend. Vulkan, Metal, D3D11 and
  D3D12 now support text rendering. OpenGL already supported text rendering.
  [#18]
- This changelog. [#20]
- Example to showcase input handling. [#15]
- Example to showcase proper colors and gamma correction. [#19]

### Changed
- The debug view is now shown by default when the `debug` feature is enabled.

### Fixed
- Gamma correction in the [`wgpu`] graphics backend. Clear colors, font colors,
  and blending should work as expected in Vulkan, Metal, D3D11 and D3D12. OpenGL
  was already working properly. [#19]

[#14]: https://github.com/hecrj/coffee/pull/14
[#15]: https://github.com/hecrj/coffee/pull/15
[#18]: https://github.com/hecrj/coffee/pull/18
[#19]: https://github.com/hecrj/coffee/pull/19
[#20]: https://github.com/hecrj/coffee/pull/20


## [0.1.1] - 2019-04-25
### Changed
- The wording in the `README` has been improved.

### Fixed
- Compilation failing when `debug_assertions` and the `debug` feature were
  disabled.


## [0.1.0] - 2019-04-25
### Added
- First release! :tada:

## 0.0.0 - 2019-04-02
### Changed
- The name of the crate has been reserved on [crates.io]
- Coffee starts being developed.


[Unreleased]: https://github.com/hecrj/coffee/compare/0.3.1...HEAD
[0.3.1]: https://github.com/hecrj/coffee/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/hecrj/coffee/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/hecrj/coffee/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/hecrj/coffee/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/hecrj/coffee/releases/tag/0.1.0

[crates.io]: https//crates.io
[`wgpu`]: https://github.com/gfx-rs/wgpu

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
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
- Improve wording in `README`.

### Fixed
- Fix compilation failing when `debug_assertions` and the `debug` feature were
  disabled.


## [0.1.0] - 2019-04-25
### Added
- Release the first version of the crate :tada:

## 0.0.0 - 2019-04-02
### Changed
- Reserve name on https://crates.io
- Start the crate


[Unreleased]: https://github.com/hecrj/coffee/compare/0.1.1...HEAD
[0.1.1]: https://github.com/hecrj/coffee/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/hecrj/coffee/releases/tag/0.1.0

[`wgpu`]: https://github.com/gfx-rs/wgpu

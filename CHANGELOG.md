# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Implemented `Game::on_close_request` to control whether the game should be
  closed when the window receives a close request by the OS. 
- Added `input::Event::TextInput` event, which triggers on text entry. Contains
  the character typed as a `char`.
- Added `input::Event::CursorEntered` and `input::Event::CursorLeft` events,
  which trigger when the mouse cursor enters or leaves the game window,
  respectively.
- Added `input::Event::MouseWheel`, which triggers when the mouse wheel is
  scrolled. Contains the number of horizontal and vertical lines scrolled as
  `f32`.
- Added `input::Event::WindowFocused` and `input::Event::WindowUnfocused`, which
  trigger when the game window gains or loses focus, respectively.
- Added `input::Event::WindowMoved`, which triggers when the game window is
  moved. Contains the new X and Y coordinates of the window as `f32`.
- Implemented font rendering for the [`wgpu`] graphics backend. Vulkan, Metal,
  D3D11 and D3D12 now support text rendering. OpenGL already supported font
  rendering.

### Changed
- Show debug view by default when `debug` feature is enabled.

### Fixed
- Handle gamma correction properly in the [`wgpu`] graphics backend. Clear
  colors, font colors, and blending should work as expected in Vulkan, Metal,
  D3D11 and D3D12. OpenGL was already working properly.

### Examples
- Added input display example.
- Added color example.

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

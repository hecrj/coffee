#[cfg(not(any(
    feature = "opengl",
    feature = "vulkan",
    feature = "metal",
    feature = "dx11",
    feature = "dx12",
    feature = "web",
    all(debug_assertions, feature = "empty")
)))]
compile_error!(
    "You need to enable a graphics backend feature. \
     Available options: opengl, vulkan, metal, dx11, dx12."
);

fn main() {}

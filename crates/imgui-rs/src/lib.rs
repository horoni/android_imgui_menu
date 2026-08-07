pub mod ffi;
pub mod ui;

/// User function that will be called every frame
/// need for defining imgui menu
pub type PfnImGuiRender = dyn Fn(bool) + Send + Sync + 'static;

// Specify crates
#[cfg(feature="winit")] #[macro_use]
extern crate conrod;
#[cfg(feature="winit")]
extern crate glutin;
#[cfg(feature="winit")]
extern crate winit;
#[cfg(feature="gfx_rs")]
extern crate gfx;
#[cfg(feature="gfx_rs")]
extern crate gfx_core;

// module usage

fn main() {
    let builder = glutin::window
}

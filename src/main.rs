// CLIPPY LINTS
#![deny(clippy::all)] // correctness, suspicious, style, complexity & perf
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// from clippy::restriction:
//#![warn(clippy::unwrap_used)]
#![warn(clippy::todo)]
#![warn(clippy::print_stdout)] // FIXME - BUG in clippy

// CONSOLE-FREE EXECUTABLE ON RELEASE BUILDS
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod blocks;
mod constants;
mod fonts;
mod input;
mod navigation_instructions;
mod resources;
mod snapshot;
mod stages;

fn main() {
    app::run();
}

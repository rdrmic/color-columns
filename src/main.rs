// CLIPPY LINTS
#![deny(clippy::all)] // correctness, suspicious, style, complexity & perf
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
//#![warn(clippy::unwrap_used)]   // FIXME remove!
//#![warn(clippy::print_stdout)] // FIXME BUG in clippy
#![warn(/*clippy::unwrap_used, */clippy::todo/*, clippy::print_stdout*/)] // from clippy::restriction

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

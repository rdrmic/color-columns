#![deny(clippy::all)] // correctness, suspicious, style, complexity, perf
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// from clippy::restriction:
#![warn(clippy::todo, clippy::print_stdout)]
//#![warn(clippy::unwrap_used)]
#![windows_subsystem = "windows"]

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

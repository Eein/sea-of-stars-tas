//! Sea of Stars TAS library crate.
//!
//! All the TAS logic (memory reading, sequencer, game manager, routes) lives
//! here so that it can be shared between the GUI binary (`src/main.rs`) and the
//! headless CLI binary (`src/bin/tas-cli.rs`).
//!
//! The GUI-independent runtime lives in [`core`]; the headless runner and its
//! argument parsing live in [`cli`].

pub mod assets;
pub mod cli;
pub mod combat;
pub mod config;
pub mod control;
pub mod core;
pub mod game_manager;
pub mod gui;
pub mod level_up;
pub mod memory;
pub mod route;
pub mod seq;
pub mod state;
pub mod util;

#[cfg(feature = "web")]
pub mod app;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "web")]
mod components;
pub mod game;

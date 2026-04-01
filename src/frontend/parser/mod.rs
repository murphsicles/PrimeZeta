// src/frontend/parser/mod.rs
pub mod expr;
pub mod location;
#[allow(clippy::module_inception)]
pub mod parser;
pub mod pattern;
pub mod stmt;
pub mod top_level;

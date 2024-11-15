pub mod spawn;
pub use spawn::*;

#[derive(Clone)]
pub enum Building {
    ConveyorBelt,
    Factory
}
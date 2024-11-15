pub mod taker;
pub mod storage;
pub mod renderer;
pub mod generator;

#[derive(Clone)]
pub struct Item {
    pub filepath: String
}
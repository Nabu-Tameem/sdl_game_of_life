mod cell;
mod universe;
mod engine;

use crate::engine::Engine;
fn main() {
    let mut engine = match Engine::new() {
        Ok(engine) => engine,
        Err(error) => panic!("Engine Failed: {:?}", error)
    };
    engine.run();
}

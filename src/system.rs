// hylaean/src/system.rs
use crate::world::World;

pub trait System {
    fn run(&mut self, world: &mut World);
}
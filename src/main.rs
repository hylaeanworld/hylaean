// hylaean/src/main.rs
use hylaean::world::World;
use hylaean::component::Component;
use hylaean::system::System;
use hylaean::entity::Entity;

#[derive(Debug, Clone)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {}

#[derive(Debug, Clone)]
struct Velocity {
    vx: f32,
    vy: f32,
}

impl Component for Velocity {}

struct MoveSystem;

impl System for MoveSystem {
    fn run(&mut self, world: &mut World) {
        for id in 0..world.next_entity_id {
            let entity = Entity(id);
            let vel_opt = world.get_component::<Velocity>(&entity).cloned();
            if let Some(vel) = vel_opt {
                if let Some(pos) = world.get_component_mut::<Position>(&entity) {
                    pos.x += vel.vx;
                    pos.y += vel.vy;
                }
            }
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Velocity>();

    let e = world.create_entity();
    world.add_component(e, Position { x: 0.0, y: 0.0 });
    world.add_component(e, Velocity { vx: 1.0, vy: 1.5 });

    let mut move_system = MoveSystem;
    move_system.run(&mut world);

    let pos = world.get_component::<Position>(&e).unwrap();
    println!("Entity {:?} moved to position: ({}, {})", e, pos.x, pos.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_system() {
        let mut world = World::new();
        world.register_component::<Position>();
        world.register_component::<Velocity>();

        let e = world.create_entity();
        world.add_component(e, Position { x: 2.0, y: 3.0 });
        world.add_component(e, Velocity { vx: 0.5, vy: -1.0 });

        let mut move_system = MoveSystem;
        move_system.run(&mut world);

        let pos = world.get_component::<Position>(&e).unwrap();
        assert_eq!(pos.x, 2.5);
        assert_eq!(pos.y, 2.0);
    }
}

use campfire::{World, data, system::new};

fn main() {
    let mut world = World::default();
    let player = world.new_entity();

    world[player].extend([
        data::new(Velocity { dx: 4.0, dy: 7.0 }),
        data::new(Position { x: 0.0, y: 0.0 }),
    ]);

    world.systems.extend([
        new::<(&_, &_)>(display_vars),
        new::<(&mut Position, &Velocity)>(|(pos, vel)| {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }),
        new::<&mut Velocity>(|vel| {
            vel.dx += 1.0;
            vel.dy -= 1.0;
        }),
    ]);

    world.run();
    world.run();
    world.run();
}

fn display_vars((pos, vel): (&Position, &Velocity)) {
    println!("Entity position : {}:{}", pos.x, pos.y);
    println!("       velocity : {}:{}\n", vel.dx, vel.dy);
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

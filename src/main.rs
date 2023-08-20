use campfire::{context::Ctx, system::*, *};

fn main() {
    let mut ctx = Ctx::default();
    let player = ctx.new_entity();

    ctx[player].extend([new_data(Speed(4, 7)), new_data(Position(0, 0))]);

    ctx.systems.extend([
        new::<(&_, &_)>(display_vars),
        new::<(&mut _, &_)>(|Position(x, y): &mut _, Speed(x_vel, y_vel): &_| {
            *x += x_vel;
            *y += y_vel;
        }),
        new::<&mut _>(|speed: &mut Speed| {
            speed.0 += 1;
            speed.1 -= 1;
        }),
    ]);

    ctx.next_update();
    ctx.next_update();
    ctx.next_update();
}

fn display_vars(Position(x, y): &Position, Speed(x_vel, y_vel): &Speed) {
    println!("Entity position : {x}:{y}");
    println!("       velocity : {x_vel}:{y_vel}\n");
}

#[derive(Debug, Clone)]
struct Speed(usize, usize);
impl Component for Speed {}

#[derive(Debug, Clone)]
struct Position(usize, usize);
impl Component for Position {}

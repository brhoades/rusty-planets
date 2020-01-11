mod bodies;
use piston_window::*;
use bodies::{World,Planet,Star};

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Rusty Planets", [640, 480])
        .exit_on_esc(true)
        .automatic_close(true)
        .build()
        .unwrap();

    let mut world = World{entities: vec!()};
    // world.entities.push(Box::new(Star{....}));

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            // Set the background to hipster grey.
            // Also clears anything previously drawn.
            clear([0.1, 0.1, 0.1, 1.0], graphics);
        });
    }
}

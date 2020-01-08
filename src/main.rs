mod bodies;
use piston_window::*;
use bodies::{World,Planet,Star};
use std::time::{Instant,Duration};

const STEP: Duration = Duration::from_millis(1000/60);

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
        // Limit to at most 60 frames.
        std::thread::sleep(STEP);

        window.draw_2d(&event, |context, graphics, _device| {
            // Set the background to hipster grey.
            // Also clears anything previously drawn.
            clear([0.1, 0.1, 0.1, 1.0], graphics);
        });
    }
}

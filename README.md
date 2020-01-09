# Rusty Planets Excercise

![image of sol](./images/rusty-planets-sol.gif)

For [reading consortium](reading-consortium.github.io/).

## Objectives
Let's make a universe simulator in a few hundred lines of Rust!

 * Plumb the physics and rendering pipeline through draw_2d.
 * Create multiple planets that rotate around the star.

Bonus points (level in bracket):
 * [Trivial] Use [clap](https://crates.io/crates/clap) to accept an arbitrary timescale.
 * [Novice] Add moons for some planets.
 * [Novice] Add an asteroid field.
 * [Novice] Add orbit lines.
 * [Novice] Click to spawn planets.
 * [Novice] Add planet labels.
 * [Average] Use [viewport transforms](http://docs.piston.rs/piston_window/piston_window/struct.Viewport.html) to accurately model the solar system.
 * [Average] Implement panning and zooming.
 * [Average] Thread and pool physics work.
 * [Average] Use textures for planets.
 * [Average] Add _accurate_ orbit lines.
 * [Average] Add planet information on mouse over.
 * [Master] Implement collision handling logic _well_ (break into pieces or remove smaller).
 * [Master] Model planets in 3d and project to 2d.
 * [Master] Back your physics engine by an event bus.
 * [Grandmaster] Add shadows projected from stars.

## Usage

```bash
$ cargo run
```

## Helpful Links
[Gravitational force](https://en.wikipedia.org/wiki/Gravitational_acceleration#Relation_to_the_Universal_Law) with some predefined velocities will cover most of your work. For spawning objects into a stable rotation, you're going to need [the equation for relative velocity](https://en.wikipedia.org/wiki/Circular_orbit#Velocity).

If you choose to use nalgebra, their [primary site](https://nalgebra.org/) has a lot of links on it. The imports provided in the skeleton should take care of most of your needs.

Most of the piston work has been done for you already. Just call rectange or another shape on the inner render method of the trait.
[draw_2d](http://docs.piston.rs/piston_window/piston_window/struct.PistonWindow.html#method.draw_2d) is doing the heavy lifting. See [piston_window's GitHub](https://github.com/PistonDevelopers/piston_window) for more help.

If you need tips, please feel free to hop on consortium-approved chat chanenls and ask!

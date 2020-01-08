use piston_window::*;
use std::time::Duration;

// NOTE: You do NOT need to use nalgebra. It's hairy but has great vector
// methods, including: ::norm() (2nd norm), ::normalize() (unit vector),
// and substraction to get a directional force vector.
//
// You can do all of this by hand with [f64; 2].
use nalgebra::{Point2,Vector2};

/***********************
 * Trait and struct definitions.
 * At minimu, set and tick need modification to get something usable.
 *
 * Just fill in the empty tubles, the empty methods in "impl", and complete fn main().
 ***********************/
pub trait Renderable {
    fn render(&self, ctx: &Context, graphics: &mut G2d);
}

pub trait PhysicsBody {
    fn mass(&self) -> f64;

    fn tick(&self) -> ();
    fn set(&self);
}

pub trait Entity: PhysicsBody + Renderable {
    fn name(&self) -> &'static str;
}

pub struct World {
    // Box for dynamic dispatch to multiple Entity types.
    pub entities: Vec<Box<dyn Entity>>
}

// some ideas
#[derive(Debug)]
pub struct Planet {
    velocity: Vector2<f64>,
    position: Point2<f64>,
    size: f64,
    mass: f64,
    color: [f32; 4],
}

/***********************
 * Trait definitions end
 ***********************/

impl Planet {
    // You will probably want one of these
    pub fn new() -> Self {
        Planet{
            velocity: Vector2::from([0.0; 2]),
            position: Point2::from([0.0; 2]),
            size: 1.0,
            mass: 1.0,
            color: [1.0; 4], // white, RGB and last is alpha.
        }
    }
}

impl Entity for Planet {
    fn name(&self) -> &'static str {
        ""
    }
}

impl Renderable for Planet {
    fn render(&self, context: &Context, graphics: &mut G2d) {
        let extents = ellipse::circle(self.position[0], self.position[1], self.size);

        // example:
        rectangle(
            self.color,
            extents,
            context.transform,
            graphics
        );
    }
}

impl PhysicsBody for Planet {
    // Need to return some data to calculate the next frame...
    // If you keep this method, this is likely where the math is.
    //
    // Billy: math took me a bit to get 100% right; feel free to ask for a tip.
    fn tick(&self) -> () {
        ()
    }

    fn set(&self) {
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}

pub struct Star {
    position: Point2<f64>,
    color: [f32; 4],
    mass: f64,
    size: f64,
}

impl Star {
    pub fn new(window_size: Size) -> Box<Star> {
        Box::new(Star{
            position: Point2::from([window_size.width / 2.0, window_size.height / 2.0]),
            color: [1.0, 1.0, 0.8, 1.0],
            mass: 1000.0,
            size: 15.0,
        })
    }
}

impl PhysicsBody for Star {
    // Let's pretend the star doesn't move to reduce nuttiness. You can just
    // return relevant phyiscs details here.
    fn tick(&self) -> () {
        ()
    }

    fn set(&self) {
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}

impl Renderable for Star {
    fn render(&self, context: &Context, graphics: &mut G2d) {
        let extents = ellipse::circle(self.position[0], self.position[1], self.size);

        rectangle(
            self.color,
            extents,
            context.transform,
            graphics
        );
    }
}

impl Entity for Star {
    fn name(&self) -> &'static str {
        "Star"
    }
}

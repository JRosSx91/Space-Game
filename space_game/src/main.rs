use piston_window::*;

struct Spaceship {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
}

struct Projectile {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
}

struct Enemy {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
    is_enabled: bool,
}
fn main() {
    println!("Hello, world!");
}

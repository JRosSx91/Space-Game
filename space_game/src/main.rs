use piston_window::*;
use rand::random;

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
    // window config
    let mut window: PistonWindow = WindowSettings::new("Space Game", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    //Init Structs
    let mut spaceship = Spaceship {
        x: 400.0,
        y: 500.0,
        width: 50.0,
        height: 50.0,
        speed: 5.0,
    };

    let mut projectiles: Vec<Projectile> = Vec::new();

    let mut enemies: Vec<Enemy> = Vec::new();

    //Background
    let stars_count = 100;
    let mut stars: Vec<(f64, f64)> = Vec::new();

    for _ in 0..stars_count {
        let x = rand::random::<f64>() * 800.0;
        let y = rand::random::<f64>() * 600.0;
        stars.push((x, y));
    }

    while let Some(e) = window.next() {
        // Dibujo del fondo estrellado
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g); // Limpia la pantalla con negro

            for &(x, y) in &stars {
                rectangle(
                    [1.0, 1.0, 1.0, 1.0], // Color blanco para las estrellas
                    [x, y, 1.0, 1.0],     // Tamaño de la estrella
                    c.transform,
                    g,
                );
            }

            // Dibuja la nave espacial, proyectiles y enemigos aquí
            // ...
        });

        // Resto de la lógica del juego
        // ...
    }
}

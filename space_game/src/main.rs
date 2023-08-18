use piston_window::*;
use rand::random;

struct Spaceship {
    x: f64,
    y: f64,
    size: f64,
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
    is_active: bool,
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
        size: 50.0,
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
        // Movimiento nave
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => spaceship.x -= spaceship.speed,
                Key::Right => spaceship.x += spaceship.speed,
                _ => {}
            }
        }

        spaceship.x = spaceship.x.clamp(0.0, 800.0 - spaceship.size);

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => spaceship.y -= spaceship.speed,
                Key::Down => spaceship.y += spaceship.speed,
                _ => {}
            }
        }

        spaceship.y = spaceship.y.clamp(0.0, 600.0 - spaceship.size);

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space => {
                    let projectile = Projectile {
                        x: spaceship.x + spaceship.size,
                        y: spaceship.y + spaceship.size / 2.0,
                        width: 10.0,
                        height: 5.0,
                        speed: 8.0,
                        is_active: true,
                    };
                    projectiles.push(projectile);
                }
                _ => {}
            }
        }

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
        });
        //Nave
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for &(x, y) in &stars {
                rectangle([1.0, 1.0, 1.0, 1.0], [x, y, 1.0, 1.0], c.transform, g);
            }

            // Dibujo de la nave espacial
            let spaceship_points = [
                [spaceship.x + spaceship.size, spaceship.y], // Punto derecho
                [spaceship.x, spaceship.y - (spaceship.size / 2.0)], // Punto superior izquierdo
                [spaceship.x, spaceship.y + (spaceship.size / 2.0)], // Punto inferior izquierdo
            ];

            polygon(
                [0.0, 1.0, 0.0, 1.0], // Color verde para la nave
                &spaceship_points,
                c.transform,
                g,
            );

            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                // Dibujo de los proyectiles
                for projectile in &projectiles {
                    if projectile.is_active {
                        rectangle(
                            [1.0, 1.0, 1.0, 1.0],
                            [
                                projectile.x,
                                projectile.y,
                                projectile.width,
                                projectile.height,
                            ],
                            c.transform,
                            g,
                        );
                    }
                }

                // ... (dibujo de otros elementos del juego)
            });
            // ...
        });

        // Resto de la lógica del juego
        // ...
    }
}

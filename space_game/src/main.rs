use piston_window::*;
use rand::random;

const GRAVITY_VALUE: f64 = 0.001;

struct Spaceship {
    x: f64,
    y: f64,
    size: f64,
    speed_x: f64,
    speed_y: f64,
    accel_x: f64,
    accel_y: f64,
}

struct Projectile {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
    is_active: bool,
}

struct Enemy {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed_x: f64,
    speed_y: f64,
    gravity_value: f64,
    is_enabled: bool,
}

impl Enemy {
    fn update(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;

        self.speed_y += self.gravity_value;

        if self.x < -self.width || self.y > 600.0 + self.height {
            self.is_enabled = false;
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Space Game", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let max_speed: f64 = 1.0;

    //Init Structs
    let mut spaceship = Spaceship {
        x: 400.0,
        y: 500.0,
        size: 50.0,
        speed_x: 0.0, // Inicialmente estático en el eje X
        speed_y: 0.0, // Inicialmente estático en el eje Y
        accel_x: 0.0, // Sin aceleración inicial en el eje X
        accel_y: 0.0, // Sin aceleración inicial en el eje Y
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

    let max_enemies = 10;

    while let Some(e) = window.next() {
        // Spaceship movement
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => spaceship.accel_x = -0.5,
                Key::Right => spaceship.accel_x = 0.5,
                Key::Up => spaceship.accel_y = -0.5,
                Key::Down => spaceship.accel_y = 0.5,
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Left | Key::Right => spaceship.accel_x = 0.0,
                Key::Up | Key::Down => spaceship.accel_y = 0.0,
                _ => {}
            }
        }

        spaceship.speed_x += spaceship.accel_x;
        spaceship.speed_y += spaceship.accel_y;

        // Applying limitations to acceleration
        spaceship.speed_x = spaceship.speed_x.clamp(-max_speed, max_speed);
        spaceship.speed_y = spaceship.speed_y.clamp(-max_speed, max_speed);

        spaceship.x += spaceship.speed_x;
        spaceship.y += spaceship.speed_y;

        // Applying limits to Spaceship's movement
        spaceship.x = spaceship.x.clamp(0.0, 800.0 - spaceship.size);
        spaceship.y = spaceship.y.clamp(0.0, 600.0 - spaceship.size);

        let active_enemies = enemies.iter().filter(|e| e.is_enabled).count();

        if random::<f64>() < 0.005 && active_enemies < max_enemies {
            let mut gravity = GRAVITY_VALUE;
            if random::<f64>() < 0.5 {
                gravity = -GRAVITY_VALUE; // Random reverse gravity for some enemies
            }
            enemies.push(Enemy {
                x: 800.0,
                y: rand::random::<f64>() * 600.0,
                width: 20.0,
                height: 20.0,
                speed_x: -1.2,
                speed_y: 0.0, // Without initial vertical movement till gravity affects
                is_enabled: true,
                gravity_value: gravity,
            });
        }

        for enemy in enemies.iter_mut() {
            enemy.update();
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space => {
                    let projectile = Projectile {
                        x: spaceship.x + spaceship.size,
                        y: spaceship.y + spaceship.size / 1000.0,
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
        for projectile in &mut projectiles {
            if projectile.is_active {
                projectile.x += projectile.speed;
            }
        }

        let mut score = 0;
        let mut remove_projectiles = Vec::new();
        let mut remove_enemies = Vec::new();

        for (i, projectile) in projectiles.iter().enumerate() {
            for (j, enemy) in enemies.iter().enumerate() {
                if check_collision(
                    projectile.x,
                    projectile.y,
                    projectile.width,
                    projectile.height,
                    enemy.x,
                    enemy.y,
                    enemy.width,
                    enemy.height,
                ) && projectile.is_active
                    && enemy.is_enabled
                {
                    println!("¡Colisión detectada!");
                    remove_projectiles.push(i);
                    remove_enemies.push(j);
                    // Without a break so the program can have multiple collisions
                }
            }
        }

        for &i in remove_projectiles.iter().rev() {
            projectiles[i].is_active = false;
        }
        for &j in remove_enemies.iter().rev() {
            enemies[j].is_enabled = false;
        }

        projectiles.retain(|projectile| projectile.is_active);
        enemies.retain(|enemy| enemy.is_enabled);

        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for &(x, y) in &stars {
                rectangle([1.0, 1.0, 1.0, 1.0], [x, y, 1.0, 1.0], c.transform, g);
            }
        });

        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for &(x, y) in &stars {
                rectangle([1.0, 1.0, 1.0, 1.0], [x, y, 1.0, 1.0], c.transform, g);
            }

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
            // ...
        });
        window.draw_2d(&e, |c, g, _| {
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

            for enemy in &enemies {
                ellipse(
                    [0.5, 0.5, 0.5, 1.0],
                    [
                        enemy.x - (enemy.width / 2.0),
                        enemy.y - (enemy.height / 2.0),
                        enemy.width,
                        enemy.height,
                    ],
                    c.transform,
                    g,
                );
            }

            //
        });
    }
}
fn check_collision(
    a_x: f64,
    a_y: f64,
    a_width: f64,
    a_height: f64,
    b_x: f64,
    b_y: f64,
    b_width: f64,
    b_height: f64,
) -> bool {
    if a_x + a_width < b_x {
        return false;
    }
    if a_x > b_x + b_width {
        return false;
    }
    if a_y + a_height < b_y {
        return false;
    }
    if a_y > b_y + b_height {
        return false;
    }

    return true;
}

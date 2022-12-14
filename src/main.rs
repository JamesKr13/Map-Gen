use macroquad::prelude::*;
use opensimplex_noise_rs::OpenSimplexNoise;
extern crate rand;
use macroquad::color::Color;
use rand::Rng;

const DEEP: Color = DARKBLUE;
const WATER: Color = BLUE;
const SHALLOW: Color = SKYBLUE;
const SAND: Color = GOLD;
const LAND: Color = GREEN;
const FERTILE: Color = DARKGREEN;
const RUGGARD: Color = LIGHTGRAY;
const MOUNTAIN: Color = DARKGRAY;
const SNOW: Color = WHITE;
enum Biomes {
    Shallow,
    Deep,
    Water,
    Land,
    Sand,
    Mountain,
    Fertile,
    Ruggard,
    Snow,
}
fn elevation_check(e: f32) -> Biomes {
    if e < 0.10 {
        Biomes::Deep
    } else if e < 0.35 {
        return Biomes::Water;
    } else if e < 0.48 {
        return Biomes::Shallow;
    } else if e < 0.55 {
        return Biomes::Sand;
    } else if e < 0.8 {
        return Biomes::Land;
    } else if e < 1.2 {
        return Biomes::Fertile;
    } else if e < 1.4 {
        return Biomes::Ruggard;
    } else if e < 1.8 {
        return Biomes::Mountain;
    } else {
        return Biomes::Snow;
    }
}
fn min(one: f32, two: f32) -> f32 {
    match one < two {
        true => one,
        _ => two,
    }
}
#[macroquad::main("Map Gen")]
async fn main() {
    println!("run",);
    // let mut map = DelaunayTriangulation::new();
    // map.gen_edges();
    // let seed = Some(323_542_654_652_395_721);
    // let test_seed = Some(453_856_999_999_344);
    let mut rng = rand::thread_rng();
    let mut seed = rng.gen_range(0..i64::MAX);
    let mut noise_generator = OpenSimplexNoise::new(Some(seed));
    let mut scale = 0.01;
    let mut all_elevations: Vec<Vec<f32>> = Vec::new();
    let mut value_island = 100.;
    for y in 0..screen_height() as u32 {
        let mut v1: Vec<f32> = Vec::new();
        for x in 0..screen_width() as u32 {
            let nx = 2. * x as f32 / screen_width() - 1.;
            let ny = 2. * y as f32 / screen_height() - 1.;
            let d = min(1., (nx.powf(2.) + ny.powf(2.)) / 2_f32.sqrt()) * value_island;
            v1.push(
                (1. * noise_generator.eval_2d(f64::from(x) * scale, f64::from(y) * scale) as f32
                    + 0.5
                        * noise_generator.eval_2d((f64::from(x) * scale) * 2., (f64::from(y) * scale) * 2.)
                            as f32
                    + 0.5
                        * noise_generator.eval_2d((f64::from(y) * scale) * 2., (f64::from(x) * scale) * 2.)
                            as f32
                    + 2. * noise_generator
                        .eval_2d((f64::from(x) * scale) * 1. / 2., (f64::from(y) * scale) * 1. / 2.)
                        as f32
                    + 0.25
                        * noise_generator.eval_2d((f64::from(x) * scale) * 4., (f64::from(y) * scale) * 4.)
                            as f32
                    + 0.1
                        * noise_generator
                            .eval_2d((f64::from(x) * scale) * 10., (f64::from(y) * scale) * 10.)
                            as f32)
                    .powf(2.)
                    .powf(0.5)
                    .powf(0.9)
                    - 0.04
                    + (1. - d) / 2.,
            );
            // println!("{}", v1[v1.len()-1]);
        }
        all_elevations.push(v1);
    }
    let mut visual_value = 1.;
    let mut pos = 0.;
    println!("complete noise gen");
    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;
    let world_up = vec3(0.0, 1.0, 0.0);

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up = right.cross(front).normalize();
    set_cursor_grab(grabbed);
    show_mouse(false);
    loop {
        clear_background(SKYBLUE);
        set_camera(&Camera3D {
            position,
            up,
            target: position + front,
            ..Default::default()
        });
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }
        if is_key_down(KeyCode::I) {
            visual_value += 1.;
        }
        if is_key_down(KeyCode::O) && visual_value != 1. {
            visual_value -= 1.;
        }
        if is_key_down(KeyCode::P) {
            value_island -= 1.;
            for y in 0..(600. / visual_value) as u32 {
                for x in 0..(800. / visual_value) as u32 {
                    let nx = 2. * x as f32 / screen_width() - 1.;
                    let ny = 2. * y as f32 / screen_height() - 1.;
                    let d = min(1., (nx.powf(2.) + ny.powf(2.)) / 2_f32.sqrt())
                        * (value_island + 1.);
                    all_elevations[y as usize][x as usize] -= (1. - d) / 2.;
                    let nx = 2. * x as f32 / screen_width() - 1.;
                    let ny = 2. * y as f32 / screen_height() - 1.;
                    let d =
                        min(1., (nx.powf(2.) + ny.powf(2.)) / 2_f32.sqrt()) * value_island;
                    all_elevations[y as usize][x as usize] += (1. - d) / 2.;
                }
            }
        }
        if is_key_down(KeyCode::U) {
            scale *= 2.;
            draw_text("Please Wait While Map is created", 250., 100., 30., BLACK);
            noise_generator = OpenSimplexNoise::new(Some(seed));
            all_elevations = Vec::new();
            for y in 0..screen_height() as u32 {
                let mut v1: Vec<f32> = Vec::new();
                for x in 0..screen_width() as u32 {
                    let nx = 2. * x as f32 / screen_width() - 1.;
                    let ny = 2. * y as f32 / screen_height() - 1.;
                    let d =
                        min(1., (nx.powf(2.) + ny.powf(2.)) / 2_f32.sqrt()) * value_island;
                    v1.push(
                        (1. * noise_generator.eval_2d(f64::from(x) * scale, f64::from(y) * scale) as f32
                            + 0.5
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 2., (f64::from(y) * scale) * 2.)
                                    as f32
                            + 0.5
                                * noise_generator
                                    .eval_2d((f64::from(y) * scale) * 2., (f64::from(x) * scale) * 2.)
                                    as f32
                            + 2. * noise_generator
                                .eval_2d((f64::from(x) * scale) * 1. / 2., (f64::from(y) * scale) * 1. / 2.)
                                as f32
                            + 0.25
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 4., (f64::from(y) * scale) * 4.)
                                    as f32
                            + 0.1
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 10., (f64::from(y) * scale) * 10.)
                                    as f32)
                            .powf(2.)
                            .powf(0.5)
                            .powf(0.9)
                            - 0.04
                            + (1. - d) / 2.,
                    );
                    // println!("{}", v1[v1.len()-1]);
                }
                all_elevations.push(v1);
            }
        }
        if is_key_down(KeyCode::Y) {
            scale /= 2.;
            draw_text("Please Wait While Map is created", 250., 100., 30., BLACK);
            noise_generator = OpenSimplexNoise::new(Some(seed));
            all_elevations = Vec::new();
            for y in 0..screen_height() as u32 {
                let mut v1: Vec<f32> = Vec::new();
                for x in 0..screen_width() as u32 {
                    let nx = 2. * x as f32 / screen_width() - 1.;
                    let ny = 2. * y as f32 / screen_height() - 1.;
                    let d =
                        min(1., (nx.powf(2.) + ny.powf(2.)) / 2_f32.sqrt()) * value_island;
                    v1.push(
                        (1. * noise_generator.eval_2d(f64::from(x) * scale, f64::from(y) * scale) as f32
                            + 0.5
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 2., (f64::from(y) * scale) * 2.)
                                    as f32
                            + 0.5
                                * noise_generator
                                    .eval_2d((f64::from(y) * scale) * 2., (f64::from(x) * scale) * 2.)
                                    as f32
                            + 2. * noise_generator
                                .eval_2d((f64::from(x) * scale) * 1. / 2., (f64::from(y) * scale) * 1. / 2.)
                                as f32
                            + 0.25
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 4., (f64::from(y) * scale) * 4.)
                                    as f32
                            + 0.1
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 10., (f64::from(y) * scale) * 10.)
                                    as f32)
                            .powf(2.)
                            .powf(0.5)
                            .powf(0.9)
                            * 5.
                            - 0.04
                            + (1. - d) / 2.,
                    );
                    // println!("{}", v1[v1.len()-1]);
                }
                all_elevations.push(v1);
            }
        }
        if is_key_down(KeyCode::L) {
            value_island += 1.;
            for y in 0..(600. / visual_value) as u32 {
                for x in 0..(800. / visual_value) as u32 {
                    let nx = 2. * x as f32 / screen_width() - 1.;
                    let ny = 2. * y as f32 / screen_height() - 1.;
                    let d = min(1., (nx.powf(2.) + ny.powf(2.)) / 2_f32.sqrt())
                        * (value_island - 1.);
                    all_elevations[y as usize][x as usize] -= (1. - d) / 2.;
                    let nx = 2. * x as f32 / screen_width() - 1.;
                    let ny = 2. * y as f32 / screen_height() - 1.;
                    let d =
                        min(1., (nx.powf(2.) + ny.powf(2.)) / 2_f32.sqrt()) * value_island;
                    all_elevations[y as usize][x as usize] += (1. - d) / 2.;
                }
            }
        }
        // draw_poly(300.,300.,4,20.,70., RED);

        if is_key_down(KeyCode::Space) {
            draw_text("Please Wait While Map is created", 250., 100., 30., BLACK);
            seed = rng.gen_range(0..i64::MAX);
            noise_generator = OpenSimplexNoise::new(Some(seed));
            all_elevations = Vec::new();
            for y in 0..screen_height() as u32 {
                let mut v1: Vec<f32> = Vec::new();
                for x in 0..screen_width() as u32 {
                    let nx = 2. * x as f32 / screen_width() - 1.;
                    let ny = 2. * y as f32 / screen_height() - 1.;
                    let d =
                        min(1., (nx.powf(2.) + ny.powf(2.)) / 2_f32.sqrt()) * value_island;
                    v1.push(
                        (1. * noise_generator.eval_2d(f64::from(x) * scale, f64::from(y) * scale) as f32
                            + 0.5
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 2., (f64::from(y) * scale) * 2.)
                                    as f32
                            + 0.5
                                * noise_generator
                                    .eval_2d((f64::from(y) * scale) * 2., (f64::from(x) * scale) * 2.)
                                    as f32
                            + 2. * noise_generator
                                .eval_2d((f64::from(x) * scale) * 1. / 2., (f64::from(y) * scale) * 1. / 2.)
                                as f32
                            + 0.25
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 4., (f64::from(y) * scale) * 4.)
                                    as f32
                            + 0.1
                                * noise_generator
                                    .eval_2d((f64::from(x) * scale) * 10., (f64::from(y) * scale) * 10.)
                                    as f32)
                            .powf(2.)
                            .powf(0.5)
                            .powf(0.9)
                            - 0.04
                            + (1. - d) / 2.,
                    );
                    // println!("{}", v1[v1.len()-1]);
                }
                all_elevations.push(v1);
            }
        }

        if is_key_down(KeyCode::T) {
            pos += 5.;
        }
        draw_plane(
            Vec3::new(0., 0., 0.),
            Vec2::new(screen_width(), screen_height()),
            None,
            BLUE,
        );
        for y in 0..(600. / visual_value) as u32 {
            for x in 0..(800. / visual_value) as u32 {
                // let colour = Color::new(0., 0.,0.,all_elevations[y as usize][x as usize]);
                let colour = match elevation_check(all_elevations[y as usize][x as usize]) {
                    Biomes::Water => WATER,
                    Biomes::Sand => SAND,
                    Biomes::Mountain => MOUNTAIN,
                    Biomes::Land => LAND,
                    Biomes::Fertile => FERTILE,
                    Biomes::Ruggard => RUGGARD,
                    Biomes::Shallow => SHALLOW,
                    Biomes::Deep => DEEP,
                    _ => SNOW,
                };

                if colour != DEEP {
                    if colour != WATER && colour != SHALLOW {
                        draw_cube(
                            Vec3::new(
                                visual_value * x as f32 - screen_width() / 2.,
                                0.,
                                visual_value * y as f32 - screen_height() / 2.,
                            ),
                            Vec3::new(
                                visual_value,
                                (all_elevations[y as usize][x as usize] - 0.48) * 15.,
                                visual_value,
                            ),
                            None,
                            colour,
                        );
                    } else {
                        draw_cube(
                            Vec3::new(
                                visual_value * x as f32 - screen_width() / 2.,
                                0.,
                                visual_value * y as f32 - screen_height() / 2.,
                            ),
                            Vec3::new(visual_value, 1., visual_value),
                            None,
                            colour,
                        );
                    }
                }
            }
        }
        if is_key_down(KeyCode::Up) {
            position += front;
        }
        if is_key_down(KeyCode::Down) {
            position -= front;
        }
        if is_key_down(KeyCode::Left) {
            position -= right;
        }
        if is_key_down(KeyCode::Right) {
            position += right;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;
        let delta = get_frame_time();
        yaw += mouse_delta.x * delta;
        pitch += mouse_delta.y * delta * -1.;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };
        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        set_default_camera();
        draw_text(
            &format!("Island value (P down L up): {value_island}"),
            0.,
            20.,
            20.,
            BLACK,
        );
        draw_text(
            &format!("Visual value (I up O down): {visual_value}"),
            0.,
            42.,
            20.,
            BLACK,
        );
        draw_text(
            &format!("Seed value (Space): {seed}"),
            0.,
            64.,
            20.,
            BLACK,
        );
        draw_text(
            &format!("Island Scale (Y down U up): {scale}"),
            0.,
            84.,
            20.,
            BLACK,
        );
        draw_text(
            format!("X: {} Y: {}", mouse_position.x, mouse_position.y).as_str(),
            10.0,
            48.0 + 18.0,
            30.0,
            BLACK,
        );
        next_frame().await
    }
}

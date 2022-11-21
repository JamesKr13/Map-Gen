
use macroquad::prelude::*;
use opensimplex_noise_rs::*;
extern crate rand;
use rand::Rng;
use macroquad::color::Color;

const DEEP : Color = DARKBLUE;
const WATER : Color = BLUE;
const SHALLOW : Color = SKYBLUE;
const SAND : Color = GOLD;
const LAND : Color = GREEN;
const FERTILE : Color = DARKGREEN;
const RUGGARD : Color = LIGHTGRAY;
const MOUNTAIN : Color = DARKGRAY;
const SNOW : Color = WHITE;
enum Biomes {
    Shallow, Deep,Water, Land, Sand, Mountain, Fertile, Ruggard, Snow
}
fn elevation_check(e: f32) -> Biomes{
    if e < 0.30 {
        return Biomes::Deep
    }
    else if e < 0.39 {
        return Biomes::Water
    }
    else if e < 0.48 {
        return Biomes::Shallow;
    }
    else if e < 0.55 {
        return Biomes::Sand;
    }
    else if e < 0.8 {
        return Biomes::Land;
    }
    else if e < 1.2 {
        return Biomes::Fertile;
    }
    else if e < 1.4 {
        return Biomes::Ruggard;
    }
    else if e < 2.2 {
        return Biomes::Mountain
    }
    else {
        return Biomes::Snow
    }
}
fn min(one: f32,two: f32) -> f32{
    match one < two {
        true => one,
        _ => two,
    }
} 
#[macroquad::main("Map Gen")]
async fn main() { 
    // let mut map = DelaunayTriangulation::new();
    // map.gen_edges();
    // let seed = Some(323_542_654_652_395_721);
    // let test_seed = Some(453_856_999_999_344);
    let mut rng = rand::thread_rng();
    let mut seed = rng.gen_range(0..i64::MAX);
    let mut noise_generator = OpenSimplexNoise::new(Some(seed));
    let scale = 0.04;
    let mut all_elevations: Vec<Vec<f32>> = Vec::new();
    let mut value_island = 100.;
    for y in 0..screen_height() as u32 {
        let mut v1: Vec<f32> = Vec::new();
        for x in 0..screen_width() as u32 {
            let nx = 2.*x as f32/screen_width() -1.;
            let ny = 2.*y as f32/screen_height() -1.;
            let d = min(1.,(nx.powf(2.)+ny.powf(2.))/(2. as f32).sqrt()) * value_island;
            v1.push(
                (1.*noise_generator.eval_2d(x as f64 * scale, y as f64 * scale) as f32
            + 0.5* noise_generator.eval_2d((x as f64 * scale) * 2., (y as f64 * scale)*2.) as f32
            + 0.5* noise_generator.eval_2d((y as f64 * scale)*2.,(x as f64 * scale) * 2.) as f32  
            + 2.* noise_generator.eval_2d((x as f64 * scale) * 1./2., (y as f64 * scale)*1./2.) as f32   
            + 0.25 * noise_generator.eval_2d((x as f64 * scale) * 4., (y as f64 * scale) * 4.) as f32
            + 0.1 * noise_generator.eval_2d((x as f64 * scale) * 10., (y as f64 * scale) * 10.) as f32).powf(2.).powf(0.5).powf(0.9) - 0.04 + (1.-d)/2.
            );
            // println!("{}", v1[v1.len()-1]);
        }
        all_elevations.push(v1);
    }
    let mut visual_value = 1.;
    
    println!("complete noise gen");
    loop {

        clear_background(BLUE);
        if is_key_down(KeyCode::I) {
            visual_value += 1.;
        }
        if is_key_down(KeyCode::O) {
            if visual_value != 1. {
                visual_value -= 1.;
            }
        }
        if is_key_down(KeyCode::P) {
            value_island -= 1.;
            for y in 0..(600./visual_value) as u32 {
                for x in 0..(800./visual_value) as u32 {
                    let nx = 2.*x as f32/screen_width() -1.;
                    let ny = 2.*y as f32/screen_height() -1.;
                    let d = min(1.,(nx.powf(2.)+ny.powf(2.))/(2. as f32).sqrt()) * (value_island+1.);
                    all_elevations[y as usize][x as usize] -= (1.-d)/2.;
                    let nx = 2.*x as f32/screen_width() -1.;
                    let ny = 2.*y as f32/screen_height() -1.;
                    let d = min(1.,(nx.powf(2.)+ny.powf(2.))/(2. as f32).sqrt()) * value_island;
                    all_elevations[y as usize][x as usize] += (1.-d)/2.;
                    
                }
            }
        }
        if is_key_down(KeyCode::L) {
            value_island += 1.;
            for y in 0..(600./visual_value) as u32 {
                for x in 0..(800./visual_value) as u32 {
                    let nx = 2.*x as f32/screen_width() -1.;
                    let ny = 2.*y as f32/screen_height() -1.;
                    let d = min(1.,(nx.powf(2.)+ny.powf(2.))/(2. as f32).sqrt()) * (value_island-1.);
                    all_elevations[y as usize][x as usize] -= (1.-d)/2.;
                    let nx = 2.*x as f32/screen_width() -1.;
                    let ny = 2.*y as f32/screen_height() -1.;
                    let d = min(1.,(nx.powf(2.)+ny.powf(2.))/(2. as f32).sqrt()) * value_island;
                    all_elevations[y as usize][x as usize] += (1.-d)/2.;
                    
                }
            }
            
        }
        
        for y in 0..(600./visual_value) as u32 {
            for x in 0..(800./visual_value) as u32 {
                // let colour = Color::new(0., 0.,0.,all_elevations[y as usize][x as usize]);
                let colour = match elevation_check(all_elevations[y as usize][x as usize] ) {
                    Biomes::Water => WATER,
                    Biomes::Sand => SAND,
                    Biomes::Mountain => MOUNTAIN,
                    Biomes::Land => LAND,
                    Biomes::Fertile => FERTILE,
                    Biomes::Ruggard => RUGGARD,
                    Biomes::Shallow => SHALLOW,
                    Biomes::Deep => DEEP,
                    _ => SNOW, };
                    
                if colour != BLUE {
                    draw_rectangle(visual_value*x as f32, visual_value*y as f32, visual_value,visual_value,colour);
                }
               
                }
            
        }
        if is_key_down(KeyCode::Space) {
            draw_text("Please Wait While Map is created",250.,100., 30., BLACK);
            seed = rng.gen_range(0..i64::MAX);
            noise_generator = OpenSimplexNoise::new(Some(seed));
            all_elevations = Vec::new();
            for y in 0..screen_height() as u32 {
                let mut v1: Vec<f32> = Vec::new();
                for x in 0..screen_width() as u32 {
                    let nx = 2.*x as f32/screen_width() -1.;
                    let ny = 2.*y as f32/screen_height() -1.;
                    let d = min(1.,(nx.powf(2.)+ny.powf(2.))/(2. as f32).sqrt()) * value_island;
                    v1.push(
                        (1.*noise_generator.eval_2d(x as f64 * scale, y as f64 * scale) as f32
                    + 0.5* noise_generator.eval_2d((x as f64 * scale) * 2., (y as f64 * scale)*2.) as f32
                    + 0.5* noise_generator.eval_2d((y as f64 * scale)*2.,(x as f64 * scale) * 2.) as f32  
                    + 2.* noise_generator.eval_2d((x as f64 * scale) * 1./2., (y as f64 * scale)*1./2.) as f32   
                    + 0.25 * noise_generator.eval_2d((x as f64 * scale) * 4., (y as f64 * scale) * 4.) as f32
                    + 0.1 * noise_generator.eval_2d((x as f64 * scale) * 10., (y as f64 * scale) * 10.) as f32).powf(2.).powf(0.5).powf(0.9) - 0.04 + (1.-d)/2.
                    );
                    // println!("{}", v1[v1.len()-1]);
                }
                all_elevations.push(v1);
            }
    
        }
        draw_text(&format!("Island value (P down L up): {}",value_island),0.,20.,20.,BLACK);
    draw_text(&format!("Visual value (I up O down): {}",visual_value),0.,42.,20.,BLACK);
    draw_text(&format!("Seed value (Space): {}",seed),0.,64.,20.,BLACK);
        next_frame().await
    }
}

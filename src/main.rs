/*
   ███████╗ ██████╗ ███╗   ██╗███████╗ ██████╗  ██╗
   ╚══███╔╝██╔═══██╗████╗  ██║██╔════╝██╔═████╗███║
     ███╔╝ ██║   ██║██╔██╗ ██║█████╗  ██║██╔██║╚██║
    ███╔╝  ██║   ██║██║╚██╗██║██╔══╝  ████╔╝██║ ██║
   ███████╗╚██████╔╝██║ ╚████║███████╗╚██████╔╝ ██║
   ╚══════╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝ ╚═════╝  ╚═╝

   Author : abalouri
   File   : main.rs
   Project: smart-road
   Date   : 3/02/2026
*/

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::VecDeque;
use std::time::Duration;

const CAR_WIDTH: u32 = 35;
const CAR_HEIGHT: u32 = 30;
const DISTANCE: i32 = 30;
const SAFE_DISTANCE: i32 = 300;

mod vehicule;
use vehicule::*;
mod data;
use data::*;

// Function bach t-charji t-sawer bla SDL2_image (Pure Rust)
fn load_texture_from_path<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    path: &str,
) -> Result<Texture<'a>, String> {
    let img = image::open(path)
        .map_err(|e| format!("Error loading image {}: {}", path, e))?
        .to_rgba8();
    let (width, height) = img.dimensions();
    let mut surface = Surface::new(width, height, PixelFormatEnum::RGBA32)?;
    surface.with_lock_mut(|buffer| {
        buffer.copy_from_slice(&img);
    });
    texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("==== Smart Road (Pure Rust Image Loading) ====", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Charji t-sawer (T-akked blli l-fichies f had l-blasa)
    let car_texture = load_texture_from_path(&texture_creator, "src/img/aaaa.png")?;
    let road_texture = load_texture_from_path(&texture_creator, "src/img/road.jpg")?;

    let mut rect: VecDeque<Vehicule> = VecDeque::new();
    // let mut nbr_cars: i32 = 0;
    // let mut collision_just = 0;
    // let nb_collision = 0;
    // let max_speed: i32 = 3;
    // let min_speed: i32 = 1;
    let mut can_add = false;
    let mut cooldown_time = 0;
    let mut vec_timer: Vec<Duration> = Vec::new();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        if can_add {
            cooldown_time += 1;
            if cooldown_time >= 300 {
                can_add = false;
                cooldown_time = 0;
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // if !vec_timer.is_empty() {
                    //     let max_timer = vec_timer.iter().max().unwrap();
                    //     let min_timer = vec_timer.iter().min().unwrap();
                    //     write_stats(nbr_cars, collision_just, nb_collision, max_speed, min_speed, max_timer, min_timer);
                    // }
                    break 'running;
                }
                Event::KeyDown { keycode: Some(k), .. } => {
                    if !can_add {
                        let mut rng = rand::thread_rng();
                        let (x, y, dir, angle) = match k {
                            Keycode::Up    => (410 + (rng.gen_range(0..3)*45), 800, Direction::Up, 0.0),
                            Keycode::Down  => (275 + (rng.gen_range(0..3)*45), 0, Direction::Down, 180.0),
                            Keycode::Left  => (800, 270 + (rng.gen_range(0..3)*45), Direction::Left, -90.0),
                            Keycode::Right => (0, 400 + (rng.gen_range(0..3)*45), Direction::Right, 90.0),
                            _ => (0, 0, Direction::Up, 0.0),
                        };

                        if k == Keycode::Up || k == Keycode::Down || k == Keycode::Left || k == Keycode::Right  {
                            let ranger = rng.gen_range(0..3)*45;
                            let mut v = Vehicule::new(x, y, dir, angle);
                            if ranger == 0 || ranger == 90 { v.turning = true; }
                            rect.push_back(v);
                            can_add = true;
                        }
                    }
                }
                _ => {}
            }
        }

        // Logic d'update
        let mut new_cars = VecDeque::new();
        let current_state = rect.clone();
        for (i, v_mut) in rect.iter_mut().enumerate() {
            let mut actif = true;
            let mut state_accel = true;

            for (j, v_other) in current_state.iter().enumerate() {
                if i != j {
                    if v_mut.collides_with(v_other, SAFE_DISTANCE) { state_accel = false; }
                    if v_mut.collides_with(v_other, DISTANCE) {
                        actif = false;
                       // if v_mut.states { collision_just += 1; }
                        v_mut.states = false;
                        break;
                    }
                }
            }

            if actif {
                if v_mut.frame_count >= 10 {
                    v_mut.speed = if state_accel { 3 } else { 1 };
                    v_mut.update();
                    v_mut.frame_count = 0;
                } else {     
                    v_mut.frame_count += 1; 
                }
            }
            let out = match v_mut.direction {
                Direction::Up => v_mut.y < -30, Direction::Down => v_mut.y > 830,
                Direction::Left => v_mut.x < -30, Direction::Right => v_mut.x > 830,
            };

            if out {
              // nbr_cars += 1;
                vec_timer.push(v_mut.timer.elapsed());
            } else {
                new_cars.push_back(*v_mut);
            }
        }
        rect = new_cars;

        // --- DRAWING ---
        canvas.set_draw_color(Color::RGB(0, 0, 0)); 
        canvas.clear();

        // 1. Rsem l-road hya l-lowla bach t-kon f l-background
        let background_rect = Rect::new(0, 0, 800, 800);
        canvas.copy(&road_texture, None, Some(background_rect))?;

        // 2. Rsem l-cars foq l-road
        for v in &rect {
            let target = Rect::new(v.x, v.y, CAR_WIDTH, CAR_HEIGHT);
            canvas.copy_ex(&car_texture, None, Some(target), v.angle, None, false, false)?;
        }

        canvas.present();
    }
    Ok(())
}
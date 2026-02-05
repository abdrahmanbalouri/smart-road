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
use sdl2::render::Canvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::collections::VecDeque;
use std::time::Duration;

use image::{Rgba, RgbaImage};
use rusttype::{Font, Scale};

const CAR_WIDTH: u32 = 35;
const CAR_HEIGHT: u32 = 30;
const DISTANCE: i32 = 40;
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
    texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
}
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("==== Smart Road (Pure Rust Image Loading) ====", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Charji t-sawer (T-akked blli l-fichies f had l-blasa)
    let car_texture = load_texture_from_path(&texture_creator, "src/img/aaaa.png")?;
    let road_texture = load_texture_from_path(&texture_creator, "src/img/road.jpg")?;

    let mut rect: VecDeque<Vehicule> = VecDeque::new();
    //  let mut nbr_of_cars: i32 = 0;
    let max_speed: i32 = 3;
    let min_speed: i32 = 1;
    let mut can_add = false;
    let mut cooldown_time = 0;
    let mut close_calls: i32 = 0;
    let nbr_cars: i32 = 1000;
    let mut vec_timer: Vec<Duration> = Vec::new();

    let mut event_pump = sdl_context.event_pump()?;

    let mut ask_exit = false;
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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if !vec_timer.is_empty() {
                        if !ask_exit {
                            let max_timer = vec_timer.iter().max().unwrap();
                            let min_timer = vec_timer.iter().min().unwrap();
                            draw_confirm_exit(
                                &mut canvas,
                                nbr_cars,
                                max_speed,
                                min_speed,
                                max_timer,
                                min_timer,
                                close_calls,
                            );
                            canvas.present();
                            ask_exit = true;
                        } else {
                            let max_timer = vec_timer.iter().max().unwrap();
                            let min_timer = vec_timer.iter().min().unwrap();
                            data(
                                nbr_cars,
                                max_speed,
                                min_speed,
                                max_timer,
                                min_timer,
                                close_calls,
                            );
                            break 'running;
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(k), ..
                } => {
                    if !can_add {
                        let mut rng = rand::thread_rng();

                        let key = if k == Keycode::R {
                            let dirs = [Keycode::Up, Keycode::Down, Keycode::Left, Keycode::Right];
                            let mut rng = rand::thread_rng();
                            dirs[rng.gen_range(0..dirs.len())]
                        } else {
                            k
                        };

                        let (x, y, dir, angle) = match key {
                            Keycode::Up => {
                                (410 + (rng.gen_range(0..3) * 45), 800, Direction::Up, 0.0)
                            }
                            Keycode::Down => {
                                (275 + (rng.gen_range(0..3) * 45), 0, Direction::Down, 180.0)
                            }
                            Keycode::Left => (
                                800,
                                270 + (rng.gen_range(0..3) * 45),
                                Direction::Left,
                                -90.0,
                            ),
                            Keycode::Right => {
                                (0, 400 + (rng.gen_range(0..3) * 45), Direction::Right, 90.0)
                            }
                            _ => (0, 0, Direction::Up, 0.0),
                        };

                        if key == Keycode::Up
                            || key == Keycode::Down
                            || key == Keycode::Left
                            || key == Keycode::Right
                        {
                            let ranger = rng.gen_range(0..3) * 45;
                            let mut v = Vehicule::new(x, y, dir, angle);
                            if ranger == 0 || ranger == 90 {
                                v.turning = true;
                            }
                            rect.push_back(v);
                            can_add = true;
                        }
                    }
                }
                _ => {}
            }
        }
        if !ask_exit {
            // Logic d'update
            let mut new_cars = VecDeque::new();
            let current_state = rect.clone();
            for (i, v_mut) in rect.iter_mut().enumerate() {
                let mut can_update_car = true;
                let mut spedd_bolean = true;

                for (j, v_other) in current_state.iter().enumerate() {
                    if i != j {
                        if v_mut.collitions(v_other, SAFE_DISTANCE) {
                            spedd_bolean = false;
                        }
                        if v_mut.collitions(v_other, DISTANCE) {
                            can_update_car = false;
                            if v_mut.states {
                                close_calls += 1;
                            }
                            v_mut.states = false;
                            break;
                        }
                    }
                }

                if can_update_car {
                    if v_mut.frame_count >= 10 {
                        v_mut.speed = if spedd_bolean { 3 } else { 1 };
                        v_mut.update();
                        v_mut.frame_count = 0;
                    } else {
                        v_mut.frame_count += 1;
                    }
                }
                let out = match v_mut.direction {
                    Direction::Up => v_mut.y < -10,
                    Direction::Down => v_mut.y > 810,
                    Direction::Left => v_mut.x < -10,
                    Direction::Right => v_mut.x > 810,
                };

                if out {
                    // nbr_of_cars += 1;
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
                canvas.copy_ex(
                    &car_texture,
                    None,
                    Some(target),
                    v.angle,
                    None,
                    false,
                    false,
                )?;
            }
            canvas.present();
        }
    }
    Ok(())
}

pub fn draw_confirm_exit(
    canvas: &mut Canvas<Window>,
    nbr_cars: i32,
    max_speed: i32,
    min_speed: i32,
    max_timer: &Duration,
    min_timer: &Duration,
    close_calls: i32,
) {
   
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(Rect::new(200, 220, 400, 300)).unwrap();

    let texts = vec![
        format!("Total cars: {}", nbr_cars),
        format!("Max speed: {}", max_speed),
        format!("Min speed: {}", min_speed),
        format!("Max time: {:.2?}", max_timer),
        format!("Min time: {:.2?}", min_timer),
        format!("Close calls: {}", close_calls),
    ];

    let mut y = 220;
    for text in texts {
        let img = text_to_image(&text);
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_static(
                Some(sdl2::pixels::PixelFormatEnum::RGBA8888),
                img.width(),
                img.height(),
            )
            .unwrap();
        texture
            .update(None, &img, 4 * img.width() as usize)
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(220, y, img.width(), img.height())),
            )
            .unwrap();
        y += 50;
    }

    canvas.present();
}

pub fn text_to_image(text: &str) -> RgbaImage {
    let font_data = include_bytes!("../assets/DejaVuSans1.ttf");

    let font = match Font::try_from_bytes(font_data as &[u8]) {
        Some(f) => f,
        None => {
            eprintln!("Failed to load font from bytes!");
            return RgbaImage::new(1, 1); 
        }
    };

    let scale = Scale::uniform(20.0);
    let width = 300;
    let height = 40;
    let mut img = RgbaImage::new(width, height);

    for (i, c) in text.chars().enumerate() {
        let v_metrics = font.v_metrics(scale);
        let glyph = font.glyph(c).scaled(scale).positioned(rusttype::point(
            5.0 + i as f32 * 15.0,
            20.0 + v_metrics.ascent,
        ));
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = x + bb.min.x as u32;
                let py = y + bb.min.y as u32;
                if px < width && py < height {
                    img.put_pixel(px, py, Rgba([0, 0, 0, (v * 255.0) as u8]));
                }
            });
        }
    }

    img
}

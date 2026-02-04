use sdl2::messagebox::{show_simple_message_box, MessageBoxFlag};
use std::time::Duration;

pub fn write_stats(nb_cars: i32, collision_just: i32, collision : i32, max_speed: i32, min_speed: i32, max_timer: &Duration, min_timer: &Duration) {
    let text = format!(
        "Number of cars: {}\nClose calls: {}\nNumber of collisions {}\nCars' max velocity px/s: {}\nCars' min velocity px/s: {}\nMax time that took vehicle to pass: {:?}\nMin time that took vehicle to pass: {:?}",
        nb_cars, collision_just, collision,max_speed, min_speed, max_timer, min_timer,
    );
    // println!("stats =============\n {:?}", text);
    _ = show_simple_message_box(
        MessageBoxFlag::INFORMATION,
        "======Trafic stats======",
        &text,
        None,
    );
}
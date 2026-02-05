use std::time::Duration;

pub fn data(
    nbr_cars: i32,
    max_speed: i32,
    min_speed: i32,
    max_timer: &Duration,
    min_timer: &Duration,
    close_calls: i32,
) {
    println!("number of cars: {}",nbr_cars);
    println!("max speed: {}",max_speed);
    println!("main speed: {}",min_speed);
    println!("max timer: {:?}",max_timer);
    println!("min timer: {:?}",min_timer);
    println!("close calls: {}",close_calls);
}

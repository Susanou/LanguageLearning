// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const PRODUCTION:f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    PRODUCTION * (speed as f64) * match speed{
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let cars_per_hour = production_rate_per_hour(speed) as u32;

    (cars_per_hour / 60) 
}

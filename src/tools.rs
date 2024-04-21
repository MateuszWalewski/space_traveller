use rand::Rng;
use crate::constants; //check realtive and absolute option

pub fn draw_event() -> (&'static str,i32) {
    let number = rand::thread_rng().gen_range(0..=3);
    return constants::EVENTS[number as usize]
}

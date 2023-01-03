pub use rdev::{simulate, EventType, SimulateError};
use std::{thread, time};

pub const FILE_NAME: &str = "HotPositionL4RA.HOT";

pub fn delay(duration: u64) {
    thread::sleep(time::Duration::from_millis(duration));
}

pub fn send_input(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => {}
        Err(SimulateError) => {
            println!("Input can not sent :  {:?}", event_type);
        }
    }

    delay(10);
}

pub mod circuit_tracks;
pub mod menu;
pub mod race_modes;
pub mod sprint_tracks;
pub mod tools;
pub mod tracks;

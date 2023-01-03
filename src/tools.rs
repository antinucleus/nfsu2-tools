use super::{send_input, tracks, FILE_NAME};
use device_query::{DeviceQuery, DeviceState, Keycode};
pub use rdev::{simulate, EventType, Key, SimulateError};
use std::fs;

pub fn fast_finish(track: u32, race_mode: u32) {
    let postitions = match race_mode {
        1 => match track {
            1 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::Freemont)),
            2 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::Providencia,
            )),
            3 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::ResortLoop)),
            4 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::LowerEastside,
            )),
            5 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::CityHall)),
            6 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::Switchback)),
            7 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::PalmHighway,
            )),
            8 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::Bellavista)),
            9 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::BayviewSummit,
            )),
            10 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::Shoreside)),
            11 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::AmbassadorRidge,
            )),
            12 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::FreewayWest,
            )),
            13 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::UniversityHill,
            )),
            14 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::Observatory,
            )),
            15 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::OuterRing)),
            16 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::FreewayEast,
            )),
            17 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::MarineAnd25th,
            )),
            18 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::Dockside)),
            19 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::Boxcar)),
            20 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::Smokestack)),
            21 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::BayviewConcrete,
            )),
            22 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::GrandviewStation,
            )),
            23 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::Jackpot)),
            24 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::WoodbinePark,
            )),
            25 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::ParkDrive)),
            26 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::_12thAndArtubus,
            )),
            27 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::BayviewInternational,
            )),
            28 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::GaribaldiRun,
            )),
            29 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::BroadStreet,
            )),
            30 => tracks::get(tracks::RaceMode::Circuit(tracks::CircuitTracks::ScenicRide)),
            31 => tracks::get(tracks::RaceMode::Circuit(
                tracks::CircuitTracks::PhoenixSteel,
            )),
            _ => panic!("The entered value not found in circuit tracks!"),
        },
        2 => match track {
            1 => tracks::get(tracks::RaceMode::Sprint(
                tracks::SprintTracks::PalominoAnd16th,
            )),
            _ => panic!("The entered value not found in sprint tracks!"),
        },
        _ => panic!("The entered value not found in race modes!"),
    };

    let mut contents;
    let mut start;
    let mut end;
    let mut pos = 0;

    println!("Run NFSU2 game\n");

    loop {
        let device_state = DeviceState::new();

        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::Key1) {
            contents =
                fs::read_to_string(FILE_NAME).expect("Should have been able to read the file");

            start = contents.find(":").unwrap_or_default() + 2;
            end = contents.rfind(',').unwrap_or_default();

            if pos % 2 == 0 {
                contents.replace_range(start..end, &postitions.start[..]);
            } else {
                contents.replace_range(start..end, &postitions.end[..]);
            }

            pos += 1;

            fs::write(FILE_NAME, contents).expect("Writing error!");

            send_input(&EventType::KeyPress(Key::ControlLeft));
            send_input(&EventType::KeyRelease(Key::ControlLeft));
        }
    }
}

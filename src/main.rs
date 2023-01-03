use nfsu2_tool::{
    circuit_tracks::CircuitTracks, delay, menu, race_modes::RaceMode, sprint_tracks::SprintTracks,
    tools, FILE_NAME,
};

use std::{fs, io};

fn main() {
    if let Err(err) = fs::File::open(FILE_NAME) {
        println!(
            "HotPositionL4RA.HOT should be included in this directory!\n[Error]: {}\n",
            err
        );

        delay(3000);

        panic!();
    }

    menu::show_menu();

    let mut user_selection = String::new();

    io::stdin()
        .read_line(&mut user_selection)
        .expect("Did not read the line!");

    let selected_tool: u32 = user_selection
        .trim()
        .parse()
        .expect("The entered value is invalid!");

    match selected_tool {
        1 => {
            menu::list_race_modes();

            user_selection.clear();

            io::stdin()
                .read_line(&mut user_selection)
                .expect("Did not read the line!");

            let selected_race_mode = user_selection
                .trim()
                .parse()
                .expect("The entered value is invalid!");

            match selected_race_mode {
                1 => menu::list_tracks(RaceMode::Circuit(CircuitTracks::AmbassadorRidge)),
                2 => menu::list_tracks(RaceMode::Sprint(SprintTracks::PalominoAnd16th)),
                _ => panic!("The entered value not found in race type!"),
            }

            user_selection.clear();

            io::stdin()
                .read_line(&mut user_selection)
                .expect("Did not read the line!");

            let selected_track: u32 = user_selection
                .trim()
                .parse()
                .expect("The entered value is invalid!");

            tools::fast_finish(selected_track, selected_race_mode);
        }
        _ => panic!("The entered value not found in tools!"),
    };
}

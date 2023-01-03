use super::{circuit_tracks::CircuitTracks, race_modes::RaceMode, sprint_tracks::SprintTracks};
use strum::IntoEnumIterator;

pub fn show_menu() {
    let title = "___NFSU2 TOOL___";

    let tool_list = "\n1: Fast Finishing\n";

    println!("{title}{tool_list}");
}

pub fn list_race_modes() {
    let title = "__Select Race Mode__";

    let race_modes = "\n1: Circuit\n2: Sprint\n";

    println!("{title}{race_modes}");
}

pub fn list_tracks(race_mode: RaceMode) {
    match race_mode {
        RaceMode::Circuit(_) => {
            println!("__Select Circuit Track__");

            for (i, track) in CircuitTracks::iter().enumerate() {
                print!("{}: {:<30}", i + 1, format!("{:?}", track));

                if (i + 1) % 4 == 0 {
                    print!("\n");
                }
            }

            print!("\n\n");
        }
        RaceMode::Sprint(_) => {
            println!("__Select Sprint Track__");

            for (i, track) in SprintTracks::iter().enumerate() {
                print!("{}: {:<30}", i + 1, format!("{:?}", track));

                if (i + 1) % 4 == 0 {
                    print!("\n");
                }
            }

            print!("\n\n");
        }
    }
}

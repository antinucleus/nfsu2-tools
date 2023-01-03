use super::{circuit_tracks::CircuitTracks, sprint_tracks::SprintTracks};
pub enum RaceMode {
    Circuit(CircuitTracks),
    Sprint(SprintTracks),
}

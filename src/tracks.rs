pub use super::{circuit_tracks::CircuitTracks, race_modes::RaceMode, sprint_tracks::SprintTracks};
pub struct Positions {
    pub start: String,
    pub end: String,
}

pub fn get(race_mode: RaceMode) -> Positions {
    match race_mode {
        RaceMode::Circuit(track) => match track {
            CircuitTracks::Freemont => Positions {
                start: String::from("-890.00, -406.00"),
                end: String::from("-374.00, -303.00"),
            },
            CircuitTracks::Providencia => Positions {
                start: String::from("470.00, -408.00"),
                end: String::from("-8.00, -776.00"),
            },
            CircuitTracks::ResortLoop => Positions {
                start: String::from("-233.00, 79.00"),
                end: String::from("7.00, -564.00"),
            },
            CircuitTracks::LowerEastside => Positions {
                start: String::from("640.00, -615.00"),
                end: String::from("92.00, -516.00"),
            },
            CircuitTracks::CityHall => Positions {
                start: String::from("1063.00, 303.00"),
                end: String::from("1035.00, 640.00"),
            },
            CircuitTracks::Switchback => Positions {
                start: String::from("1616.00, 543.00"),
                end: String::from("-908.00, 672.00"),
            },
            CircuitTracks::PalmHighway => Positions {
                start: String::from("-312.00, 622.00"),
                end: String::from("1264.00, 961.00"),
            },
            CircuitTracks::Bellavista => Positions {
                start: String::from("-1545.00, 1172.00"),
                end: String::from("-2084.00, 2167.00"),
            },
            CircuitTracks::BayviewSummit => Positions {
                start: String::from("-2067.00, 1916.00"),
                end: String::from("-1524.00, 2732.00"),
            },
            CircuitTracks::Shoreside => Positions {
                start: String::from("-1029.00,  903.00"),
                end: String::from("-1159.00,  224.00"),
            },
            CircuitTracks::AmbassadorRidge => Positions {
                start: String::from("-1001.00,  905.00"),
                end: String::from("-946.00,  250.00"),
            },
            CircuitTracks::FreewayWest => Positions {
                start: String::from("-481.00, -890.00"),
                end: String::from("-8.00, -286.00"),
            },
            CircuitTracks::UniversityHill => Positions {
                start: String::from("-408.00,  254.00"),
                end: String::from("-1465.00,  527.00"),
            },
            CircuitTracks::Observatory => Positions {
                start: String::from("-1881.00, 2616.00"),
                end: String::from("-2501.00, 2503.00"),
            },
            CircuitTracks::OuterRing => Positions {
                start: String::from("-410.00,  268.00"),
                end: String::from("852.00, -933.00"),
            },
            CircuitTracks::FreewayEast => Positions {
                start: String::from("-689.00,  244.00"),
                end: String::from("604.00,-1022.00"),
            },
            CircuitTracks::MarineAnd25th => Positions {
                start: String::from("-282.00,  583.00"),
                end: String::from("-825.00, -137.00"),
            },
            CircuitTracks::Dockside => Positions {
                start: String::from("-745.00,-1949.00"),
                end: String::from("253.00,-1300.00"),
            },
            CircuitTracks::Boxcar => Positions {
                start: String::from("715.00,-1265.00"),
                end: String::from("-946.00,-1428.00"),
            },
            CircuitTracks::Smokestack => Positions {
                start: String::from("736.00,-1538.00"),
                end: String::from("-1379.00,-1763.00"),
            },

            CircuitTracks::BayviewConcrete => Positions {
                start: String::from("738.00,-1263.00"),
                end: String::from("-1082.00,-1874.00"),
            },
            CircuitTracks::GrandviewStation => Positions {
                start: String::from("788.00, -313.00"),
                end: String::from("-517.00,  -66.00"),
            },
            CircuitTracks::Jackpot => Positions {
                start: String::from("-475.00,  351.00"),
                end: String::from("14.00, -548.00"),
            },

            CircuitTracks::WoodbinePark => Positions {
                start: String::from("570.00, -343.00"),
                end: String::from("201.00, -950.00"),
            },

            CircuitTracks::ParkDrive => Positions {
                start: String::from("208.00, 1402.00"),
                end: String::from("-838.00,  866.00"),
            },

            CircuitTracks::_12thAndArtubus => Positions {
                start: String::from("-756.00, -681.00"),
                end: String::from("1064.00,-1014.00"),
            },

            CircuitTracks::BayviewInternational => Positions {
                start: String::from("1170.00,-1226.00"),
                end: String::from("-193.00,   27.00"),
            },

            CircuitTracks::GaribaldiRun => Positions {
                start: String::from("-435.00, -298.00"),
                end: String::from("688.00,-1011.00"),
            },
            CircuitTracks::BroadStreet => Positions {
                start: String::from("-436.00, -304.00"),
                end: String::from("-470.00, -669.00"),
            },
            CircuitTracks::ScenicRide => Positions {
                start: String::from("-12.00,  367.00"),
                end: String::from("302.00,   22.00"),
            },
            CircuitTracks::PhoenixSteel => Positions {
                start: String::from("-691.00,-1928.00"),
                end: String::from("-458.00,-1541.00"),
            },
        },

        RaceMode::Sprint(track) => match track {
            SprintTracks::PalominoAnd16th => Positions {
                start: String::from("-473.00, 351.00"),
                end: String::from("-473.00, 351.00"),
            },
        },
    }
}

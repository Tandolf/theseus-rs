use std::time::Duration;

use crate::algorithms::Algorithm;

#[derive(Debug, Default)]
pub struct Statistics<'a> {
    pub img: ImageStats,
    pub alg: AlgStats,
    pub maze: MazeStats,
    pub general: GeneralStats<'a>,
}

#[derive(Debug, Default)]
pub struct ImageStats {
    pub width: u32,
    pub height: u32,
    pub total: u32,
    pub load_duration: Duration,
}

#[derive(Debug, Default)]
pub struct MazeStats {
    pub total_nodes: u32,
    pub dead_ends: u32,
}

#[derive(Debug, Default)]
pub struct AlgStats {
    pub algorithm: Algorithm,
    pub decisions: u32,
    pub solution_length: u32,
    pub solution_time: Duration,
}

#[derive(Debug, Default)]
pub struct GeneralStats<'a> {
    pub filename: &'a str,
    pub time_total: Duration,
}

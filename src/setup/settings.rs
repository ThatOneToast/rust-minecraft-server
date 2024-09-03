use std::path::PathBuf;

use valence::{math::DVec3, prelude::Resource, GameMode};


#[derive(Clone, Debug)]
pub struct Settings {
    /// Number of chunks to pre-load
    ///
    /// chunks are loaded parallel of neg and pos
    /// 4 would load chunks -4..4 and 4..-4
    ///
    /// total of 16 chunks
    ///
    /// Self * Self chunks to be loaded
    pub pre_load_chunks: i32,
    /// The number of threads to use for chunk loading
    ///
    /// Default will be half of the number of available cores
    pub chunk_thread_count: Option<usize>,
    /// The path to the world directory;
    /// Path must contain a subdirectory of "region"
    ///
    /// If none is provided, the default flat world will be used.
    pub world_path: Option<PathBuf>,
    /// The max height of the world
    ///
    /// Max world height is defaulted to 384 for Non provided worlds
    ///
    /// Generated worlds will always be 384 high
    pub world_max_height: u32,
    /// The default spawn point for every player
    pub spawn_point: DVec3,
    /// The default gamemode for every player
    pub default_gamemode: GameMode,
}

impl Resource for Settings {}



use std::thread;

use setup::settings::Settings;

mod commands;
mod interacting;
mod server;
mod setup;
mod world;
use valence::{command::AddCommand, prelude::*};

fn main() {
    let settings = Settings {
        pre_load_chunks: 4,
        world_path: None,
        // world_path: Some("/Users/toast/Documents/RustWorld".into()),
        chunk_thread_count: Some(thread::available_parallelism().unwrap().get()),
        // chunk_thread_count: Some(4),
        world_max_height: 384,
        spawn_point: DVec3::new(0.0, 81.0, 0.0),
        default_gamemode: GameMode::Creative,
    };

    let mut server = server::McServer::new(settings);

    server.app
        .add_systems(Update, (
            interacting::digging, interacting::place_blocks,
            commands::teleport::handle, commands::gamemode::handle
        ))
        .add_command::<commands::teleport::Command>()
        .add_command::<commands::gamemode::Command>()
        
    ;

    server.run();
}

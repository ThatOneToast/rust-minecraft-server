use setup::settings::Settings;

mod commands;
mod interacting;
mod server;
mod setup;
mod world;
use valence::{command::AddCommand, prelude::*};

fn main() {
    let settings = Settings {
        pre_load_chunks: 50,
        // world_path: Some("/Users/toast/Documents/RustWorld".into()),
        world_path: None,
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

use valence::{
    app::App, command::AddCommand, prelude::{ConnectionMode, NetworkSettings}, DefaultPlugins
};

mod interacting;
mod commands;
mod setup;
mod world;
use valence::prelude::*;


fn display_loaded_chunk_count(mut layers: Query<&mut ChunkLayer>, mut last_count: Local<usize>) {
    let mut layer = layers.single_mut();
    let cnt = layer.chunks().count();
    if *last_count != cnt {
        *last_count = cnt;
        layer.send_action_bar_message("Chunk Count: ".into_text() + cnt.color(Color::LIGHT_PURPLE));
    }
}

fn main() {
    println!("Starting...");
    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Online {
                prevent_proxy_connections: true,
            },
            callbacks: setup::login::MyCallbacks.into(),
            ..Default::default()
        })
        

        .add_systems(Startup, setup::setup)
        .add_systems(Update, (
            
                despawn_disconnected_clients,
                
                (setup::init_clients, world::handle_chunk_loads).chain(), 
                display_loaded_chunk_count,

                interacting::digging, 
                interacting::place_blocks,
                
                
                commands::teleport::handle,
            ),
        )
        .add_plugins(DefaultPlugins) 
        // ----- COMMANDS AFTER DEFAULT PLUGINS ------
        .add_command::<commands::teleport::Command>()

        .run();
}

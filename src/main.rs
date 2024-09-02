use setup::settings::Settings;
use valence::{
    command::AddCommand, DefaultPlugins
};

mod interacting;
mod commands;
mod setup;
mod server;
mod world;
use valence::prelude::*;

fn main() {
    
    let settings = Settings {
        pre_load_chunks: 16,
        // world_path: Some("/Users/toast/Desktop/TheWorld".into()),
        world_path: None,
        world_max_height: 384,
        spawn_point: DVec3::new(0.0, 81.0, 0.0),
    };
 
        
    let mut server = server::McServer::new(settings);
    
    server.app
        .add_systems(Update, (
            interacting::digging, 
            interacting::place_blocks,
            
            
            commands::teleport::handle,
        ),
    )
    .add_plugins(DefaultPlugins) 
    // ----- COMMANDS AFTER DEFAULT PLUGINS ------
    .add_command::<commands::teleport::Command>()
    
    ;
    
    server.run();
    
    
}

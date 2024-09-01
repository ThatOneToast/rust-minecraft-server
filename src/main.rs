use valence::{
    app::App, prelude::{
        ConnectionMode, NetworkSettings
    }, DefaultPlugins
};

mod setup;
mod interacting;
use valence::prelude::*;


fn main() {
    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Online { prevent_proxy_connections: true },
            callbacks: setup::login::MyCallbacks.into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (
            setup::init_clients,
            interacting::digging,
            interacting::place_blocks
            )
        )
        .run();
    
    
}

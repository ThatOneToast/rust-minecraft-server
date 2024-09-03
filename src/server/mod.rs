use valence::{
    app::{App, AppExit, Startup, Update},
    client::despawn_disconnected_clients,
    prelude::*,
    weather::{Rain, Thunder},
    ChunkLayer,
};

use crate::{
    setup::{self, settings::Settings},
    world,
};

pub struct McServer {
    pub settings: Settings,
    pub app: App,
}

fn change_weather(mut layers: Query<(&mut Rain, &mut Thunder), With<ChunkLayer>>) {
    let level: f32 = 4.02342;

    for (mut rain, mut thunder) in &mut layers {
        rain.0 = level as f32;
        thunder.0 = level as f32;
    }
}

impl McServer {
    /// Returns a new server instance, with network settings and world resources
    ///
    /// This implementation runs a setup that puts players in creative
    /// If there is a world folder provided, will load the following world
    /// If no world is provided, a flat world will be created.
    ///
    /// Weather and Storages is also implemented here
    ///
    pub fn new(settings: Settings) -> Self {
        let mut sself = Self {
            settings,
            app: App::new(),
        };

        sself.app.insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Online {
                prevent_proxy_connections: true,
            },
            callbacks: setup::login::MyCallbacks.into(),
            ..Default::default()
        });

        sself.app.insert_resource(sself.settings.to_owned());
        sself.app.add_systems(Startup, setup::setup);

        sself.app.add_systems(
            Update,
            (
                setup::init_clients,
                change_weather,
                despawn_disconnected_clients,
            ),
        );

        if sself.settings.world_path.clone().is_some() {
            sself
                .app
                .add_systems(Update, world::handle_chunk_loads_anvil);
        } else {
            sself
                .app
                .add_systems(Update, world::handle_chunk_loads_flat);
        }

        sself
    }

    /// Runs the server
    pub fn run(&mut self) -> AppExit {
        println!("Starting...");
        self.app.run()
    }
}

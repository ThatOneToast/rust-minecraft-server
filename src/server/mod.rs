use std::collections::{hash_map::Entry, HashMap};

use flume::{Receiver, Sender};
use valence::{
    app::{App, AppExit, Startup, Update}, client::despawn_disconnected_clients, prelude::*, weather::{Rain, Thunder}, ChunkLayer
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

#[derive(Resource)]
struct GameState {
    /// Chunks that need to be generated. Chunks without a priority have already
    /// been sent to the thread pool.
    pending: HashMap<ChunkPos, Option<Priority>>,
    sender: Sender<ChunkPos>,
    receiver: Receiver<(ChunkPos, UnloadedChunk)>,
}

/// The order in which chunks should be processed by the thread pool. Smaller
/// values are sent first.
type Priority = u64;



fn update_client_views(
    mut layers: Query<&mut ChunkLayer>,
    mut clients: Query<(&mut Client, View, OldView)>,
    mut state: ResMut<GameState>,
) {
    let layer = layers.single_mut();

    for (client, view, old_view) in &mut clients {
        let view = view.get();
        let queue_pos = |pos: ChunkPos| {
            if layer.chunk(pos).is_none() {
                match state.pending.entry(pos) {
                    Entry::Occupied(mut oe) => {
                        if let Some(priority) = oe.get_mut() {
                            let dist = view.pos.distance_squared(pos);
                            *priority = (*priority).min(dist);
                        }
                    }
                    Entry::Vacant(ve) => {
                        let dist = view.pos.distance_squared(pos);
                        ve.insert(Some(dist));
                    }
                }
            }
        };

        // Queue all the new chunks in the view to be sent to the thread pool.
        if client.is_added() {
            view.iter().for_each(queue_pos);
        } else {
            let old_view = old_view.get();
            if old_view != view {
                view.diff(old_view).for_each(queue_pos);
            }
        }
    }
}

impl McServer {
    /// Returns a new server instance, with network settings and world resources
    ///
    /// This implementation runs a setup that puts players in creative
    /// If there is a world folder provided, will load the following world
    /// If no world is provided, a flat world will be created.
    ///
    /// Weather is also implemented here
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

        sself
            .app
            .add_systems(Update, (
                setup::init_clients, change_weather,
                despawn_disconnected_clients,

            ));

        if sself.settings.world_path.clone().is_some() {
            sself.app.add_systems(Update, world::handle_chunk_loads_anvil);
        } else {
            sself.app.add_systems(Update, world::handle_chunk_loads_flat);
        }
        
        

        sself
    }

    /// Runs the server
    pub fn run(&mut self) -> AppExit {
        println!("Starting...");
        self.app.run()
    }
}

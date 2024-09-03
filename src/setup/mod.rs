pub mod login;
pub mod settings;

use std::{collections::HashMap, sync::Arc, thread, time::SystemTime};

use noise::SuperSimplex;
use settings::Settings;
use valence::{
    anvil::AnvilLevel, command::{scopes::CommandScopes, CommandScopeRegistry}, log::info, op_level::OpLevel, prelude::*, spawn::IsFlat
};

use crate::world::{self, chunks::{ChunkWorkerState, GameState}};

pub fn init_clients(
    mut clients: Query<
        (
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            &mut OpLevel,
            &mut CommandScopes,
            &mut IsFlat,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, With<ChunkLayer>>,
    settings: Res<Settings>,
) {
    for (
        mut client,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        mut op_level,
        mut permissions,
        mut is_flat,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set(settings.spawn_point);
        *game_mode = settings.default_gamemode;
        op_level.set(4);
        
        if settings.world_path.clone().is_some() {
            is_flat.0 = false;
        } else {
            is_flat.0 = true;
        }

        permissions.add("admin");

        client.send_chat_message("Welcome to a Minecraft Server written in Rust!".italic());
    }
}

pub fn setup(
    mut commands: Commands,
    server: Res<Server>,
    biomes: Res<BiomeRegistry>,
    dimensions: Res<DimensionTypeRegistry>,
    mut command_scopes: ResMut<CommandScopeRegistry>,
    settings: Res<Settings>,
) {
    let current_time = std::time::SystemTime::now();

    let layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    if let Some(world_path_buf) = settings.world_path.clone() {
        let mut level = AnvilLevel::new(world_path_buf, &biomes);

        let num_chunks = settings.pre_load_chunks;

        let current_chunk_time = std::time::SystemTime::now();
        for z in -num_chunks..num_chunks {
            for x in -num_chunks..num_chunks {
                let pos = ChunkPos::new(x, z);
                level.ignored_chunks.insert(pos);
            }
        }
        let elapsed_add_ignore_chunks = current_chunk_time.elapsed().unwrap();
        info!(
            "Added {} chunks to ignored chunks in {:.2?}ms",
            num_chunks * num_chunks,
            elapsed_add_ignore_chunks.as_millis()
        );

        let ignored_chunks = &mut level.ignored_chunks.clone().into_iter();

        let current_chunk_time = std::time::SystemTime::now();
        for chunk in ignored_chunks {
            level.force_chunk_load(chunk);
        }
        let elapsed_chunk = current_chunk_time.elapsed().unwrap();

        info!(
            "Pre-loaded: {} chunks in {:.2?}ms",
            num_chunks * num_chunks,
            elapsed_chunk.as_millis()
        );

        commands.spawn((layer, level));
    } else {
        info!("Default World generation starting!");
        let seconds_per_day = 86_400;
        let seed = (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / seconds_per_day) as u32;
    
        info!("current seed: {seed}");
    
        let (finished_sender, finished_receiver) = flume::unbounded();
        let (pending_sender, pending_receiver) = flume::unbounded();
    
        let state = Arc::new(ChunkWorkerState {
            sender: finished_sender,
            receiver: pending_receiver,
            density: SuperSimplex::new(seed),
            hilly: SuperSimplex::new(seed.wrapping_add(1)),
            stone: SuperSimplex::new(seed.wrapping_add(2)),
            gravel: SuperSimplex::new(seed.wrapping_add(3)),
            grass: SuperSimplex::new(seed.wrapping_add(4)),
        });
    
        let current_time = std::time::SystemTime::now();
        for _ in 0..settings.chunk_thread_count.unwrap_or(thread::available_parallelism().unwrap().get() / 2) {
            let state = state.clone();
            thread::spawn(move || world::chunks::chunk_worker(state));
        }
    
        commands.insert_resource(GameState {
            pending: HashMap::new(),
            sender: pending_sender,
            receiver: finished_receiver,
        });
    
        commands.spawn(layer);
        let elapsed = current_time.elapsed().unwrap();
        info!("Chunk state up in {:.2?}ms", elapsed.as_millis());
    }

    command_scopes.link("admin", "command.teleport");
    command_scopes.link("admin", "command.gamemode");

    let elapsed = current_time.elapsed().unwrap();
    info!("Server up in {:.2?}ms", elapsed.as_millis());
}

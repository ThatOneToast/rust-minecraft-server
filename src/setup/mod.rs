pub mod login;
pub mod settings;

use settings::Settings;
use valence::{
    anvil::AnvilLevel,
    command::{scopes::CommandScopes, CommandScopeRegistry},
    op_level::OpLevel,
    prelude::*,
    weather::WeatherBundle,
};

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
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set(settings.spawn_point);
        *game_mode = GameMode::Creative;
        op_level.set(4);

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

    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    if let Some(world_path_buf) = settings.world_path.clone() {
        let mut level = AnvilLevel::new(world_path_buf, &biomes);

        let num_chunks = settings.pre_load_chunks;

        let current_chunk_time = std::time::SystemTime::now();
        for z in -num_chunks..num_chunks {
            for x in -num_chunks..num_chunks {
                let pos = ChunkPos::new(x, z);
                level.ignored_chunks.insert(pos);
                level.force_chunk_load(pos);
            }
        }
        let elapsed_chunk = current_chunk_time.elapsed().unwrap();
        println!(
            "Pre-loaded: {} chunks in {:.2?}ms",
            num_chunks * num_chunks,
            elapsed_chunk.as_millis()
        );

        commands.spawn((layer, level));
    } else {
        let size = 10;
        for z in -size..size {
            for x in -size..size {
                let mut chunk = UnloadedChunk::new();
                
                chunk.set_height(settings.world_max_height);
                
                for x in 0..9 { // 9x16 = new base height
                    chunk.fill_block_state_section(x, BlockState::SANDSTONE);
                }
                
                layer.chunk.insert_chunk([x, z], chunk);
            }
        }

        commands.spawn((layer, WeatherBundle::default()));
    }

    command_scopes.link("admin", "command.teleport");

    let elapsed = current_time.elapsed().unwrap();
    println!("Up in {:.2?}ms", elapsed.as_millis());
    
}

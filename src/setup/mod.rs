pub mod login;

use std::path::PathBuf;

use valence::{anvil::AnvilLevel, command::{scopes::CommandScopes, CommandScopeRegistry}, op_level::OpLevel, prelude::*};

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
        pos.set([10.5, 100.0, 10.5]);
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
) {
    let current_time = std::time::SystemTime::now();

    let layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);
    let world_path_buf: PathBuf = "/Users/toast/Desktop/World".into();
    let mut level = AnvilLevel::new(world_path_buf, &biomes);

    for z in -8..8 {
        for x in -8..8 {
            let pos = ChunkPos::new(x, z);

            level.ignored_chunks.insert(pos);
            level.force_chunk_load(pos);
        }
    }

    commands.spawn((level, layer));
    
    command_scopes.link("admin", "command.teleport");
    

    let elapsed = current_time.elapsed().unwrap();
    println!("Up in {:.2?}ms", elapsed.as_millis());
}

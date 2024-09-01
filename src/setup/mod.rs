pub mod login;

use valence::prelude::*;

pub fn init_clients(
    mut clients: Query<
        (
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut client,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([10.5, 100.0, 10.5]);
        *game_mode = GameMode::Creative;

        client.send_chat_message("Welcome to a Minecraft Server written in Rust!".italic());
    }
}

pub fn setup(
    mut commands: Commands,
    server: Res<Server>,
    biomes: Res<BiomeRegistry>,
    dimensions: Res<DimensionTypeRegistry>,
) {
    let current_time = std::time::SystemTime::now();

    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    // 100 chunks
    for z in -50..50 {
        for x in -50..50 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in 0..100 {
        for x in 0..100 {
            for y in 0..100 {
                if y == 0 || y == 1 {
                    layer.chunk.set_block([x, y, z], BlockState::BEDROCK);
                }
                if y < 60 {
                    layer.chunk.set_block([x, y, z], BlockState::STONE);
                } else if y < 99 {
                    layer.chunk.set_block([x, y, z], BlockState::DIRT);
                } else {
                    layer.chunk.set_block([x, y, z], BlockState::GRASS_BLOCK);
                }
            }
        }
    }

    commands.spawn(layer);

    let elapsed = current_time.elapsed().unwrap();
    println!("Up in {:.2?}", elapsed);
}

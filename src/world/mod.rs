use valence::{
    anvil::{AnvilLevel, ChunkLoadEvent, ChunkLoadStatus},
    message::SendMessage,
    prelude::*,
    text::{Color, IntoText},
    ChunkLayer,
};

use crate::setup::settings::Settings;

pub fn handle_chunk_loads_flat(
    mut events: EventReader<ChunkLoadEvent>,
    mut layers: Query<&mut ChunkLayer>,
    settings: Res<Settings>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        match &event.status {
            ChunkLoadStatus::Success { .. } => {}
            ChunkLoadStatus::Empty => {
                // There's no chunk here so let's insert an empty chunk. If we were doing
                // terrain generation we would prepare that here.
                let mut chunk = UnloadedChunk::new();

                chunk.set_height(settings.world_max_height);

                for x in 0..9 {
                    // 9x16 = new base height
                    chunk.fill_block_state_section(x, BlockState::SANDSTONE);
                }

                layer.insert_chunk(event.pos, chunk);
            }
            ChunkLoadStatus::Failed(e) => {
                let errmsg = format!(
                    "failed to load chunk at ({}, {}): {e:#}",
                    event.pos.x, event.pos.z
                );

                eprintln!("{errmsg}");
                layer.send_chat_message(errmsg.color(Color::RED));

                layer.insert_chunk(event.pos, UnloadedChunk::new());
            }
        }
    }
}

pub fn handle_chunk_loads_anvil(
    mut events: EventReader<ChunkLoadEvent>,
    mut layers: Query<&mut ChunkLayer, With<AnvilLevel>>,
    settings: Res<Settings>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        match &event.status {
            ChunkLoadStatus::Success { .. } => {}
            ChunkLoadStatus::Empty => {
                // There's no chunk here so let's insert an empty chunk. If we were doing
                // terrain generation we would prepare that here.
                let mut chunk = UnloadedChunk::new();

                chunk.set_height(settings.world_max_height);

                for x in 0..9 {
                    // 9x16 = new base height
                    chunk.fill_block_state_section(x, BlockState::SANDSTONE);
                }

                layer.insert_chunk(event.pos, chunk);
            }
            ChunkLoadStatus::Failed(e) => {
                let errmsg = format!(
                    "failed to load chunk at ({}, {}): {e:#}",
                    event.pos.x, event.pos.z
                );

                eprintln!("{errmsg}");
                layer.send_chat_message(errmsg.color(Color::RED));

                let mut chunk = UnloadedChunk::new();

                chunk.set_height(settings.world_max_height);

                for x in 0..8 {
                    chunk.fill_block_state_section(x, BlockState::WATER);
                }

                layer.insert_chunk(event.pos, chunk);
            }
        }
    }
}

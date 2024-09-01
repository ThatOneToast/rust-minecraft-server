use valence::{
    anvil::{AnvilLevel, ChunkLoadEvent, ChunkLoadStatus},
    message::SendMessage,
    prelude::{Chunk, EventReader, Query, UnloadedChunk, With},
    text::{Color, IntoText},
    BlockState, ChunkLayer,
};

pub fn handle_chunk_loads(
    mut events: EventReader<ChunkLoadEvent>,
    mut layers: Query<&mut ChunkLayer, With<AnvilLevel>>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        match &event.status {
            ChunkLoadStatus::Success { .. } => {
                println!("Loaded chunk at {:?}", event.pos);
            }
            ChunkLoadStatus::Empty => {
                // There's no chunk here so let's insert an empty chunk. If we were doing
                // terrain generation we would prepare that here.
                let chunk = UnloadedChunk::new();
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

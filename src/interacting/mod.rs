use valence::{
    action::{DiggingEvent, DiggingState},
    block::{BlockKind, PropName, PropValue},
    interact_block::InteractBlockEvent,
    inventory::HeldItem,
    log::debug,
    prelude::{EventReader, Inventory, Query},
    BlockState, ChunkLayer, Direction, GameMode, Hand, ItemStack,
};


pub fn digging(
    mut clients: Query<(&GameMode, &mut Inventory)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<DiggingEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((game_mode, mut inventory)) = clients.get_mut(event.client) else {
            continue;
        };

        if (*game_mode == GameMode::Creative && event.state == DiggingState::Start)
            || (*game_mode == GameMode::Survival && event.state == DiggingState::Stop)
        {
            let prev = layer.set_block(event.position, BlockState::AIR).unwrap();

            if *game_mode == GameMode::Survival {
                let broken_block_item = prev.state.to_kind().to_item_kind();

                if let Some(slot) = inventory.first_slot_with_item_in(broken_block_item, 64, 9..45)
                {
                    let count = inventory.slot(slot).count;
                    inventory.set_slot_amount(slot, count + 1);
                } else {
                    let stack = ItemStack::new(broken_block_item, 1, None);
                    if let Some(empty_slot) = inventory.first_empty_slot_in(9..45) {
                        inventory.set_slot(empty_slot, stack);
                    } else if let Some(empty_slot) = inventory.first_empty_slot() {
                        inventory.set_slot(empty_slot, stack);
                    } else {
                        debug!(
                            "No empty slot to give item to player: {:?}",
                            broken_block_item
                        );
                    }
                }
            }
        }
    }
}

pub fn place_blocks(
    mut clients: Query<(&mut Inventory, &GameMode, &HeldItem)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((mut inventory, game_mode, held)) = clients.get_mut(event.client) else {
            continue;
        };
        if event.hand != Hand::Main {
            continue;
        }

        // get the held item
        let slot_id = held.slot();
        let stack = inventory.slot(slot_id);
        if stack.is_empty() {
            continue;
        };

        let Some(block_kind) = BlockKind::from_item_kind(stack.item) else {
            continue;
        };

        if *game_mode == GameMode::Survival {
            // check if the player has the item in their inventory and remove
            // it.
            if stack.count > 1 {
                let amount = stack.count - 1;
                inventory.set_slot_amount(slot_id, amount);
            } else {
                inventory.set_slot(slot_id, ItemStack::EMPTY);
            }
        }
        let real_pos = event.position.get_in_direction(event.face);
        let state = block_kind.to_state().set(
            PropName::Axis,
            match event.face {
                Direction::Down | Direction::Up => PropValue::Y,
                Direction::North | Direction::South => PropValue::Z,
                Direction::West | Direction::East => PropValue::X,
            },
        );

        layer.set_block(real_pos, state);
    }
}

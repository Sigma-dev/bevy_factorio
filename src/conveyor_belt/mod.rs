use bevy::prelude::*;

use crate::item::{storage::{ExternalItemStorage, InternalItemStorage}, taker::ItemTaker, Item};

struct ItemProgress {
    item: Item,
    progress: f32
}

#[derive(Component, Default)]
pub struct ConveyorBelt {
    items: Vec<ItemProgress>
}

pub struct ConveyorBeltPlugin;

impl Plugin for ConveyorBeltPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_conveyors);
    }
}

fn update_conveyors(
    mut conveyor_query: Query<(&mut ConveyorBelt, &ItemTaker, &mut InternalItemStorage, &mut ExternalItemStorage)>,
    time: Res<Time>
) {
    for (mut conveyor, taker, mut internal_storage, mut external_storage) in conveyor_query.iter_mut() {
        if let Some(item) = internal_storage.try_remove_any() {
            conveyor.items.push(ItemProgress { item: item, progress: 0. });
        }

        conveyor.items.retain_mut(|item_progress| {
            item_progress.progress += 0.05;
            if item_progress.progress >= 1. {
                external_storage.add(item_progress.item.clone());
                return false;
            }
            true
        });
    }
}
use bevy::prelude::*;

use crate::{chunked_grid::world_chunked_grid::WorldChunkedGrid, item::{renderer::{ItemRender, ItemRenderer}, storage::{ExternalItemStorage, InternalItemStorage}, taker::ItemTaker, Item}, GridEntity};

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
        app
        .add_systems(FixedUpdate, update_conveyors)
        .add_systems(Update, render_items);
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
            if external_storage.get().len() == 0 && item_progress.progress >= 1. {
                external_storage.add(item_progress.item.clone());
                return false;
            }
            true
        });
    }
}

fn render_items(
    mut render_items: ResMut<ItemRenderer>,
    mut conveyor_query: Query<(&ConveyorBelt, &ExternalItemStorage, &GridEntity)>,
    world_grid: Res<WorldChunkedGrid>
) {
    for (conveyor, external_storage, grid_entity) in conveyor_query.iter() {
        for item in &conveyor.items {
            let pos_2d = world_grid.grid_to_world_pos(grid_entity.grid_position);
            let pos_3d = Vec3::new(pos_2d.x, 3., pos_2d.y);
            render_items.items.push(ItemRender { position: pos_3d, color: Color::srgb(1., 0., 0.) });
        }
    }
}
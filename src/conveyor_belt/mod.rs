use bevy::{prelude::*, render::Render};

use crate::{chunked_grid::world_chunked_grid::WorldChunkedGrid, item::{renderer::{ItemRender, ItemRenderer}, storage::{ExternalItemStorage, InternalItemStorage}, taker::{CardinalDirection, ItemTaker}, Item}, pooled_rendering::RenderCall, GridEntity};

struct ItemProgress {
    item: Item,
    progress: f32
}

#[derive(Component)]
pub struct ConveyorBelt {
    items: Vec<ItemProgress>,
    direction: CardinalDirection
}

impl ConveyorBelt {
    pub fn new(direction: CardinalDirection) -> ConveyorBelt {
        ConveyorBelt {
            items: Vec::new(),
            direction
        }
    }
}

pub struct ConveyorBeltPlugin;

impl Plugin for ConveyorBeltPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(FixedUpdate, update_conveyors)
        .add_systems(PostUpdate, render_items);
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
            if item_progress.progress < 1. {
                item_progress.progress += 1. * time.delta_seconds();
            }
            if external_storage.get().len() == 0 && item_progress.progress >= 1. {
                external_storage.add(item_progress.item.clone());
                return false;
            }
            true
        });
    }
}

fn render_items(
    mut render_writer: EventWriter<RenderCall>,
    mut render_items: ResMut<ItemRenderer>,
    mut conveyor_query: Query<(&ConveyorBelt, &ExternalItemStorage, &GridEntity)>,
    world_grid: Res<WorldChunkedGrid>
) {
    for (conveyor, external_storage, grid_entity) in conveyor_query.iter() {
        for item in &conveyor.items {
            let progress_offset = conveyor.direction.as_vec2() * (item.progress - 0.5);
            let pos_2d = world_grid.grid_to_world_pos(grid_entity.grid_position.as_vec2() + progress_offset);
            let pos_3d = Vec3::new(pos_2d.x, 3., pos_2d.y);
            render_items.items.push(ItemRender { position: pos_3d, color: Color::srgb(1., 0., 0.) });
            render_writer.send(RenderCall::new(Transform::from_translation(pos_3d), "models/items/ingot_iron/ingot_iron.glb"));
        }
    }
}
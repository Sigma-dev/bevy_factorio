use bevy::{prelude::*, render::Render};

use crate::{chunked_grid::world_chunked_grid::WorldChunkedGrid, item::{self, renderer::{ItemRender, ItemRenderer}, storage::{ExternalItemStorage, InternalItemStorage}, taker::{CardinalDirection, ItemTaker}, Item}, pooled_rendering::RenderCall, GridEntity};

#[derive(Debug)]
struct ItemProgress {
    item: Item,
    progress: f32
}

#[derive(Component)]
pub struct ConveyorBelt {
    pub speed: f32,
    pub item_spacing: f32,
    items: Vec<ItemProgress>,
    direction: CardinalDirection,
}

impl ConveyorBelt {
    pub fn new(direction: CardinalDirection, speed: f32, item_spacing: f32) -> ConveyorBelt {
        ConveyorBelt {
            items: Vec::new(),
            direction,
            speed,
            item_spacing
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
        let mut front_pos = 25.; //High number to avoid collision on the first one
        if external_storage.get().len() == 1 {
            front_pos = 1.
        }
        let spacing = conveyor.item_spacing;
        let speed = conveyor.speed;
        conveyor.items.retain_mut(|item_progress| {
            if item_progress.progress < 1. && front_pos - item_progress.progress > spacing {
                item_progress.progress += speed * time.delta_seconds();
            }
            if external_storage.get().len() == 0 && item_progress.progress >= 1. {
                external_storage.add(item_progress.item.clone());
                return false;
            }
            front_pos = item_progress.progress;
            true
        });
        let front_progress = conveyor.items.last();
        internal_storage.set_input_block(front_progress.is_some_and(|p| p.progress < conveyor.item_spacing * 1.5));
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
        if external_storage.get().len() == 1 {
            let progress_offset = conveyor.direction.as_vec2() * (1.0 - 0.5);
            let pos_2d = world_grid.grid_to_world_pos(grid_entity.grid_position.as_vec2() + progress_offset);
            let pos_3d = Vec3::new(pos_2d.x, 3., pos_2d.y);
            render_writer.send(RenderCall::new(Transform::from_translation(pos_3d), "models/items/ingot_iron/ingot_iron.glb"));
        }
    }
}
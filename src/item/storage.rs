use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashSet};

use crate::{chunked_grid::world_chunked_grid::WorldChunkedGrid, grid_shape::GridShape, GridEntity};

use super::Item;

#[derive(Component, Default)]
pub struct ExternalItemStorage {
    items: VecDeque<Item>
}

#[derive(Component, Default)]
pub struct InternalItemStorage {
    items: VecDeque<Item>
}

#[derive(Event)]
pub struct StorageUpdate {
    pub entity: Entity
}

impl ExternalItemStorage {
    pub fn new(items: Vec<Item>) -> ExternalItemStorage {
        ExternalItemStorage { items: items.into() }
    }

    pub fn try_remove_any(&mut self) -> Option<Item> {
        self.items.pop_front()
    }

    pub fn add(&mut self, item: Item) {
        self.items.push_back(item);
    }

    pub fn get(&self) -> Vec<Item> {
        return self.items.clone().into();
    }
}

impl InternalItemStorage {
    pub fn new(items: Vec<Item>) -> InternalItemStorage {
        InternalItemStorage { items: items.into() }
    }

    pub fn try_remove_any(&mut self) -> Option<Item> {
        self.items.pop_front()
    }

    pub fn add(&mut self, item: Item) {
        self.items.push_back(item);
    }

    pub fn get(&self) -> Vec<Item> {
        return self.items.clone().into();
    }
}

pub struct ItemStoragePlugin;

impl Plugin for ItemStoragePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<StorageUpdate>()
        .observe(setup_storage);
    }
}

fn setup_storage(
    trigger: Trigger<OnAdd, ExternalItemStorage>,
    shape_query: Query<&GridEntity>,
    world_grid: Res<WorldChunkedGrid>,
    mut storage_evs: EventWriter<StorageUpdate>
) {
    let Ok(grid_entity) = shape_query.get(trigger.entity()) else { return; };
    let mut neighbors = HashSet::new();
    for pos in grid_entity.shape.get_neighboring_coordinates() {
        if let Some(neighbor) = world_grid.grid.get_entity_at(pos) {
            neighbors.insert(neighbor);
        }
    }
    for neighbor in neighbors {
        storage_evs.send(StorageUpdate { entity: neighbor });
    }
    //for maybe_neighbor in grid_entity.shape.ge
}
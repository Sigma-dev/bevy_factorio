use std::f32::consts::PI;

use bevy::{ecs::{entity, intern::Internable}, prelude::*};

use crate::{chunked_grid::world_chunked_grid::WorldChunkedGrid, GridEntity};

use super::{storage::{ExternalItemStorage, InternalItemStorage, StorageUpdate }, Item};

#[derive(Component)]
pub struct ItemGenerator {
    produced_item: Item,
    production_delay: f32,
    last_production_time: Option<f32>,  
}

impl ItemGenerator {
    pub fn new(item: Item, production_delay: f32) -> ItemGenerator {
      ItemGenerator {
        produced_item: item,
        production_delay,
        last_production_time: None
      }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_generators);
}

fn update_generators(
    mut taker_query: Query<(&mut ItemGenerator, &mut ExternalItemStorage)>,
    time: Res<Time>
) {
    for (mut generator, mut storage) in taker_query.iter_mut() {
      if generator.last_production_time.is_none_or(|production_time| time.elapsed_seconds() >= production_time + generator.production_delay) {
        generator.last_production_time = Some(time.elapsed_seconds());
        storage.add(generator.produced_item.clone());
      }
    }
}
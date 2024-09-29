use bevy::{ecs::{entity, intern::Internable}, prelude::*};

use crate::{chunked_grid::world_chunked_grid::WorldChunkedGrid, GridEntity};

use super::{storage::{ExternalItemStorage, InternalItemStorage, StorageUpdate }, Item};

pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

#[derive(Component)]
pub struct ItemTaker {
    source_storage: Option<Entity>,
    source_direction: Direction,
}

impl ItemTaker {
    pub fn new(source_direction: Direction) -> ItemTaker {
        ItemTaker { source_storage: None, source_direction, }
    }
}

pub struct ItemTakerPlugin;

impl Plugin for ItemTakerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(FixedUpdate, update_takers)
        .add_systems(Update, handle_storage_updates)
        .observe(on_add);
    }
}

fn on_add(
    trigger: Trigger<OnAdd, ItemTaker>,
    mut taker_query: Query<(&mut ItemTaker, &GridEntity)>,
    world_grid: Res<WorldChunkedGrid>
) {
    setup_taker(trigger.entity(), &mut taker_query, &world_grid);
}

fn setup_taker(
    taker_entity: Entity,
    taker_query: &mut Query<(&mut ItemTaker, &GridEntity)>,
    world_grid: &Res<WorldChunkedGrid>
) {
    let Ok((mut taker, grid_entity)) = taker_query.get_mut(taker_entity) else { return; };
    let pos = grid_entity.grid_position;
    let offset = match taker.source_direction {
        Direction::Up => IVec2::new(0, 1),
        Direction::Right => IVec2::new(1, 0),
        Direction::Left => IVec2::new(-1, 0),
        Direction::Down => IVec2::new(0, -1),
    };
    let Some(storage_entity) = world_grid.grid.get_entity_at(pos + offset) else { return; };
    println!("Click");
    taker.source_storage = Some(storage_entity);
}

fn update_takers(
    mut taker_query: Query<(&mut ItemTaker, &mut InternalItemStorage)>,
    mut storage_query: Query<&mut ExternalItemStorage>
) {
    for (mut taker, mut taker_storage) in taker_query.iter_mut() {
        let Some(storage) = taker.source_storage else { continue; };
        let Ok(mut storage) = storage_query.get_mut(storage) else { continue; };
        let Some(item) = storage.try_remove_any() else { continue; };
        taker_storage.add(item);
        println!("Yoink");
    }
}

fn handle_storage_updates(
    mut taker_query: Query<(&mut ItemTaker, &GridEntity)>,
    world_grid: Res<WorldChunkedGrid>,
    mut storage_events: EventReader<StorageUpdate>
) {
    for ev in storage_events.read() {
        setup_taker(ev.entity, &mut taker_query, &world_grid);
    }
}
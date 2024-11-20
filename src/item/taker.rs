use std::f32::consts::PI;

use bevy::{ecs::{entity, intern::Internable}, prelude::*};

use crate::{chunked_grid::world_chunked_grid::WorldChunkedGrid, GridEntity};

use super::{storage::{ExternalItemStorage, InternalItemStorage, StorageUpdate }, Item};

#[derive(Clone, Copy, Debug)]
pub enum CardinalDirection {
    Up,
    Right,
    Down,
    Left,
}

impl CardinalDirection {
    pub fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }

    pub fn as_ivec2(&self) -> IVec2 {
        match self {
            CardinalDirection::Up => IVec2::new(0, 1),
            CardinalDirection::Right => IVec2::new(1, 0),
            CardinalDirection::Down => IVec2::new(0, -1),
            CardinalDirection::Left => IVec2::new(-1, 0),
        }
    }

    pub fn as_rad(&self) -> f32 {
        match self {
            CardinalDirection::Up => 0.,
            CardinalDirection::Right => PI / 2.,
            CardinalDirection::Down => PI,
            CardinalDirection::Left =>  3. * PI / 2.,
        }
    }

    pub fn rotate(&mut self) {
        *self = match self {
            CardinalDirection::Up => CardinalDirection::Right,
            CardinalDirection::Right => CardinalDirection::Down,
            CardinalDirection::Down => CardinalDirection::Left,
            CardinalDirection::Left => CardinalDirection::Up,
        }
    }

    pub fn rotate_ccw(&mut self) {
        *self = match self {
            CardinalDirection::Up => CardinalDirection::Left,
            CardinalDirection::Right => CardinalDirection::Up,
            CardinalDirection::Down => CardinalDirection::Right,
            CardinalDirection::Left => CardinalDirection::Down,
        }
    }

    pub fn flipped(&self) -> CardinalDirection {
        match self {
            CardinalDirection::Up => CardinalDirection::Down,
            CardinalDirection::Right => CardinalDirection::Left,
            CardinalDirection::Down => CardinalDirection::Up,
            CardinalDirection::Left => CardinalDirection::Right,
        }
    }
}

#[derive(Component)]
pub struct ItemTaker {
    source_storage: Option<Entity>,
    source_direction: CardinalDirection,
}

impl ItemTaker {
    pub fn new(source_direction: CardinalDirection) -> ItemTaker {
        ItemTaker { source_storage: None, source_direction }
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
    println!("Taking from {:?}", taker.source_direction);
    let offset = taker.source_direction.as_ivec2();
    let Some(storage_entity) = world_grid.grid.get_entity_at(pos + offset) else { return; };
    println!("Click");
    taker.source_storage = Some(storage_entity);
}

fn update_takers(
    mut taker_query: Query<(&mut ItemTaker, &mut InternalItemStorage)>,
    mut storage_query: Query<&mut ExternalItemStorage>
) {
    for (mut taker, mut taker_storage) in taker_query.iter_mut() {
        if !taker_storage.can_add() { continue; }
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
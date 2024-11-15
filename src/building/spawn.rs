
use std::f32::consts::PI;

use bevy::{gizmos::grid, prelude::*};

use crate::{chunked_grid::world_chunked_grid::WorldChunkedGrid, conveyor_belt::ConveyorBelt, item::{storage::{ExternalItemStorage, InternalItemStorage}, taker::{CardinalDirection, ItemTaker}, Item}, GridEntity, GridSquare};

use super::Building;

#[derive(Event)]
pub struct SpawnBuilding {
    pub building: Building,
    pub grid_position: IVec2,
    pub orientation: CardinalDirection,
}
pub struct BuildingSpawnPlugin;

impl Plugin for BuildingSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SpawnBuilding>()
        .add_systems(Update, (handle_spawns));
    }
}

pub fn handle_spawns(
    mut spawn_reader: EventReader<SpawnBuilding>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut world_grid: ResMut<WorldChunkedGrid>,
) {
    for ev in spawn_reader.read() {
        match ev.building {
            Building::ConveyorBelt => try_place_conveyor_belt(&mut commands, &assets, &mut world_grid, ev.grid_position, ev.orientation),
            Building::Factory => todo!(),
        }
    }
}

fn try_place_conveyor_belt(
    mut commands: &mut Commands,
    assets: &Res<AssetServer>,
    mut world_grid: &mut WorldChunkedGrid,
    grid_position: IVec2,
    orientation: CardinalDirection
) {
    let square = GridSquare { bl_position: grid_position - IVec2::splat(1 as i32 / 2), size: 1 };
    if !world_grid.grid.can_insert_shape(&square) { return; };
    let world_position = world_grid.grid_to_world_pos(grid_position.as_vec2());
    println!("Spawned in dir {:?}", orientation);
    let entity = commands.spawn((
        SceneBundle {
            scene: assets.load("models/buildings/conveyor_belt/conveyor_belt.glb#Scene0"),
            transform: Transform::from_xyz(world_position.x, 0.0, world_position.y).with_rotation(Quat::from_axis_angle(Vec3::Y, -orientation.as_rad() + PI)),
            ..default()
        },
        GridEntity { shape: square.clone(), grid_position },
        ExternalItemStorage::new(vec![Item { filepath: "".to_string() }]),
        InternalItemStorage::new(vec![]),
        ItemTaker::new(orientation.flipped()),
        ConveyorBelt::new(orientation)
    )).id();
    world_grid.grid.insert_shape(&square, entity);
}
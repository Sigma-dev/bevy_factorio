use bevy::{ecs::world, gizmos::grid, input::keyboard::Key, math::VectorSpace, prelude::*, render::camera::ScalingMode};
use building::BuildingSpawnPlugin;
use chunked_grid::{placer::{GridPlacerPlugin, SetPlacerBuilding}, visualizer::ChunkedTreeVisualizerPlugin, world_chunked_grid::{self, WorldChunkedGrid, WorldChunkedGridPlugin}, ChunkedGrid};
use conveyor_belt::{ConveyorBelt, ConveyorBeltPlugin};
use grid_tree::*;
use item::{renderer::ItemRendererPlugin, storage::{ExternalItemStorage, InternalItemStorage, ItemStoragePlugin}, taker::{GridDirection, ItemTaker, ItemTakerPlugin}, Item};

mod grid_tree;
mod chunked_grid;
mod grid_shape;
mod item;
mod conveyor_belt;
mod building;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((WorldChunkedGridPlugin {element_size: 0.5}, ChunkedTreeVisualizerPlugin, ConveyorBeltPlugin, ItemTakerPlugin, ItemStoragePlugin, ItemRendererPlugin, GridPlacerPlugin, BuildingSpawnPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 5., 0.).looking_at(Vec3::ZERO, -Vec3::Z),
        projection: OrthographicProjection {
            // 6 world units per window height.
            scaling_mode: ScalingMode::FixedVertical(8.),
            ..default()
        }
        .into(),
        ..default()
    });
}

fn update(
    mut building_writer: EventWriter<SetPlacerBuilding>,
    keys: Res<ButtonInput<KeyCode>>,
    mut world_chunked_grid: ResMut<WorldChunkedGrid>,
    mut gizmos: Gizmos 
) {
    if keys.just_pressed(KeyCode::KeyR) {
        println!("Get at: {:?}", world_chunked_grid.grid.get_entity_at(IVec2::new(3, 3)));
    }
    if keys.just_pressed(KeyCode::KeyT) {
        println!("Get at2: {:?}", world_chunked_grid.grid.get_entity_at(IVec2::new(-1, -1)));
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        building_writer.send(SetPlacerBuilding { building: building::Building::ConveyorBelt });
    }
}

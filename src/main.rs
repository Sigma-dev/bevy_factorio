use bevy::{ecs::world, gizmos::grid, math::VectorSpace, prelude::*, render::camera::ScalingMode};
use chunked_grid::{visualizer::ChunkedTreeVisualizerPlugin, world_chunked_grid::{self, WorldChunkedGrid, WorldChunkedGridPlugin}, ChunkedGrid};
use conveyor_belt::{ConveyorBelt, ConveyorBeltPlugin};
use grid_tree::*;
use item::{storage::{ExternalItemStorage, InternalItemStorage, ItemStoragePlugin}, taker::{ItemTaker, ItemTakerPlugin}, Item};

mod grid_tree;
mod chunked_grid;
mod grid_shape;
mod item;
mod conveyor_belt;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((WorldChunkedGridPlugin {element_size: 0.5}, ChunkedTreeVisualizerPlugin, ConveyorBeltPlugin, ItemTakerPlugin, ItemStoragePlugin))
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
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut world_chunked_grid: ResMut<WorldChunkedGrid>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    assets: Res<AssetServer>,
    mut gizmos: Gizmos 
) {
    if keys.just_pressed(KeyCode::KeyR) {
        println!("Get at: {:?}", world_chunked_grid.grid.get_entity_at(IVec2::new(3, 3)));
    }
    if keys.just_pressed(KeyCode::KeyT) {
        println!("Get at2: {:?}", world_chunked_grid.grid.get_entity_at(IVec2::new(-1, -1)));
    }
    let Some(grid_position) = world_chunked_grid.grid_mouse_position else { return; };
    if mouse_buttons.just_pressed(MouseButton::Left) {
        //try_place(&mut commands, &assets,  grid_position,&mut world_chunked_grid, "models/buildings/conveyor_belt/conveyor_belt.glb#Scene0".to_owned(), 1);
        try_place_conveyor_belt(&mut commands, &assets, grid_position, &mut world_chunked_grid);
    }
    if mouse_buttons.just_pressed(MouseButton::Right) {
        try_place(&mut commands, &assets,  grid_position,&mut world_chunked_grid, "models/buildings/factory/factory.glb#Scene0".to_owned(), 3);
    }
}

fn try_place(
    mut commands: &mut Commands,
    assets: &Res<AssetServer>,
    grid_position: IVec2,
    mut world_grid: &mut WorldChunkedGrid,
    path: String,
    size: u32,
) {
    let my_gltf = assets.load(path);
    let square = GridSquare { bl_position: grid_position - IVec2::splat(size as i32 / 2), size };
    if !world_grid.grid.can_insert_shape(&square) { return; };
    let world_position = world_grid.grid_to_world_pos(grid_position);
    let entity = commands.spawn((
        SceneBundle {
            scene: my_gltf,
            transform: Transform::from_xyz(world_position.x, 0.0, world_position.y),
            ..default()
        },
        GridEntity { shape: square.clone(), grid_position },
        ExternalItemStorage::new(vec![Item{ filepath: "".to_string() }; size as usize])
    )).id();
    world_grid.grid.try_insert_shape(&square, entity).expect("Shouldn't be a shape there!");
}

fn try_place_conveyor_belt(
    mut commands: &mut Commands,
    assets: &Res<AssetServer>,
    grid_position: IVec2,
    mut world_grid: &mut WorldChunkedGrid,
) {
    let my_gltf = assets.load("models/buildings/conveyor_belt/conveyor_belt.glb#Scene0");
    let square = GridSquare { bl_position: grid_position - IVec2::splat(1 as i32 / 2), size: 1 };
    if !world_grid.grid.can_insert_shape(&square) { return; };
    let world_position = world_grid.grid_to_world_pos(grid_position);
    let entity = commands.spawn((
        SceneBundle {
            scene: my_gltf,
            transform: Transform::from_xyz(world_position.x, 0.0, world_position.y),
            ..default()
        },
        GridEntity { shape: square.clone(), grid_position },
        ExternalItemStorage::new(vec![]),
        InternalItemStorage::new(vec![]),
        ItemTaker::new(item::taker::Direction::Down),
        ConveyorBelt::default()
    )).id();
    world_grid.grid.try_insert_shape(&square, entity).expect("Shouldn't be a shape there!");
}
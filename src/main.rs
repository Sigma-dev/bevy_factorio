use bevy::{gizmos::grid, math::VectorSpace, prelude::*, render::camera::ScalingMode};
use chunked_grid::{visualizer::ChunkedTreeVisualizerPlugin, world_chunked_grid::{self, WorldChunkedGrid, WorldChunkedGridPlugin}, ChunkedGrid};
use grid_tree::*;

mod grid_tree;
mod chunked_grid;
mod grid_shape;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((WorldChunkedGridPlugin {element_size: 0.5}, ChunkedTreeVisualizerPlugin))
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
        println!("Get at: {:?}", world_chunked_grid.grid.get_entity_at(IVec2::new(10, 3)));
    }
    if keys.just_pressed(KeyCode::KeyT) {
        println!("Get at2: {:?}", world_chunked_grid.grid.get_entity_at(IVec2::new(-1, -1)));
    }
    let Some(grid_position) = world_chunked_grid.grid_mouse_position else { return; };
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let my_gltf = assets.load("models/buildings/conveyor_belt/conveyor_belt.glb#Scene0");
        let square = GridSquare { bl_position: grid_position, size: 1};
        let world_position = world_chunked_grid.grid_to_world_pos(grid_position);
        let entity = commands.spawn((
            SceneBundle {
                scene: my_gltf,
                transform: Transform::from_xyz(world_position.x, 0.0, world_position.y),
                ..default()
            },
            GridEntity { shape: square.clone() }
        )).id();
        world_chunked_grid.grid.insert_shape(&square, entity);
    }
   
}
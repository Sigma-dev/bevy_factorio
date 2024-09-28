use bevy::{gizmos::grid, prelude::*};
use chunked_grid::{visualizer::ChunkedTreeVisualizerPlugin, world_chunked_grid::{self, WorldChunkedGrid, WorldChunkedGridPlugin}, ChunkedGrid};
use grid_tree::*;

mod grid_tree;
mod chunked_grid;
mod grid_shape;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((WorldChunkedGridPlugin {scale: 32.}, ChunkedTreeVisualizerPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}

fn update(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut world_chunked_grid: ResMut<WorldChunkedGrid>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let Some(grid_position) = world_chunked_grid.grid_mouse_position else { return; };
    let pos = IVec2::new(34, 206);
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let square = GridSquare { bl_position: grid_position, size: 1};
        let entity = commands.spawn((
            SpatialBundle::default(),
            GridEntity { shape: square.clone() }
        )).id();
        world_chunked_grid.grid.insert_shape(&square, entity);
    }
    
    println!("Chunk pos {:?} Grid pos {:?}", world_chunked_grid.grid.get_chunk_pos(grid_position), grid_position);

    if keys.just_pressed(KeyCode::KeyR) {
        println!("Get at: {:?}", world_chunked_grid.grid.get_entity_at(pos));
    }
}
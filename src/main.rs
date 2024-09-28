use bevy::{gizmos::grid, prelude::*};
use chunked_grid::{grid_mouse_position::{ChunkGridMousePosition, ChunkGridMousePositionPlugin}, visualizer::ChunkedTreeVisualizerPlugin, ChunkedGrid};
use grid_tree::*;

mod grid_tree;
mod chunked_grid;
mod grid_shape;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ChunkGridMousePositionPlugin {scale: 1.}, ChunkedTreeVisualizerPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .insert_resource(GridTreeChunk::default())
        .insert_resource(ChunkedGrid::default())
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
    mut grid: ResMut<ChunkedGrid>,
    keys: Res<ButtonInput<KeyCode>>,
    grid_mouse_position: Res<ChunkGridMousePosition>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let Some(chunk_position) = grid_mouse_position.chunk_position else { return; };
    let Some(grid_position) = grid_mouse_position.grid_position else { return; };
    let pos = IVec2::new(34, 206);
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let square = GridSquare { bl_position: grid_position, size: 1};
        let entity = commands.spawn((
            SpatialBundle::default(),
            GridEntity { shape: square.clone() }
        )).id();
        grid.insert_shape(&square, entity);
    }
    
    println!("Chunk pos {:?} Grid pos {:?}", grid.get_chunk_pos(grid_position), grid_position);

    if keys.just_pressed(KeyCode::KeyR) {
        println!("Get at: {:?}", grid.get_entity_at(pos));
    }
}

/* 
fn update(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut tree: ResMut<GridTreeChunk>,
    grid_mouse_position: Res<GridMousePosition>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let pos = IVec2::new(34, 206);
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let Some(grid_position) = grid_mouse_position.grid_position else { return; };
        let square = GridSquare { bl_position: grid_position, size: 6};
        let entity = commands.spawn((
            SpatialBundle::default(),
            GridEntity { shape: square.clone() }
        )).id();
        tree.insert(entity, &square);
    }

    if keys.just_pressed(KeyCode::KeyR) {
        println!("Get at: {:?}", tree.get_entity_at(pos));
    }
}
*/
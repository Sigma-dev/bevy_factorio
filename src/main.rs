use bevy::prelude::*;
use grid_tree::*;

mod grid_tree;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GridTreeVisualizerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .insert_resource(GridTreeChunk::default())
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
    mut tree: ResMut<GridTreeChunk>
) {
    if keys.just_pressed(KeyCode::KeyT) {
        tree.store_grid_position(IVec2::new(-105, -13));
    }

    if keys.just_pressed(KeyCode::KeyP) {
        let square = GridSquare { tl_position: IVec2::new(34, 206), size: 14};
        tree.insert(&square);
        commands.spawn((
            SpatialBundle::default(),
            GridEntity { shape: square }
        ));
    }
}
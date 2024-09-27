use bevy::prelude::*;
use grid_tree::*;

mod grid_tree;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((GridMousePositionPlugin {scale: 1.}, GridTreeVisualizerPlugin))
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
    mut tree: ResMut<GridTreeChunk>,
    grid_mouse_position: Res<GridMousePosition>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let pos = IVec2::new(34, 206);
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let square = GridSquare { bl_position: grid_mouse_position.grid_position, size: 6};
        let entity = commands.spawn((
            SpatialBundle::default(),
            GridEntity { shape: square.clone() }
        )).id();
        tree.insert(entity, &square);
    }

    if keys.just_pressed(KeyCode::KeyR) {
        println!("Get at: {:?}", tree.get_entity_at(pos));
    }

    println!("{}", grid_mouse_position.grid_position);
}
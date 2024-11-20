use building::BuildingSpawnPlugin;
use item::{generator::{self, ItemGenerator}, renderer::ItemRendererPlugin, storage::{ExternalItemStorage, InternalItemStorage, ItemStoragePlugin}, taker::{CardinalDirection, ItemTaker, ItemTakerPlugin}, Item};
use pooled_rendering::PooledRenderingPlugin;
use std::f32::consts::PI;
use bevy::{ecs::{system::EntityCommands, world}, gizmos::grid, math::VectorSpace, prelude::*, render::camera::ScalingMode};
use chunked_grid::{placer::{GridPlacerPlugin, SetPlacerBuilding}, visualizer::ChunkedTreeVisualizerPlugin, world_chunked_grid::{self, WorldChunkedGrid, WorldChunkedGridPlugin}, ChunkedGrid};
use conveyor_belt::{ConveyorBelt, ConveyorBeltPlugin};
use grid_tree::*;

mod grid_tree;
mod chunked_grid;
mod grid_shape;
mod item;
mod conveyor_belt;
mod building;
mod pooled_rendering;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((WorldChunkedGridPlugin {element_size: 0.5}, ChunkedTreeVisualizerPlugin, ConveyorBeltPlugin, ItemTakerPlugin, ItemStoragePlugin, ItemRendererPlugin, generator::plugin, GridPlacerPlugin, BuildingSpawnPlugin, PooledRenderingPlugin::new(5000)))
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
            scaling_mode: ScalingMode::FixedVertical(8.),
            ..default()
        }
        .into(),
        ..default()
    });

    commands.spawn(
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                //color: Color::srgb(0.2, 0.05, 0.1),
                illuminance: 2000.,
                ..default()
            },
            transform: Transform::looking_at(Transform::default(), -Vec3::Y + Vec3::X, Vec3::Z),
            ..default()
        }
    );
}

fn update(
    mut building_writer: EventWriter<SetPlacerBuilding>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut world_chunked_grid: ResMut<WorldChunkedGrid>,
    mut gizmos: Gizmos 
) {
    if keys.just_pressed(KeyCode::KeyY) {
        println!("Get at: {:?}", world_chunked_grid.grid.get_entity_at(IVec2::new(3, 3)));
    }
    if keys.just_pressed(KeyCode::KeyT) {
        println!("Get at2: {:?}", world_chunked_grid.grid.get_entity_at(IVec2::new(-1, -1)));
    }
    if keys.just_pressed(KeyCode::KeyR) {
        world_chunked_grid.grid_pointer_direction.rotate()
    }
    let Some(grid_position) = world_chunked_grid.grid_mouse_position else { return; };
    if keys.just_pressed(KeyCode::Digit1) {
        //try_place(&mut commands, &assets,  grid_position,&mut world_chunked_grid, "models/buildings/conveyor_belt/conveyor_belt.glb#Scene0".to_owned(), 1);
        building_writer.send(SetPlacerBuilding { building: building::Building::ConveyorBelt });
        //try_place_conveyor_belt(&mut commands, &assets, grid_position, world_chunked_grid.grid_pointer_direction, &mut world_chunked_grid);
    }
    if keys.just_pressed(KeyCode::KeyA) {
       // try_place(&mut commands, &assets,  grid_position, world_chunked_grid.grid_pointer_direction, &mut world_chunked_grid, "models/buildings/factory/factory.glb#Scene0".to_owned(), 3);
    }
    if keys.just_pressed(KeyCode::KeyG) {
      //  try_place_generator(&mut commands, &assets, grid_position, world_chunked_grid.grid_pointer_direction, &mut world_chunked_grid);
    }
}

fn try_place(
    mut commands: &mut Commands,
    assets: &Res<AssetServer>,
    grid_position: IVec2,
    direction: CardinalDirection,
    mut world_grid: &mut WorldChunkedGrid,
    path: impl Into<String>,
    size: u32,
) -> Option<Entity> {
    println!("spawn grid {:?}", grid_position);
    let my_gltf = assets.load(path.into());
    let square = GridSquare { bl_position: grid_position - IVec2::splat(size as i32 / 2), size };
    if !world_grid.grid.can_insert_shape(&square) { return None; };
    let world_position = world_grid.grid_to_world_pos(grid_position.as_vec2());
    let entity = commands.spawn((
        SceneBundle {
            scene: my_gltf,
            transform: Transform::from_xyz(world_position.x, 0.0, world_position.y).with_rotation(Quat::from_axis_angle(Vec3::Y, -direction.as_rad() + PI)),
            ..default()
        },
        GridEntity { shape: square.clone(), grid_position },
        ExternalItemStorage::new(vec![Item{ filepath: "".to_string() }; size as usize])
    )).id();
    world_grid.grid.insert_shape(&square, entity);
    return Some(entity);
}

fn try_place_generator(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    grid_position: IVec2,
    direction: CardinalDirection,
    world_grid: &mut WorldChunkedGrid,
) -> Option<Entity> {
    println!("Direction: {:?}", direction);
    let entity = try_place(commands, assets, grid_position, direction, world_grid, "models/buildings/factory/factory.glb#Scene0", 3)?;
    commands.get_entity(entity).unwrap().insert((
        ItemGenerator::new(Item{ filepath: "".to_string() }, 1.)
    ));
    Some(entity)
}

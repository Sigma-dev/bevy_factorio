use bevy::{gizmos::grid, prelude::*};

use crate::{building::{Building, SpawnBuilding}, item::taker::GridDirection};

use super::world_chunked_grid::WorldChunkedGrid;

#[derive(Component)]
pub struct GridPlacer {
    pub orientation: GridDirection,
    building: Option<Building>
}

#[derive(Event)]
pub struct SetPlacerBuilding {
    pub(crate) building: Building
}
pub struct GridPlacerPlugin;

impl Plugin for GridPlacerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SetPlacerBuilding>()
        .add_systems(Startup, spawn_placer)
        .add_systems(Update, (handle_inputs, handle_grid_placer));
    }
}

fn spawn_placer(
    mut commands: Commands,
    mut assets: Res<AssetServer>,
    mut gltf_assets: Res<Assets<Gltf>>
) {
    let mut gltf = assets.load("models/buildings/conveyor_belt/conveyor_belt.glb#Scene0");
    commands.spawn((
        SceneBundle {
            scene: gltf_assets.get(&mut gltf).unwrap().named_scenes["0"].clone(),
            ..default()
        },
        GridPlacer { orientation: GridDirection::Down, building: None }
    ));
}

fn handle_inputs(
    mut placer_query: Query<&mut GridPlacer>,
    mut spawn_writer: EventWriter<SpawnBuilding>,
    world_grid: Res<WorldChunkedGrid>,
    mut building_writer: EventReader<SetPlacerBuilding>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Some(mouse_pos) = world_grid.grid_mouse_position else { return; };
    for mut placer in placer_query.iter_mut() {
        for ev in building_writer.read() {
            placer.building = Some(ev.building.clone());
        }
        let Some(building) = placer.building.clone() else { continue; };
        if keys.just_pressed(KeyCode::KeyR) {
            placer.orientation = placer.orientation.rotate_counter_clockwise();
        }
        if mouse_buttons.just_pressed(MouseButton::Left) {
            spawn_writer.send(SpawnBuilding { building: building, grid_position: mouse_pos, orientation: placer.orientation.clone() });
        }
    }
}

fn handle_grid_placer(
    mut placer_query: Query<(&mut Transform, &mut Visibility, &GridPlacer)>,
    world_grid: Res<WorldChunkedGrid>,
    mut gizmos: Gizmos
) {
    let (mut placer_transform, mut visibility, placer) = placer_query.single_mut();
    let Some(mouse_pos) = world_grid.grid_mouse_position else { return; };
    let grid_pos = world_grid.grid_to_world_pos(mouse_pos);
    placer_transform.translation = Vec3::new(grid_pos.x, 0., grid_pos.y);
    placer_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., placer.orientation.to_radians(), 0.);
    if placer.building.is_none() {
        *visibility = Visibility::Hidden;
    } else {
        *visibility = Visibility::Visible;
    }
} 
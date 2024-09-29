use bevy::{ecs::{entity, intern::Internable}, prelude::*};

pub struct ItemRender {
    pub position: Vec3,
    pub color: Color
}

#[derive(Resource, Default)]
pub struct ItemRenderer {
    pub items: Vec<ItemRender>
}

pub struct ItemRendererPlugin;

impl Plugin for ItemRendererPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ItemRenderer::default())
        .add_systems(PostUpdate, render_items);
    }
}

fn render_items(
    mut gizmos: Gizmos,
    mut render_items: ResMut<ItemRenderer>
) {
    for render_item in &render_items.items {
        gizmos.sphere(render_item.position, Quat::IDENTITY, 0.1, render_item.color);
    }
    render_items.items.clear();
}
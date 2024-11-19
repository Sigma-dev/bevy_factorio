use bevy::{prelude::*, utils::hashbrown::HashMap};

#[derive(Resource)]
pub struct PooledRendering {
    pool_size: u32,
    handles: HashMap<String, (Handle<Mesh>, Handle<StandardMaterial>)>
}

impl PooledRendering {
    pub fn new(pool_size: u32) -> PooledRendering {
        PooledRendering { pool_size, handles: HashMap::new() }
    }

    pub fn get_or_load(&mut self, assets: &Res<AssetServer>, model_path: impl Into<String>) -> (Handle<Mesh>, Handle<StandardMaterial>) {
        let path = model_path.into();
        if let Some(handle) = self.handles.get(&path.clone()) {
            return handle.clone();
        } else {
            let mesh: Handle<Mesh> = assets.load(format!("{}#Mesh0/Primitive0", path.clone()));
            let mat: Handle<StandardMaterial> = assets.load(format!("{}#Material0", path.clone()));
            let clone = (mesh.clone(), mat.clone());
            self.handles.insert(path.into(), (mesh.clone(), mat.clone()));
            return clone;
        }
    }
}

#[derive(Component, Default)]
pub struct PooledMesh {
    model_path: Option<String>
}

#[derive(Event)]
pub struct RenderCall {
    transform: Transform,
    model_path: String,
}

impl RenderCall {
    pub fn new(transform: Transform, model_path: impl Into<String>) -> RenderCall {
        RenderCall { transform, model_path: model_path.into() }
    }
}

pub struct PooledRenderingPlugin {
    pool_size: u32
}

impl Plugin for PooledRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PooledRendering::new(self.pool_size));
        app.add_event::<RenderCall>();
        app.add_systems(Startup, init_pool);
        app.add_systems(PreUpdate, cleanup);
        app.add_systems(PostUpdate, draw_calls);
    }
}

impl PooledRenderingPlugin {
    pub fn new(pool_size: u32) -> PooledRenderingPlugin {
        PooledRenderingPlugin { pool_size }
    }
}

fn init_pool(
    pooled_rendering: Res<PooledRendering>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_mesh: Res<Assets<Mesh>>,
) {
    for i in 0..pooled_rendering.pool_size {
        commands.spawn((
            PbrBundle {
            mesh: Handle::default(),
            material: Handle::default(),
            visibility: Visibility::Hidden,
            ..Default::default()
            },
            PooledMesh::default()
        ));
    }
}

fn cleanup(
    mut pooled_meshes_q: Query<&mut Visibility, With<PooledMesh>>
) {
    for mut pooled in pooled_meshes_q.iter_mut() {
        *pooled = Visibility::Hidden;
    }
}

fn draw_calls(
    mut calls_reader: EventReader<RenderCall>,
    mut pooled_rendering: ResMut<PooledRendering>,
    mut pooled_meshes_q: Query<(Entity, &mut Visibility, &mut Transform, &mut Handle<Mesh>, &mut Handle<StandardMaterial>, &mut PooledMesh)>,
    assets: Res<AssetServer>,
    assets_mesh: Res<Assets<Mesh>>,
) {
    let mut pool: Vec<(Entity, &mut Visibility, &mut Transform, &mut Handle<Mesh>, &mut Handle<StandardMaterial>, &mut PooledMesh)> = pooled_meshes_q.iter_mut().map(|(e, a, b, c, d, f)| (e, a.into_inner(), b.into_inner(), c.into_inner(), d.into_inner(), f.into_inner())).collect();
    for call in calls_reader.read() {
        if let Some((entity, mut vis, mut transform, mut mesh, mut material, pooled_mesh)) = pool.pop() {
            let (mesh_handle, mat_handle) = pooled_rendering.get_or_load(&assets, call.model_path.clone());
            *vis = Visibility::Visible;
            *transform = call.transform;
            *mesh  = mesh_handle;
            *material = mat_handle;
        }
    }
}

/*
fn handle_loaded(
    mut commands: Commands,
    mut animated_query: Query<(Entity, &mut AnimatedGltf)>,
    assets_gltf: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>
) {
    for (animated_entity, mut animated) in animated_query.iter_mut() {
        if animated.loaded { continue; };
        if !asset_server.is_loaded_with_dependencies(animated.gltf.id()) { continue; };
        animated.loaded = true;
        let Some(gltf) = assets_gltf.get(animated.gltf.id()) else { continue; };
        commands.entity(animated_entity).with_children(|p| {
            p.spawn(SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        });
        let mut graph = AnimationGraph::new();
        for handle in &gltf.animations {
            for (name, clip) in gltf.named_animations.clone() {
                if clip.id() == handle.id() {
                    animated.animations.insert(name.to_string(),graph.add_clip(clip, 1.0, graph.root));
                }
            }
        }
        animated.graph = graphs.add(graph).clone();
    }
} */
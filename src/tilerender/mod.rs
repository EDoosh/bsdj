use bevy::prelude::*;
use std::collections::HashMap;

pub mod colors;
pub mod init_renderer;
pub mod parse_colorset;
pub mod parse_tilesprite;

pub mod cluster;
pub mod colorset;
pub mod layer_handler;
pub mod tilelayer;
pub mod tilerenderer;
pub mod tilesprite;
pub mod tr_error;

pub use cluster::*;
pub use colors::*;
pub use colorset::*;
pub use layer_handler::*;
pub use tilelayer::*;
pub use tilerenderer::*;
pub use tilesprite::*;
pub use tr_error::*;

type TileId = String;
type TileIdRef = str;

// region:      TileRenderPlugin

pub struct TileRenderPlugin;

impl Plugin for TileRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(CoreStage::First, SystemSet::new().with_system(reload_map));
        app.add_system_set_to_stage(CoreStage::PreUpdate, SystemSet::new().with_system(load_map));
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new().with_system(reload_map_properties),
        );

        app.add_plugin(init_renderer::InitRendererPlugin);
    }
}

// endregion:   TileRenderPlugin

// region:      Error

// endregion:   Error

/// Indicates this is the TileRenderer sprite
#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone, Component)]
pub struct TileRendererSprite;

/// Indicates this is a TileLayer sprite
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Component)]
pub struct TileLayerSprite {
    title: String,
}

/// A component bundle for spawning a Tile Renderer.
///
/// Requires a TileRenderer Resource to be created before use,
/// however this cannot be checked by the program so the implementor of
/// this bundle must do it.
#[derive(Bundle, Default)]
pub struct TileRendererBundle {
    /// The tile renderer component
    pub tilerenderer: TileRendererSprite,
    /// The transform of the renderer
    pub transform: Transform,
    /// The world position
    pub global_transform: GlobalTransform,
}

// region:      Systems

/// Load the maps
fn load_map(
    mut commands: Commands,
    renderer_entity: Query<Entity, With<TileRendererSprite>>,
    mut textures: ResMut<Assets<Image>>,
    mut lh: ResMut<LayerHandler>,
) {
    let tr = renderer_entity.single();
    // Clone all the items and collect into a vector so we dont get an 'immut + mut' error.
    let layer_names = lh.get_layer_names().cloned().collect::<Vec<String>>();
    let renderer = lh.get_renderer().clone();

    for layer_name in layer_names {
        let layer = lh.get_layer_mut(&layer_name).unwrap();

        if layer.should_add {
            let image = layer.as_image(&renderer);
            commands.entity(tr).with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: textures.add(image),
                        transform: Transform {
                            translation: Vec3::new(
                                layer.get_position().x,
                                layer.get_position().y,
                                layer.get_z_index(),
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(TileLayerSprite {
                        title: layer_name.clone(),
                    });
            });
        }

        layer.should_add = false;
    }
}

/// Check if a tilemap has changed, and if so, set the images on it.
fn reload_map(
    mut commands: Commands,
    maps: Query<(Entity, &Handle<Image>, &TileLayerSprite)>,
    mut textures: ResMut<Assets<Image>>,
    mut lh: ResMut<LayerHandler>,
) {
    for (entity, tex_handle, map) in maps.iter() {
        let renderer = lh.get_renderer().clone();
        let layer = lh.get_layer_mut(&map.title).unwrap();

        if layer.requires_reload {
            commands.entity(entity).despawn();

            if let Some(texture) = textures.get_mut(tex_handle) {
                *texture = layer.as_image(&renderer);
            }

            layer.requires_reload = false;
            layer.should_add = true;
        }
    }
}

/// Check if a components property's changed and update them.
fn reload_map_properties(
    mut commands: Commands,
    mut maps: Query<(&mut Transform, &TileLayerSprite)>,
    mut lh: ResMut<LayerHandler>,
) {
    for (mut transform, map) in maps.iter_mut() {
        let layer = lh.get_layer_mut(&map.title).unwrap();
        if layer.properties_changed {
            let translation = &mut transform.translation;
            translation.x = layer.get_position().x;
            translation.y = layer.get_position().y;
            translation.z = layer.get_z_index();
            layer.properties_changed = false;
        }
    }
}

// endregion:   Systems

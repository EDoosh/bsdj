use bevy::prelude::*;
use bevy::reflect;
use bevy_retrograde::core::image::{GenericImage, Rgba, RgbaImage};
use bevy_retrograde::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub mod map_renderer;
pub mod parse_tilesprite;
pub mod ui_renderer;

pub use map_renderer::MapRenderer;
pub use ui_renderer::UiRenderer;

type TileId = String;
type ColorId = String;
type ClusterId = String;
type TileIdRef = str;
type ColorIdRef = str;
type ClusterIdRef = str;
type PixelColorId = u32;

// region:      TileRenderPlugin

pub struct TileRenderPlugin;

impl Plugin for TileRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new().with_system(reload_map.system()),
        );
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new().with_system(load_map.system()),
        );

        app.add_asset::<TileRenderer>();

        app.add_plugin(map_renderer::MapRendererPlugin);
        app.add_plugin(ui_renderer::UiRendererPlugin);
    }
}

// endregion:   TileRenderPlugin

// region:      Error

/// An error that occurs when using the TileRenderer type incorrectly.
#[derive(thiserror::Error, Debug)]
pub enum TileRendererError {
    // #[error(
    //     "Invalid color set color count from ColorID {color_id}: Expected {expected}, got {recieved}"
    // )]
    // InvalidColorSetColorCount {
    //     color_id: ColorId,
    //     expected: u32,
    //     recieved: u32,
    // },
    #[error("Invalid ColorID `{0}` while trying to retrieve ColorSet.")]
    InvalidColorId(ColorId),
    #[error("Invalid TileID `{0}` while trying to retrieve TileSprite.")]
    InvalidTileId(TileId),
    #[error("Invalid PixelColorID `{0}` while trying to retrieve RGBA color from color palette.")]
    InvalidPixelColorId(PixelColorId),
    #[error(
        "TileSprite not correct size: Got TileSprite of size `{recieved}`, expected `{expected}`"
    )]
    IncorrectTileSpriteSize { recieved: usize, expected: usize },
    #[error("The Tile in the TileRenderer at position `({x}, {y})` has not yet been set.")]
    TileNotSet { x: usize, y: usize },
}

// endregion:   Error

// region:      Tile

/// A tile on the Tile Renderer.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Tile {
    /// The ID of the tile to use.
    /// Should be a valid tile used in the TileRenderer.
    id: TileId,
    /// The ID of the color to use.
    /// Should be a valid color used in the TileRenderer.
    color: ColorId,
}

impl Tile {
    pub fn new(id: &TileIdRef, color: &ColorIdRef) -> Tile {
        Tile {
            id: id.to_string(),
            color: color.to_string(),
        }
    }
}

// endregion:   Tile

// region:      TileSprite

/// A struct holding the colour information of a tile.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TileSprite {
    /// The color information of the tile.
    /// Each PixelColorId represents the color the pixel will be when drawn
    /// on the screen according to the ColorSet from the palette defined
    /// by the Tile struct this TileSprite was referenced from.
    ///
    /// e.g., Tile has a `TileId` of `z` and a `ColorId` of `warm`, where this TileSprite
    /// is TileId `z`. Assume the first 4 values of `sprite` are `1`, `5`, `6`, and `1`.
    /// ColorId `warm` defines `1` as orange, `5` as red, and `6` as yellow, so the first 4 pixels
    /// are orange, red, yellow, orange.
    sprite: Vec<PixelColorId>,
}

impl TileSprite {
    /// Construct a new TileSprite from the sprite PixelColorId.
    pub fn new(sprite: Vec<PixelColorId>) -> TileSprite {
        TileSprite { sprite }
    }

    /// Returns the number of pixels in this TileSprite
    pub fn size(&self) -> usize {
        self.sprite.len()
    }

    /// Returns the content of the TileSprite
    pub fn read(&self) -> &Vec<PixelColorId> {
        &self.sprite
    }

    /// Gets the pixel at a position
    ///
    /// # PANICS
    ///
    /// Panics if the size of the sprite is less than `idx`
    pub fn get_pixel(&self, idx: usize) -> PixelColorId {
        self.sprite[idx]
    }
}

// endregion:   TileSprite

// region:      TileSpriteHandler

/// A tile sprite handler for the TileRenderer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileSpriteHandler {
    /// The hashmap of tile sprites.
    /// Each TileSprite is mapped to a TileId, referred to by the Tile struct.
    tile_sprites: HashMap<TileId, TileSprite>,
    /// The width of each tilesprite
    width: usize,
    /// The height of each tilesprite
    height: usize,
    /// A vector of all TileIds.
    /// Should remain a vector to keep an ordered insertion of all tiles.
    tile_ids: Vec<TileId>,
}

impl TileSpriteHandler {
    /// Constructs a new TileSprite handler, where `width` and `height`
    /// are the dimensions of each TileSprite inserted into it.
    pub fn new(width: usize, height: usize) -> TileSpriteHandler {
        TileSpriteHandler {
            tile_sprites: HashMap::new(),
            width,
            height,
            tile_ids: vec![],
        }
    }

    /// Add a new color to the TileSprite hashmap, with a provided TileId
    pub fn add_tilesprite(
        &mut self,
        tileid: TileId,
        tilesprite: TileSprite,
    ) -> Result<(), TileRendererError> {
        if tilesprite.size() != self.width * self.height {
            return Err(TileRendererError::IncorrectTileSpriteSize {
                recieved: tilesprite.size(),
                expected: self.width * self.height,
            });
        }

        if !self.tile_ids.contains(&tileid) {
            self.tile_ids.push(tileid.clone());
        }
        self.tile_sprites.insert(tileid, tilesprite);

        Ok(())
    }

    pub fn get_tilesprite(&self, tileid: &TileIdRef) -> Option<&TileSprite> {
        self.tile_sprites.get(tileid)
    }
}

// endregion:   TileSpriteHandler

// region:      Cluster

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cluster {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

// endregion:   Cluster

// region:      ColorSet

/// A color palette for the TileRenderer to apply to Tiles.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ColorSet {
    /// A hashmap of colors.
    /// Each Color is mapped to a PixelColorId, used by the TileSprite struct.
    colors: HashMap<PixelColorId, Rgba<u8>>,
}

impl ColorSet {
    /// Constructs a ColorSet from an array of (PixelColorId, Rgba<u8>) tuples.
    pub fn from_tuple(tuple: &[(PixelColorId, Rgba<u8>)]) -> ColorSet {
        let mut colorset = ColorSet::default();
        for (id, color) in tuple.iter() {
            colorset.colors.insert(*id, *color);
        }

        colorset
    }

    /// Constructs a ColorSet from a <PixelColorId, Rgba<u8>> HashMap.
    pub fn from_hashmap(hashmap: HashMap<PixelColorId, Rgba<u8>>) -> ColorSet {
        ColorSet { colors: hashmap }
    }

    /// Gets the Color with the provided PixelColorId
    pub fn get_color(&self, pixel_id: &PixelColorId) -> Option<&Rgba<u8>> {
        self.colors.get(pixel_id)
    }
}

// endregion:   ColorSet

// region:      ColorHandler

/// A color palette handler for the TileRenderer.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct ColorHandler {
    /// The hashmap of color palettes.
    /// Each ColorSet is mapped to a Color ID, referred to by the Tile struct.
    colors: HashMap<ColorId, ColorSet>,
}

impl ColorHandler {
    /// Constructs a ColorHandler from an array of (ColorId, ColorSet) tuples.
    pub fn from_tuple(tuple: &[(ColorId, ColorSet)]) -> ColorHandler {
        let mut colorhandler = ColorHandler::default();
        for (id, set) in tuple {
            colorhandler.add_colorset(id.clone(), set.clone());
        }

        colorhandler
    }

    /// Constructs a ColorHandler from a <ColorId, ColorSet> HashMap.
    pub fn from_hashmap(hashmap: HashMap<ColorId, ColorSet>) -> ColorHandler {
        ColorHandler { colors: hashmap }
    }

    /// Insert a ColorSet into the handler with a designated Color ID.
    ///
    /// # NOTES
    ///
    /// Will replace any ColorSet with the same ColorId with the new one.
    pub fn add_colorset(&mut self, color_id: ColorId, colorset: ColorSet) {
        self.colors.insert(color_id, colorset);
    }

    /// Returns a ColorSet from the handler with the given ID.
    pub fn get_colorset(&self, color_id: &ColorId) -> Option<&ColorSet> {
        self.colors.get(color_id)
    }
}

// endregion:   ColorHandler

// region:      Components

// region:      TileRenderer

/// The TileRenderer component.
/// Allows rendering of Tiles to the screen in a grid-like fashion,
/// similar to older consoles like the (S)NES and Gameboy (Color).
#[derive(Clone, Debug, PartialEq, reflect::TypeUuid)]
#[uuid = "24930d95-3e87-4c29-b23c-a9e74017b74a"]
pub struct TileRenderer {
    /// The list of Tiles to render to the screen.
    /// Should be sized `width * height`, stored as `y * width + x`
    tilemap: Vec<Option<Tile>>,
    /// A map of tiles that are combined into a larger tile.
    clusters: HashMap<ClusterId, Cluster>,
    /// A struct holding available tile sprites.
    /// Each TileSprite held by it is mapped to a Tile ID, referred to by the Tile struct.
    tile_sprites: TileSpriteHandler,
    /// A struct holding available color palettes.
    /// Each ColorSet held by it is mapped to a Color ID, referred to by the Tile struct.
    colors: ColorHandler,
    /// The number of tiles wide the tilemap is to represent.
    width: usize,
    /// The number of tiles tall the tilemap is to represent.
    height: usize,
    /// Whether the rendered map has changed.
    changed: bool,
}

impl TileRenderer {
    /// Creates a new TileRenderer. `width` and `height` are the size of the map,
    /// while `tile_width` and `tile_height` are the size of the tiles.
    pub fn new(width: usize, height: usize, tile_width: usize, tile_height: usize) -> TileRenderer {
        TileRenderer {
            tilemap: vec![None; (width * height) as usize],
            clusters: HashMap::new(),
            tile_sprites: TileSpriteHandler::new(tile_width, tile_height),
            colors: ColorHandler::default(),
            width,
            height,
            changed: true,
        }
    }

    /// Sets `self.changed` to false.
    pub fn set_unchanged(&mut self) {
        self.changed = false;
    }

    /// Add a new ColorSet with a designated Color ID.
    ///
    /// # NOTES
    ///
    /// Will replace any ColorSet with the same ColorId.
    pub fn add_colorset(&mut self, color_id: &ColorIdRef, colorset: ColorSet) {
        self.colors.add_colorset(color_id.to_string(), colorset);
        self.changed = true;
    }

    /// Add a new ColorSet from a tuple arr with a designated Color ID.
    ///
    /// # NOTES
    ///
    /// Will replace any ColorSet with the same ColorId.
    pub fn add_colorset_arr(&mut self, color_id: &ColorIdRef, arr: &[(PixelColorId, Rgba<u8>)]) {
        self.colors
            .add_colorset(color_id.to_string(), ColorSet::from_tuple(arr));
        self.changed = true;
    }

    /// Gets the ColorSet with the provided Color ID.
    pub fn get_colorset(&self, color_id: &ColorIdRef) -> Result<&ColorSet, TileRendererError> {
        self.colors
            .get_colorset(&color_id.to_string())
            .ok_or_else(|| TileRendererError::InvalidColorId(color_id.to_string()))
    }

    /// Add a new TileSprite with a designated Tile ID.
    ///
    /// # NOTES
    ///
    /// Will replace any TileSprite with the same TileId.
    pub fn add_tilesprite(
        &mut self,
        tile_id: &TileIdRef,
        tilesprite: TileSprite,
    ) -> Result<(), TileRendererError> {
        self.changed = true;
        self.tile_sprites
            .add_tilesprite(tile_id.to_string(), tilesprite)
    }

    /// Add a new TileSprite from an array with a designated Tile ID.
    ///
    /// # NOTES
    ///
    /// Will replace any TileSprite with the same TileId.
    pub fn add_tilesprite_arr(
        &mut self,
        tile_id: &TileIdRef,
        arr: &[PixelColorId],
    ) -> Result<(), TileRendererError> {
        self.changed = true;
        self.add_tilesprite(tile_id, TileSprite::new(arr.to_vec()))
    }

    /// Gets the TileSprite with the provided Tile ID.
    pub fn get_tilesprite(&self, tile_id: &ColorIdRef) -> Result<&TileSprite, TileRendererError> {
        self.tile_sprites
            .get_tilesprite(&tile_id.to_string())
            .ok_or_else(|| TileRendererError::InvalidTileId(tile_id.to_string()))
    }

    /// Returns an iterable of Tile IDs
    pub fn get_all_tile_ids(&self) -> &Vec<TileId> {
        &self.tile_sprites.tile_ids
    }

    /// Adds a cluster
    pub fn add_cluster(&mut self, cluster_id: ClusterId, cluster: Cluster) {
        self.clusters.insert(cluster_id, cluster);
    }

    /// Creates and adds a cluster
    pub fn add_new_cluster(
        &mut self,
        cluster_id: &ClusterIdRef,
        width: usize,
        height: usize,
        tiles: &[Tile],
    ) {
        assert_eq!(width * height, tiles.len());
        self.add_cluster(
            cluster_id.to_string(),
            Cluster {
                width,
                height,
                tiles: tiles.to_vec(),
            },
        )
    }

    /// Returns the index of these x and y coordinates in the tilemap
    fn vec_index(&self, x: usize, y: usize) -> usize {
        y * self.width as usize + x
    }

    /// Sets a TileId with a ColorId at a position on the map.
    /// As co-ordinates are 0-indexed, `x` values are limited to
    /// `0..=self.width - 1`, and similarly with `y` values.
    ///
    /// # PANICS
    ///
    /// The function will panic if `x` or `y` are larger than or equal to
    /// `self.width` or `self.height` respectively.
    pub fn set_tile(&mut self, x: usize, y: usize, tile_id: &TileIdRef, color_id: &ColorIdRef) {
        assert!(
            x < self.width,
            "set_tile called with invalid `x` value: Got `{}`, expected `0..={}`",
            x,
            self.width - 1
        );
        assert!(
            y < self.height,
            "set_tile called with invalid `y` value: Got `{}`, expected `0..={}`",
            y,
            self.height - 1
        );

        self.changed = true;

        let index = self.vec_index(x, y);
        self.tilemap[index] = Some(Tile {
            id: tile_id.to_string(),
            color: color_id.to_string(),
        })
    }

    /// Returns the Tile at a position on the map.
    /// As co-ordinates are 0-indexed, `x` values are limited to
    /// `0..=self.width - 1`, and similarly with `y` values.
    ///
    /// # PANICS
    ///
    /// The function will panic if `x` or `y` are larger than or equal to
    /// `self.width` or `self.height` respectively.
    pub fn get_tile(&self, x: usize, y: usize) -> Result<&Tile, TileRendererError> {
        assert!(
            x < self.width,
            "get_tile called with invalid `x` value: Got `{}`, expected `0..={}`",
            x,
            self.width - 1
        );
        assert!(
            y < self.height,
            "get_tile called with invalid `y` value: Got `{}`, expected `0..={}`",
            y,
            self.height - 1
        );

        let index = self.vec_index(x, y);

        self.tilemap[index]
            .as_ref()
            .ok_or(TileRendererError::TileNotSet { x, y })
    }

    /// Sets a cluster at a point
    pub fn set_cluster(&mut self, x: isize, y: isize, cluster: &ClusterIdRef) {
        let cluster = self
            .clusters
            .get(cluster)
            .unwrap_or_else(|| panic!("set_cluster called with invalid ClusterId: `{}`", cluster))
            .clone();
        for tile_x in 0..cluster.width {
            for tile_y in 0..cluster.height {
                let tile = &cluster.tiles[tile_y * cluster.width + tile_x];
                let set_x = tile_x as isize + x;
                let set_y = tile_y as isize + y;
                if set_x < 0 || set_y < 0 {
                    continue;
                }
                self.set_tile(set_x as usize, set_y as usize, &tile.id, &tile.color);
            }
        }
    }

    /// Clears the map by setting all tiles to these values.
    pub fn clear_map(&mut self, tile_id: &TileIdRef, color_id: &ColorIdRef) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_tile(x, y, tile_id, color_id)
            }
        }
    }

    /// Returns an Image of the tilemap
    pub fn as_image(&self) -> Image {
        let map_width = self.width as u32;
        let map_height = self.height as u32;
        let tile_width = self.tile_sprites.width as u32;
        let tile_height = self.tile_sprites.height as u32;

        // Stores the image
        let mut map = RgbaImage::new(map_width * tile_width, map_height * tile_height);

        for map_x in 0..map_width {
            for map_y in 0..map_height {
                let mut tile_position = map.sub_image(
                    map_x * tile_width,
                    map_y * tile_height,
                    tile_width,
                    tile_height,
                );
                // Get the tile sprite and color palette at this index
                let tile = self.get_tile(map_x as usize, map_y as usize);

                // If tile returns an error (because its unset) then
                // it should be transparent, which the image by default is.
                // If there is a tile, actually apply that
                if let Ok(tile) = tile {
                    let tile_sprite = self.get_tilesprite(&tile.id).unwrap();
                    let color_palette = self.get_colorset(&tile.color).unwrap();
                    let tile_data = tile_sprite.read();
                    for tile_x in 0..tile_width {
                        for tile_y in 0..tile_height {
                            let pixel_idx = tile_y * tile_height + tile_x;
                            let pixel_color = &tile_data[pixel_idx as usize];
                            let color = color_palette.get_color(pixel_color).unwrap_or_else(|| {
                                panic!("{}", TileRendererError::InvalidPixelColorId(*pixel_color))
                            });
                            tile_position.put_pixel(tile_x, tile_y, *color);
                        }
                    }
                }
            }
        }

        Image::from(map)
    }
}

// endregion:   TileRenderer

/// A component struct used to indicate the map sprite.
pub struct TileRendererMap {
    renderer: Handle<TileRenderer>,
}

/// A component bundle for spawning a Tile Renderer
#[derive(Bundle)]
pub struct TileRendererBundle {
    /// The tile renderer component
    pub tilerenderer: Handle<TileRenderer>,
    /// The transform of the renderer
    pub transform: Transform,
    /// The world position
    pub global_transform: GlobalTransform,
}

// endregion:   Components

// region:      Systems

/// Load the map into the TileRenderer entity
fn load_map(
    mut commands: Commands,
    mut new_tilemaps: Query<(Entity, &mut Handle<TileRenderer>)>,
    mut image_assets: ResMut<Assets<Image>>,
    mut tr_assets: ResMut<Assets<TileRenderer>>,
) {
    for (map, tr) in new_tilemaps.iter_mut() {
        let tilerenderer = tr_assets
            .get_mut(tr.id)
            .expect("No TileRenderer asset found for entity with TileRenderer component.");
        if tilerenderer.changed {
            let image = tilerenderer.as_image();

            commands.entity(map).with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        image: image_assets.add(image),
                        sprite: Sprite {
                            centered: false,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(TileRendererMap {
                        renderer: tr.clone(),
                    });
            });

            tilerenderer.set_unchanged();
        }
    }
}

/// Check if a tilemap has changed, and if so, remove the images on it.
/// The load_map system will handle inserting the image back in.
fn reload_map(
    mut commands: Commands,
    maps: Query<(Entity, &Handle<Image>, &TileRendererMap)>,
    mut asset_server: ResMut<Assets<Image>>,
    mut tr_assets: ResMut<Assets<TileRenderer>>,
) {
    for (entity, image, map) in maps.iter() {
        let tilerenderer = tr_assets
            .get_mut(map.renderer.id)
            .expect("No TileRenderer asset found for entity with TileRendererMap component.");
        if tilerenderer.changed {
            commands.entity(entity).despawn();
            asset_server.remove(image);
        }
    }
}

// endregion:   Systems

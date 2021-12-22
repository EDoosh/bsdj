use bevy::prelude::*;
use bevy_retrograde::core::image::{GenericImage, Rgba, RgbaImage};
use bevy_retrograde::prelude::*;
use std::collections::HashMap;

pub mod colors;
pub mod init_renderer;
pub mod parse_colorset;
pub mod parse_tilesprite;

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
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new().with_system(reload_map_properties.system()),
        );

        app.add_plugin(init_renderer::InitRendererPlugin);
    }
}

// endregion:   TileRenderPlugin

// region:      Error

/// An error that occurs when using the TileRenderer type incorrectly.
#[derive(thiserror::Error, Debug)]
pub enum TileRendererError {
    #[error("Invalid ColorID `{0}` while trying to retrieve ColorSet.")]
    InvalidColorId(ColorId),
    #[error("Invalid TileID `{0}` while trying to retrieve TileSprite.")]
    InvalidTileId(TileId),
    #[error("Invalid Cluster `{0}` while trying to retrieve Cluster.")]
    InvalidClusterId(ClusterId),
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

    /// Add a new color to the TileSprite hashmap, with a provided TileId.
    /// Returns `true` if it overwrote an existing TileSprite.
    ///
    /// # ERRORS
    ///
    /// Errors if the tilesprite is not the expected size (tile_width * tile_height)
    pub fn add_tilesprite(
        &mut self,
        tileid: TileId,
        tilesprite: TileSprite,
    ) -> Result<bool, TileRendererError> {
        if tilesprite.size() != self.width * self.height {
            return Err(TileRendererError::IncorrectTileSpriteSize {
                recieved: tilesprite.size(),
                expected: self.width * self.height,
            });
        }

        if !self.tile_ids.contains(&tileid) {
            self.tile_ids.push(tileid.clone());
        }

        Ok(self.tile_sprites.insert(tileid, tilesprite).is_some())
    }

    /// Returns a TileSprite
    pub fn get_tilesprite(&self, tileid: &TileIdRef) -> Option<&TileSprite> {
        self.tile_sprites.get(tileid)
    }

    /// Checks if a TileSprite exists.
    pub fn has_tilesprite(&self, tileid: &TileIdRef) -> bool {
        self.tile_sprites.contains_key(tileid)
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
    /// Returns `true` if it overwrote an existing color.
    ///
    /// # NOTES
    ///
    /// Will replace any ColorSet with the same ColorId with the new one.
    pub fn add_colorset(&mut self, color_id: ColorId, colorset: ColorSet) -> bool {
        self.colors.insert(color_id, colorset).is_some()
    }

    /// Returns a ColorSet from the handler with the given ID.
    pub fn get_colorset(&self, color_id: &ColorIdRef) -> Option<&ColorSet> {
        self.colors.get(color_id)
    }

    /// Returns if the colorset exists.
    pub fn has_colorset(&self, color_id: &ColorIdRef) -> bool {
        self.colors.contains_key(color_id)
    }
}

// endregion:   ColorHandler

// region:      Components

// region:      LayerHandler

pub struct LayerHandler {
    renderer: TileRenderer,
    layers: HashMap<String, TileLayer>,
    pub active_font: String,
    pub active_glyph: String,
    pub active_colorset: String,
}

impl LayerHandler {
    pub fn new(renderer: TileRenderer) -> LayerHandler {
        LayerHandler {
            renderer,
            layers: HashMap::default(),
            active_font: "lowr".to_string(),
            active_glyph: "dflt".to_string(),
            active_colorset: "cute".to_string(),
        }
    }

    /// Returns a weak reference to the tile renderer
    pub fn get_renderer(&self) -> &TileRenderer {
        &self.renderer
    }

    /// Creates a new Layer.
    /// Returns `true` if it replaced an existing layer
    pub fn add_layer(&mut self, layer: TileLayer) -> bool {
        self.layers.insert(layer.title.clone(), layer).is_some()
    }

    /// Retrieves a layer given a Layer ID.
    pub fn get_layer(&self, layer_id: &str) -> Option<&TileLayer> {
        self.layers.get(layer_id)
    }

    /// Retrieves a mutable layer given a Layer ID.
    pub fn get_layer_mut(&mut self, layer_id: &str) -> Option<&mut TileLayer> {
        self.layers.get_mut(layer_id)
    }

    /// Deletes a layer.
    pub fn remove_layer(&mut self, layer_id: &str) {
        self.layers.remove(layer_id);
    }

    /// Returns an array of all the layer names.
    pub fn get_layer_names(&self) -> std::collections::hash_map::Keys<'_, String, TileLayer> {
        self.layers.keys()
    }

    /// Returns the name of a tile.
    fn get_tile_name(&self, tile_id: &TileIdRef) -> Result<String, String> {
        let fontname = format!("{}_{}", &self.active_font, tile_id);
        let has_font = self.renderer.tile_sprites.has_tilesprite(&fontname);
        if has_font {
            return Ok(fontname);
        }

        let glyphname = format!("{}_{}", &self.active_glyph, tile_id);
        let has_glyph = self.renderer.tile_sprites.has_tilesprite(&glyphname);
        if has_glyph {
            return Ok(glyphname);
        }

        Err(format!(
            "Tilesprite not found for font or glyph - Font: {} - Glyph: {}",
            fontname, glyphname
        ))
    }

    fn get_colorset_name(&self, color_id: &ColorIdRef) -> Result<String, String> {
        let colorset = format!("{}_{}", &self.active_colorset, color_id);
        let has_colorset = self.renderer.colors.has_colorset(&colorset);
        if has_colorset {
            return Ok(colorset);
        }
        Err(format!("Colorset not found - {}", colorset))
    }

    /// Clears the map by setting it all to a specific tile.
    pub fn clear_layer(
        &mut self,
        layer_id: &str,
        tile_id: &TileIdRef,
        color_id: colors::Colors,
    ) -> Result<(), String> {
        // Get the font or glyph name
        let tilename = self.get_tile_name(tile_id)?;

        // Get the colorset name
        let colorset = self.get_colorset_name(&color_id.to_string())?;

        let layer = self.get_layer_mut(layer_id).ok_or("Layer not found.")?;
        layer.clear_layer(&tilename, &colorset);

        Ok(())
    }

    /// Sets a tile at a given index, where the font or glyph and the colorset prefixes will be appended.
    pub fn set_tile(
        &mut self,
        layer_id: &str,
        x: usize,
        y: usize,
        tile_id: &TileIdRef,
        color_id: colors::Colors,
    ) -> Result<(), String> {
        if tile_id.is_empty() {
            return Ok(());
        }
        // Get the font or glyph name
        let tilename = self.get_tile_name(tile_id)?;

        // Get the colorset name
        let colorset = self.get_colorset_name(&color_id.to_string())?;

        let layer = self.get_layer_mut(layer_id).ok_or("Layer not found.")?;
        layer.set_tile(x, y, &tilename, &colorset);

        Ok(())
    }

    /// Set the tiles at the given index.
    pub fn set_tiles(
        &mut self,
        layer_id: &str,
        x: usize,
        y: usize,
        tile_ids: &[&TileIdRef],
        color_id: colors::Colors,
    ) -> Result<(), String> {
        for (idx, id) in tile_ids.iter().enumerate() {
            self.set_tile(layer_id, x + idx, y, id, color_id)?
        }
        Ok(())
    }

    /// Set the tiles at the given index.
    pub fn set_tiles_string(
        &mut self,
        layer_id: &str,
        x: usize,
        y: usize,
        tile_ids: &str,
        color_id: colors::Colors,
    ) -> Result<(), String> {
        // https://stackoverflow.com/a/47829722/12057036
        for (idx, id) in tile_ids.split_terminator("").skip(1).enumerate() {
            self.set_tile(layer_id, x + idx, y, id, color_id)?
        }
        Ok(())
    }

    /// Sets a hex number on a layer.
    pub fn set_tiles_hex(
        &mut self,
        layer_id: &str,
        x: usize,
        y: usize,
        num: usize,
        width: usize,
        color_id: colors::Colors,
    ) -> Result<(), String> {
        let hex = format!("{:0width$x}", num, width = width);
        self.set_tiles_string(layer_id, x, y, &hex, color_id)
    }
}

// endregion:   LayerHandler

// region:      TileRenderer

/// The TileRenderer component.
/// Allows rendering of Tiles to the screen in a grid-like fashion,
/// similar to older consoles like the (S)NES and Gameboy (Color).
#[derive(Clone, Debug)]
pub struct TileRenderer {
    /// A map of tiles that are combined into a larger tile.
    clusters: HashMap<ClusterId, Cluster>,
    /// A struct holding available tile sprites.
    /// Each TileSprite held by it is mapped to a Tile ID, referred to by the Tile struct.
    tile_sprites: TileSpriteHandler,
    /// A struct holding available color palettes.
    /// Each ColorSet held by it is mapped to a Color ID, referred to by the Tile struct.
    colors: ColorHandler,
    /// Whether all layers should reload
    require_reload: bool,
}

impl TileRenderer {
    /// Creates a new TileRenderer. `tile_width` and `tile_height` are the size of the tiles.
    pub fn new(tile_width: usize, tile_height: usize) -> TileRenderer {
        TileRenderer {
            clusters: HashMap::new(),
            tile_sprites: TileSpriteHandler::new(tile_width, tile_height),
            colors: ColorHandler::default(),
            require_reload: true,
        }
    }

    /// Sets a reload to be required on all layers
    pub fn require_reload(&mut self) {
        self.require_reload = true;
    }

    /// Add a new ColorSet with a designated Color ID.
    ///
    /// # NOTES
    ///
    /// Will replace any ColorSet with the same ColorId.
    pub fn add_colorset(&mut self, color_id: &ColorIdRef, colorset: ColorSet) {
        let overwrote_color = self.colors.add_colorset(color_id.to_string(), colorset);
        if overwrote_color {
            self.require_reload();
        }
    }

    /// Add a new ColorSet from a tuple arr with a designated Color ID.
    ///
    /// # NOTES
    ///
    /// Will replace any ColorSet with the same ColorId.
    pub fn add_colorset_arr(&mut self, color_id: &ColorIdRef, arr: &[(PixelColorId, Rgba<u8>)]) {
        self.add_colorset(color_id, ColorSet::from_tuple(arr));
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
        let overwrote_sprite = self
            .tile_sprites
            .add_tilesprite(tile_id.to_string(), tilesprite)?;

        if overwrote_sprite {
            self.require_reload()
        }
        Ok(())
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
        let cluster_changed = self.clusters.insert(cluster_id, cluster).is_some();

        if cluster_changed {
            self.require_reload()
        }
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

    /// Returns a cluster
    pub fn get_cluster(&self, cluster_id: &ClusterIdRef) -> Result<&Cluster, TileRendererError> {
        self.clusters
            .get(cluster_id)
            .ok_or_else(|| TileRendererError::InvalidClusterId(cluster_id.to_string()))
    }
}

// endregion:   TileRenderer

// region:      TileLayer

/// A component struct used to indicate the map sprite.
#[derive(Clone, Debug)]
pub struct TileLayer {
    /// The tiles on the map
    tilemap: Vec<Vec<Option<Tile>>>,
    /// Width of the Layer
    width: usize,
    /// Height of the Layer
    height: usize,
    /// Title of the layer
    title: String,
    /// The Z-order this layer is on
    z_order: f32,
    /// The position of this layer
    position: Vec2,
    /// Whether the TileLayer should be created (or recreated if after reload)
    should_add: bool,
    /// Whether the TileLayer requires a reload
    requires_reload: bool,
    /// Whether the TileLayer's z_order or position has changed
    properties_changed: bool,
}

impl TileLayer {
    pub fn new(title: String, width: usize, height: usize) -> Self {
        Self {
            tilemap: vec![vec![None; width]; height],
            width,
            height,
            title,
            z_order: 0.,
            position: Vec2::ZERO,
            should_add: true,
            requires_reload: true,
            properties_changed: true,
        }
    }

    /// Sets a TileId with a ColorId at a position on the layer.
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

        let cur_tile = Some(Tile {
            id: tile_id.to_string(),
            color: color_id.to_string(),
        });

        if self.tilemap[y][x] != cur_tile {
            self.tilemap[y][x] = cur_tile;
            self.requires_reload = true;
        }
    }

    /// Returns the Tile at a position on the layer.
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

        self.tilemap[y][x]
            .as_ref()
            .ok_or(TileRendererError::TileNotSet { x, y })
    }

    /// Sets a cluster at a point
    pub fn set_cluster(
        &mut self,
        renderer: &TileRenderer,
        x: isize,
        y: isize,
        cluster: &ClusterIdRef,
    ) {
        let cluster = renderer.get_cluster(cluster).unwrap();
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

    /// Clears the layer by setting all tiles to these values.
    pub fn clear_layer(&mut self, tile_id: &TileIdRef, color_id: &ColorIdRef) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_tile(x, y, tile_id, color_id)
            }
        }
    }

    /// Set the width of the layer
    pub fn set_width(&mut self, width: usize) {
        if self.get_width() == width {
            return;
        }

        for v in self.tilemap.iter_mut() {
            v.resize(width, None)
        }
        self.width = width;
        self.requires_reload = true;
    }

    /// Get the width of the layer
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Set the height of the layer
    pub fn set_height(&mut self, height: usize) {
        if self.get_height() == height {
            return;
        }

        self.tilemap.resize(height, vec![None; self.height]);
        self.height = height;
        self.requires_reload = true;
    }

    /// Get the height of the layer
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Set the z-index of the layer
    pub fn set_z_index(&mut self, z_index: f32) {
        self.z_order = z_index;
        self.properties_changed = true;
    }
    /// Gets the z-index of the layer
    pub fn get_z_index(&self) -> f32 {
        self.z_order
    }

    /// Sets the x and y positioning of the layer
    pub fn set_position(&mut self, pos: Vec2) {
        self.properties_changed = true;
        self.position = pos
    }
    /// Adds these x and y values to the position of the layer
    pub fn add_position(&mut self, pos: Vec2) {
        self.properties_changed = true;
        self.position += pos
    }
    /// Gets the x and y position of the layer
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Returns an Image of the tilemap
    pub fn as_image(&self, renderer: &TileRenderer) -> Image {
        let map_width = self.width as u32;
        let map_height = self.height as u32;
        let tile_width = renderer.tile_sprites.width as u32;
        let tile_height = renderer.tile_sprites.height as u32;

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
                    let tile_sprite = renderer.get_tilesprite(&tile.id).unwrap();
                    let color_palette = renderer.get_colorset(&tile.color).unwrap();
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

// endregion:   TileLayer

/// Indicates this is the TileRenderer sprite
#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct TileRendererSprite;

/// Indicates this is a TileLayer sprite
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
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

// endregion:   Components

// region:      Systems

/// Load the maps
fn load_map(
    mut commands: Commands,
    renderer_entity: Query<Entity, With<TileRendererSprite>>,
    mut image_assets: ResMut<Assets<Image>>,
    mut lh: ResMut<LayerHandler>,
) {
    if let Ok(tr) = renderer_entity.single() {
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
                            image: image_assets.add(image),
                            sprite: Sprite {
                                centered: false,
                                ..Default::default()
                            },
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
}

/// Check if a tilemap has changed, and if so, remove the images on it.
/// The load_map system will handle inserting the image back in.
fn reload_map(
    mut commands: Commands,
    maps: Query<(Entity, &Handle<Image>, &TileLayerSprite)>,
    mut asset_server: ResMut<Assets<Image>>,
    mut lh: ResMut<LayerHandler>,
) {
    for (entity, image, map) in maps.iter() {
        let layer = lh.get_layer_mut(&map.title).unwrap();

        if layer.requires_reload {
            commands.entity(entity).despawn();
            asset_server.remove(image);
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

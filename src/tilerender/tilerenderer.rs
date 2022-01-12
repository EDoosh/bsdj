use super::*;
use image::Rgba;
use std::collections::HashMap;

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
    requires_reload: bool,
}

impl TileRenderer {
    /// Creates a new TileRenderer. `tile_width` and `tile_height` are the size of the tiles.
    pub fn new(tile_width: usize, tile_height: usize) -> TileRenderer {
        TileRenderer {
            clusters: HashMap::new(),
            tile_sprites: TileSpriteHandler::new(tile_width, tile_height),
            colors: ColorHandler::default(),
            requires_reload: true,
        }
    }

    /// Gets whether a reload is required on all layers.
    pub fn reload_required(&self) -> bool {
        self.requires_reload
    }

    /// Unsets whether a reload is required.
    pub fn unset_require_reload(&mut self) {
        self.requires_reload = false
    }

    /// Add a new ColorSet with a designated Color ID.
    ///
    /// # NOTES
    ///
    /// Will replace any ColorSet with the same ColorId.
    pub fn add_colorset(&mut self, color_id: &ColorIdRef, colorset: ColorSet) {
        let overwrote_color = self.colors.add_colorset(color_id.to_string(), colorset);
        if overwrote_color {
            self.requires_reload = true;
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

    /// Returns true if the Color ID exists.
    pub fn has_colorset(&self, color_id: &ColorIdRef) -> bool {
        self.colors.has_colorset(color_id)
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
            self.requires_reload = true;
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
    pub fn get_tilesprite(&self, tile_id: &TileIdRef) -> Result<&TileSprite, TileRendererError> {
        self.tile_sprites
            .get_tilesprite(&tile_id.to_string())
            .ok_or_else(|| TileRendererError::InvalidTileId(tile_id.to_string()))
    }

    /// Returns an iterable of Tile IDs
    pub fn get_all_tile_ids(&self) -> &Vec<TileId> {
        self.tile_sprites.get_all_tile_ids()
    }

    /// Returns true if the Tile ID exists.
    pub fn has_tilesprite(&self, tile_id: &TileIdRef) -> bool {
        self.tile_sprites.has_tilesprite(tile_id)
    }

    /// Returns the width of a tile.
    pub fn get_tile_width(&self) -> usize {
        self.tile_sprites.get_width()
    }

    /// Returns the height of a tile.
    pub fn get_tile_height(&self) -> usize {
        self.tile_sprites.get_height()
    }

    /// Adds a cluster
    pub fn add_cluster(&mut self, cluster_id: ClusterId, cluster: Cluster) {
        let cluster_changed = self.clusters.insert(cluster_id, cluster).is_some();

        if cluster_changed {
            self.requires_reload = true;
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

    // /// Returns true if the Cluster ID exists.
    // pub fn has_cluster(&self, cluster_id: &ClusterIdRef) -> bool {
    //     self.clusters.has_cluster(tile_id)
    // }
}

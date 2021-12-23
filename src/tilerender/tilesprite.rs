use super::*;

/// A tile on the Tile Renderer.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Tile {
    /// The ID of the tile to use.
    /// Should be a valid tile used in the TileRenderer.
    pub id: TileId,
    /// The ID of the color to use.
    /// Should be a valid color used in the TileRenderer.
    pub color: ColorId,
}

impl Tile {
    pub fn new(id: &TileIdRef, color: &ColorIdRef) -> Tile {
        Tile {
            id: id.to_string(),
            color: color.to_string(),
        }
    }
}

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

    /// Returns an iterable of Tile IDs
    pub fn get_all_tile_ids(&self) -> &Vec<TileId> {
        &self.tile_ids
    }

    /// Returns the width of a tile.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Returns the height of a tile.
    pub fn get_height(&self) -> usize {
        self.height
    }
}

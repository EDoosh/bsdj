use image::Rgba;
use std::collections::HashMap;

pub type ColorId = String;
pub type ColorIdRef = str;
pub type PixelColorId = u32;

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

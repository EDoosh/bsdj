use super::*;
use std::collections::HashMap;

pub struct LayerHandler {
    renderer: TileRenderer,
    layers: HashMap<String, TileLayer>,
    /// A vec of all font names.
    pub font_names: Vec<String>,
    /// A vec of all glyph names.
    pub glyph_names: Vec<String>,
    /// A vec of all color names
    pub color_names: Vec<String>,
    pub active_font: String,
    pub active_glyph: String,
    pub active_colorset: String,
}

impl LayerHandler {
    pub fn new(renderer: TileRenderer) -> LayerHandler {
        LayerHandler {
            renderer,
            layers: HashMap::default(),
            font_names: vec![],
            glyph_names: vec![],
            color_names: vec![],
            active_font: "lowr".to_string(),
            active_glyph: "dflt".to_string(),
            active_colorset: "gray".to_string(),
        }
    }

    /// Returns a reference to the tile renderer
    pub fn get_renderer(&self) -> &TileRenderer {
        &self.renderer
    }
    /// Returns a mutable reference to the tile renderer
    pub fn get_renderer_mut(&mut self) -> &mut TileRenderer {
        &mut self.renderer
    }

    /// Creates a new Layer.
    /// Returns `true` if it replaced an existing layer
    pub fn add_layer(&mut self, layer: TileLayer) -> bool {
        self.layers
            .insert(layer.get_title().clone(), layer)
            .is_some()
    }

    /// Retrieves a layer given a Layer ID.
    pub fn get_layer(&self, layer_id: &str) -> Option<&TileLayer> {
        self.layers.get(layer_id)
    }

    /// Retrieves a mutable layer given a Layer ID.
    pub fn get_layer_mut(&mut self, layer_id: &str) -> Option<&mut TileLayer> {
        self.layers.get_mut(layer_id)
    }

    /// Returns an array of all the layer names.
    pub fn get_layer_names(&self) -> std::collections::hash_map::Keys<'_, String, TileLayer> {
        self.layers.keys()
    }

    /// Returns the name of a tile.
    fn get_tile_name(&self, tile_id: &TileIdRef) -> Result<String, String> {
        // Remap some characters to others.
        let remap = HashMap::from([(" ", "space")]);
        let tile_id = remap.get(tile_id).unwrap_or(&tile_id);

        let fontname = format!("{}_{}", &self.active_font, tile_id);
        let has_font = self.renderer.has_tilesprite(&fontname);
        if has_font {
            return Ok(fontname);
        }

        let glyphname = format!("{}_{}", &self.active_glyph, tile_id);
        let has_glyph = self.renderer.has_tilesprite(&glyphname);
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
        let has_colorset = self.renderer.has_colorset(&colorset);
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
    ///
    /// Errors if the tilename, colorset, or layer provided do not exist.
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
    pub fn set_tiles<T: AsRef<str>>(
        &mut self,
        layer_id: &str,
        x: usize,
        y: usize,
        tile_ids: &[T],
        color_id: colors::Colors,
    ) -> Result<(), String> {
        for (idx, id) in tile_ids.iter().enumerate() {
            self.set_tile(layer_id, x + idx, y, id.as_ref(), color_id)?
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

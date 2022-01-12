use crate::tilerender::*;
use image::{GenericImage, RgbaImage};
use wgpu::{Extent3d, TextureDimension, TextureFormat};

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
    pub should_add: bool,
    /// Whether the TileLayer requires a reload
    pub requires_reload: bool,
    /// Whether the TileLayer's z_order or position has changed
    pub properties_changed: bool,
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

    /// Returns the title of the layer.
    pub fn get_title(&self) -> &String {
        &self.title
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
        let tile_width = renderer.get_tile_width() as u32;
        let tile_height = renderer.get_tile_height() as u32;

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

        Image::new(
            Extent3d {
                width: map.width(),
                height: map.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            map.into_raw(),
            TextureFormat::Rgba8UnormSrgb,
        )
    }
}

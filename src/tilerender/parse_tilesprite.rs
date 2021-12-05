use super::*;
use std::fs;

/// Parses a `.tilesprite` file into a `HashMap<TileId, TileSprite>`.
///
/// # FILE STRUCTURE
///
/// A valid file structure starts with `=[sprite_prefix],[tilewidth],[tileheight]`.
/// It then has the sprite data, of the format `_[sprite_name]`, followed on the next
/// `tileheight` lines by `.[PixelColorId] [PixelColorId] [PixelColorId]...`, where
/// `[PixelColorId]` is repeated `tilewidth` times. Any lines not starting with
/// `=`, `_`, or `.` are ignored. See `assets/fonts/lower.tilesprite` for an example in action.
///
/// # PANICS
///
/// This code will panic if the file structure is not valid.
pub struct TileSpriteParser {
    contents: String,
    /// Where the TileSprites are stored, alongside their TileIds.
    /// A vector is used over a hashmap so that it can be stored in the order it was inserted.
    tilesprites: Vec<(TileId, TileSprite)>,
    title: String,
    tile_width: usize,
    tile_height: usize,
}

impl TileSpriteParser {
    pub fn parse_and_add(
        filename: &str,
        tilerenderer: &mut TileRenderer,
    ) -> Result<(), TileRendererError> {
        let tilesprites = TileSpriteParser::parse(filename);
        for (tileid, tilesprite) in tilesprites {
            tilerenderer.add_tilesprite(&tileid, tilesprite)?
        }
        Ok(())
    }

    /// Parses the `.tilesprite` format found in the assets.
    /// Returns a hashmap of TileSprites to their Tile IDs.
    pub fn parse(filename: &str) -> Vec<(TileId, TileSprite)> {
        let contents = fs::read_to_string(filename).unwrap();
        let (title, width, height) = TileSpriteParser::parse_metadata(&contents, filename);

        let mut tsp = TileSpriteParser {
            contents,
            // Temporary Tilesprite handler
            tilesprites: vec![],
            title,
            tile_width: width,
            tile_height: height,
        };

        tsp.parse_contents();

        tsp.tilesprites
    }

    /// Finds a line starting with `=` and attempts to find the
    /// title, tile width, and tile height from it.
    fn parse_metadata(contents: &str, filename: &str) -> (String, usize, usize) {
        let mut lines = contents.lines();
        let mut line = lines
            .next()
            .unwrap_or_else(|| {
                panic!(
                    "No metadata information found for TileSprite file `{}`",
                    filename
                )
            })
            .trim();

        while !line.starts_with('=') {
            line = lines
                .next()
                .unwrap_or_else(|| {
                    panic!(
                        "No metadata information found for TileSprite file `{}`",
                        filename
                    )
                })
                .trim();
        }

        let mut metadata = line[1..].split(',');
        let title = metadata.next().unwrap_or_else(|| {
            panic!("No metadata title found for TileSprite file `{}`", filename)
        });
        let width = metadata.next().unwrap_or_else(|| {
            panic!("No metadata width found for TileSprite file `{}`", filename)
        });
        let height = metadata.next().unwrap_or_else(|| {
            panic!(
                "No metadata height found for TileSprite file `{}`",
                filename
            )
        });
        (
            title.to_string(),
            width.parse::<usize>().unwrap(),
            height.parse::<usize>().unwrap(),
        )
    }

    /// Parse the contents of the file to find the TileSprite info.
    fn parse_contents(&mut self) {
        // An iterable over all the lines.
        // When this runs out, the program either panics or has finished.
        let mut line_iter = self.contents.lines();

        // The current line being processed
        let mut line = line_iter.next();

        while line.is_some() {
            let line_trimmed = line.unwrap().trim();
            // If theres a line starting with `.` here, the file
            // structure was messed up.
            if line_trimmed.starts_with('.') {
                panic!("Unexpected spritedata line: {}", line_trimmed);
            }
            if !line_trimmed.starts_with('_') {
                line = line_iter.next();
                continue;
            }

            // The TileId of this sprite.
            let id = self.title.clone() + &line_trimmed[1..];
            // Holds the PixelColorIds from the TileSprite
            let mut spritedata = vec![];
            // The number of lines with spritedata on them
            let mut data_lines = 0;
            // Iterate until the number of lines with spritedata on them
            // equals the height of a tile.
            while data_lines < self.tile_height {
                let spriteline = line_iter
                    .next()
                    .expect("End of file when expecting more spritedata.");
                let spriteline = spriteline.trim();
                // If it starts with `_`, the user is missing lines
                if spriteline.starts_with('_') {
                    panic!("Expected spritedata line, got new tile: `{}`", spriteline);
                }
                // If it doesn't hold spritedata, get the next line
                if !spriteline.starts_with('.') {
                    continue;
                }

                // Sprite data is stored with spaces between the PixelColorIds.
                // Split at whitespace and iterate over each.
                let spriteline_split = spriteline[1..].split_whitespace();
                let spriteline_count = spriteline_split.clone().count();
                if spriteline_count != self.tile_width {
                    panic!("Count of SpriteData on line not equal to width of tile. SpriteData Length: `{}`, TileWidth: `{}`, Line: `{}`", spriteline_count, self.tile_width, spriteline);
                }

                for pixel in spriteline_split {
                    spritedata.push(pixel.parse::<PixelColorId>().unwrap())
                }

                data_lines += 1;
            }

            let tilesprite = TileSprite { sprite: spritedata };
            self.tilesprites.push((id, tilesprite));

            // Ensures that it doesn't get crash due to `Unexpected spritedata line` when it next goes around
            line = line_iter.next();
        }
    }
}

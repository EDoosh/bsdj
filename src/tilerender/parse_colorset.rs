use super::*;
use bevy::prelude::Color;
use image::Rgba;
use std::fs;

const FIVE_RATIO: f64 = 255. / 31.;

/// Parses a `.colorset` file and inserts it into the TileRenderer.
///
/// # FILE STRUCTURE
///
/// A valid file structure has colorsets that start the line with `_[colorset_name]`,
/// followed on the next lines by the colorset colors.
/// Lines containing `#xxxxxx` will be assumed as an RGB hex code (or RGBA if `#xxxxxxxx`).
/// Lines containing `%xx xx xx` will be assumed as a 555 RGB code (or 5555 RGBA if `%xx xx xx xx`)
/// Lines containing `*xx xx xx` will be assumed as a 888 RGB code (or 8888 RGBA if `*xx xx xx xx`)
/// See the example file structure in `assets/colorsets/cute.colorset`
///
/// # PANICS
///
/// This code will panic if the file structure is not valid.
pub struct ColorSetParser {
    contents: String,
    colorsets: Vec<(ColorId, ColorSet)>,
    prefix: String,
}

impl ColorSetParser {
    pub fn parse_and_add(
        filename: &str,
        prefix: &str,
        tilerenderer: &mut TileRenderer,
    ) -> Result<(), TileRendererError> {
        let colorsets = ColorSetParser::parse(filename, prefix);
        for (colorsetid, colorset) in colorsets {
            tilerenderer.add_colorset(&colorsetid, colorset)
        }
        Ok(())
    }

    /// Parses the `.colorset` format found in the assets.
    /// Returns a hashmap of ColorSets to their ColorSet IDs.
    pub fn parse(filename: &str, prefix: &str) -> Vec<(ColorId, ColorSet)> {
        let contents = fs::read_to_string(filename).unwrap();

        let mut tsp = ColorSetParser {
            contents,
            colorsets: vec![],
            prefix: prefix.to_string(),
        };

        tsp.parse_contents();

        tsp.colorsets
    }

    /// Parse the contents of the file
    pub fn parse_contents(&mut self) {
        // An iterable over all the lines.
        // When this runs out, the program either panics or has finished.
        let mut line_iter = self.contents.lines();

        // The current line being processed
        let mut line = line_iter.next();

        while line.is_some() {
            let line_trimmed = line.unwrap().trim();

            // Keep iterating until a colorset title line was found
            if !line_trimmed.starts_with('_') {
                line = line_iter.next();
                continue;
            }

            // The TileId of this sprite.
            let id = self.prefix.clone() + "_" + &line_trimmed[1..];
            // Holds the colors from the colorset
            let mut colors = HashMap::new();

            let mut colorset_line = line_iter.next();
            let mut colorset_line_trim = colorset_line
                .expect("End of file when expecting more color data.")
                .trim();

            let mut color_id = 0;
            // Iterate adding colors until a colorset title line is found
            // (or end of file)
            while !colorset_line_trim.starts_with('_') {
                let rgba = match colorset_line_trim.get(0..=0) {
                    Some("#") => parse_hex(colorset_line_trim.get(1..).unwrap().as_bytes()),
                    Some("%") => parse_555(colorset_line_trim.get(1..).unwrap()),
                    Some("*") => parse_888(colorset_line_trim.get(1..).unwrap()),
                    _ => {
                        colorset_line = line_iter.next();
                        if colorset_line.is_none() {
                            break;
                        }
                        colorset_line_trim = colorset_line.unwrap().trim();
                        continue;
                    }
                };

                colors.insert(color_id, rgba);
                color_id += 1;
                // if end of file, exit
                colorset_line = line_iter.next();
                if colorset_line.is_none() {
                    break;
                }
                colorset_line_trim = colorset_line.unwrap().trim();
            }

            let colorset = ColorSet::from_hashmap(colors);
            self.colorsets.push((id, colorset));

            line = colorset_line;
        }
    }
}

/// Parses a hex color into an RGBA struct.
pub fn parse_hex(hex: &[u8]) -> Rgba<u8> {
    Rgba(match hex.len() {
        8 => [
            from_hex(hex[0]) * 16 + from_hex(hex[1]),
            from_hex(hex[2]) * 16 + from_hex(hex[3]),
            from_hex(hex[4]) * 16 + from_hex(hex[5]),
            from_hex(hex[6]) * 16 + from_hex(hex[7]),
        ],
        6 => [
            from_hex(hex[0]) * 16 + from_hex(hex[1]),
            from_hex(hex[2]) * 16 + from_hex(hex[3]),
            from_hex(hex[4]) * 16 + from_hex(hex[5]),
            255,
        ],
        4 => [
            from_hex(hex[0]) * 17,
            from_hex(hex[1]) * 17,
            from_hex(hex[2]) * 17,
            from_hex(hex[3]) * 17,
        ],
        3 => [
            from_hex(hex[0]) * 17,
            from_hex(hex[1]) * 17,
            from_hex(hex[2]) * 17,
            255,
        ],
        _ => panic!(
            "Unexpected length of hex string: Expected 3, 4, 6, or 8, got {}",
            hex.len()
        ),
    })
}

/// Parses a single character from hex.
fn from_hex(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("Invalid hex character: {}", c),
    }
}

/// Parses a 555(5) RGB(A) color into an RGBA struct.
pub fn parse_555(s: &str) -> Rgba<u8> {
    let mut split = s.split_ascii_whitespace();

    let r: f64 = split
        .next()
        .expect("Expected Red value for 555 color.")
        .parse()
        .expect("Invalid Red value for 555 color.");
    let g: f64 = split
        .next()
        .expect("Expected Green value for 555 color.")
        .parse()
        .expect("Invalid Green value for 555 color.");
    let b: f64 = split
        .next()
        .expect("Expected Blue value for 555 color.")
        .parse()
        .expect("Invalid Blue value for 555 color.");
    let a: f64 = split
        .next()
        .unwrap_or("31")
        .parse()
        .expect("Invalid Alpha value for 555 color.");

    if split.next().is_some() {
        panic!("Unexpected extra value(s) in 555 color.");
    }

    Rgba([
        (r * FIVE_RATIO) as u8,
        (g * FIVE_RATIO) as u8,
        (b * FIVE_RATIO) as u8,
        (a * FIVE_RATIO) as u8,
    ])
}

/// Parses an standard RGB(A) color into an RGBA struct.
pub fn parse_888(s: &str) -> Rgba<u8> {
    let mut split = s.split_ascii_whitespace();

    let r = split
        .next()
        .expect("Expected Red value for 888 color.")
        .parse()
        .expect("Invalid Red value for 888 color.");
    let g = split
        .next()
        .expect("Expected Green value for 888 color.")
        .parse()
        .expect("Invalid Green value for 888 color.");
    let b = split
        .next()
        .expect("Expected Blue value for 888 color.")
        .parse()
        .expect("Invalid Blue value for 888 color.");
    let a = split
        .next()
        .unwrap_or("255")
        .parse()
        .expect("Invalid Alpha value for 888 color.");

    if split.next().is_some() {
        panic!("Unexpected extra value(s) in 888 color.");
    }

    Rgba([r, g, b, a])
}

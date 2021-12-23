use super::*;

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

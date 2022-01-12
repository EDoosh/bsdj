pub trait Position {
    const MIN_X: isize;
    const MAX_X: isize;
    const MIN_Y: isize;
    const MAX_Y: isize;

    fn clamp_x(&mut self, x: isize) -> isize {
        x.clamp(Self::MIN_X, Self::MAX_X)
    }

    fn clamp_y(&mut self, y: isize) -> isize {
        y.clamp(Self::MIN_Y, Self::MAX_Y)
    }
}

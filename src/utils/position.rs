pub trait Position {
    const MIN_X: isize;
    const MAX_X: isize;
    const MIN_Y: isize;
    const MAX_Y: isize;

    fn clamp_x(&mut self, x: isize) -> isize {
        std::cmp::max(Self::MIN_X, std::cmp::min(Self::MAX_X, x))
    }

    fn clamp_y(&mut self, y: isize) -> isize {
        std::cmp::max(Self::MIN_Y, std::cmp::min(Self::MAX_Y, y))
    }
}

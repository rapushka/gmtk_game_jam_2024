 trait NumberExtensions {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl NumberExtensions for u8 {
    fn clamp(self, min: Self, max: Self) -> Self {
        match self {
            n if n < min => min,
            n if n > max => max,
            _ => self
        }
    }
}
impl NumberExtensions for i16 {
    fn clamp(self, min: Self, max: Self) -> Self {
        match self {
            n if n < min => min,
            n if n > max => max,
            _ => self
        }
    }
}
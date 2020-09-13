#[derive(Debug, Default, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Position {
    fn from(position: (f32, f32)) -> Self {
        Self {
            x: position.0,
            y: position.1,
        }
    }
}

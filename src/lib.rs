#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RgbaColor {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_luminance(&self) -> f32 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    pub m11: f32, pub m12: f32,
    pub m21: f32, pub m22: f32,
    pub dx: f32,  pub dy: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 0.0, dy: 0.0 }
    }
}

impl Transform2D {
    pub fn translate(dx: f32, dy: f32) -> Self {
        Self { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx, dy }
    }

    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        (self.m11 * x + self.m21 * y + self.dx, self.m12 * x + self.m22 * y + self.dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_and_color() {
        // Arrange
        let color = RgbaColor::new(1.0, 1.0, 1.0, 1.0);
        let t = Transform2D::translate(10.0, 20.0);
        // Act
        let lum = color.to_luminance();
        let (nx, ny) = t.apply(5.0, 5.0);
        // Assert
        assert!((lum - 1.0).abs() < 1e-4);
        assert_eq!((nx, ny), (15.0, 25.0));
    }
}

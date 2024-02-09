#[derive(Debug)]
pub struct GridConfig {
    width: usize,
    height: usize,
}

impl GridConfig {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        if width.checked_mul(height).is_none() {
            return Err("The value (width * height) should not exceed usize::MAX");
        }
        return Ok(Self { width, height });
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }

    pub fn size(&self) -> usize {
        return self.width * self.height;
    }
}

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

#[cfg(test)]
mod tests {
    use super::GridConfig;

    #[test]
    fn test_new() {
        let config = GridConfig::new(100, 100);
        assert_eq!(true, config.is_ok(), "The config failed to be created with basic values");
    }

    #[test]
    fn test_new_invalid_size() {
        let config = GridConfig::new(usize::MAX, 100);
        assert_eq!(true, config.is_err(), "The config should have failed to create because the values would overload");

        let config = GridConfig::new(2, usize::MAX);
        assert_eq!(true, config.is_err(), "The config should have failed to create because the values would overload");
    }
}
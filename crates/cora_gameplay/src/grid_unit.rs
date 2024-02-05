#[derive(Debug, Clone, Copy)]
pub struct GridUnit {
    id: u32,
    pos_x: usize,
    pos_y: usize,
}

impl GridUnit {
    pub fn new(id: u32) -> Self {
        return Self {
            id: id,
            pos_x: 0,
            pos_y: 0,
        };
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }

    pub fn x(&self) -> usize {
        return self.pos_x;
    }

    pub fn y(&self) -> usize {
        return self.pos_y;
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.pos_x = x;
        self.pos_y = y;
    }
}

pub struct Buffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let l = y * self.width + x;
        debug_assert!(l <= self.width * self.height);
        self.buffer[l] = color;
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }
}

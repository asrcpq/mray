pub struct Canvas {
    pub data: Vec<u8>,
    size: (i32, i32),
    color: [f32; 3],
}

impl Canvas {
    pub fn new(size: (i32, i32)) -> Canvas {
        Canvas {
            data: vec![0; (size.0 * size.1 * 3) as usize],
            size,
            color: [0., 0., 0.],
        }
    }

    pub fn flush(&mut self) {
        self.data = vec![0; (self.size.0 * self.size.1 * 3) as usize];
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }

    #[inline]
    pub fn putpixel(&mut self, x: i32, y: i32, alpha: f32) {
        if x < 0 || y < 0 || x >= self.size.0 || y >= self.size.1 {
            return;
        }
        let mut pos = ((y * self.size.0 + x) * 3) as usize;
        self.data[pos] = (self.data[pos] as f32 * (1. - alpha) + self.color[0] * 255. * alpha) as u8;
        pos += 1;
        self.data[pos] = (self.data[pos] as f32 * (1. - alpha) + self.color[1] * 255. * alpha) as u8;
        pos += 1;
        self.data[pos] = (self.data[pos] as f32 * (1. - alpha) + self.color[2] * 255. * alpha) as u8;
    }
}

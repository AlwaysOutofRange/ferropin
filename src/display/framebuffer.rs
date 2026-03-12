pub struct FrameBuffer<const W: usize, const H: usize, const N: usize> {
    pub buf: [u8; N],
}

impl<const W: usize, const H: usize, const N: usize> FrameBuffer<W, H, N> {
    pub fn new() -> Self {
        FrameBuffer { buf: [0u8; N] }
    }

    pub fn clear(&mut self) {
        self.buf.fill(0);
    }

    pub fn fill(&mut self) {
        self.buf.fill(0xFF);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        if x >= W || y >= H {
            return;
        }

        let index = (y / 8) * W + x;
        let bit = y % 8;

        if on {
            self.buf[index] |= 1 << bit;
        } else {
            self.buf[index] &= !(1 << bit);
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        if x >= W || y >= H {
            return false;
        }

        let index = (y / 8) * W + x;
        let bit = y % 8;

        self.buf[index] & (1 << bit) != 0
    }
}

pub struct Source {
    bytes: Vec<u8>,
    n: usize,
}

pub const EOZ: i32 = -1;

impl Source {
    pub fn new(input: &str) -> Source {
        Source {
            bytes: Vec::from(input),
            n: 0,
        }
    }

    pub fn getc(&mut self) -> i32 {
        let ch = if self.n >= self.bytes.len() {
            EOZ
        } else {
            self.bytes[self.n] as i32
        };
        self.n += 1;
        ch
    }
}

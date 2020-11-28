pub struct Source<'a> {
    len: usize,
    bytes: &'a [u8],
    n: usize,
}

pub const EOZ: i32 = -1;

impl<'a> Source<'a> {
    pub fn new(input: &'a str) -> Source {
        Source {
            len: input.len(),
            bytes: input.as_bytes(),
            n: 0,
        }
    }

    pub fn getc(&mut self) -> i32 {
        let ch = if self.n >= self.len {
            EOZ
        } else {
            self.bytes[self.n] as i32
        };
        self.n += 1;
        ch
    }
}

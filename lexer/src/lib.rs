pub mod source;

const TOK_OFS: i32 = 256;

const TOK_and: i32 = 257;
const TOK_break: i32 = 258;
const TOK_do: i32 = 259;
const TOK_else: i32 = 260;
const TOK_elseif: i32 = 261;
const TOK_end: i32 = 262;
const TOK_false: i32 = 263;
const TOK_for: i32 = 264;
const TOK_function: i32 = 265;
const TOK_goto: i32 = 266;
const TOK_if: i32 = 267;
const TOK_in: i32 = 268;
const TOK_local: i32 = 269;
const TOK_defer: i32 = 270;
const TOK_nil: i32 = 271;
const TOK_not: i32 = 272;
const TOK_or: i32 = 273;
const TOK_repeat: i32 = 274;
const TOK_return: i32 = 275;
const TOK_then: i32 = 276;
const TOK_true: i32 = 277;
const TOK_until: i32 = 278;
const TOK_while: i32 = 279;
const TOK_IDIV: i32 = 280;
const TOK_CONCAT: i32 = 281;
const TOK_DOTS: i32 = 282;
const TOK_EQ: i32 = 283;
const TOK_GE: i32 = 284;
const TOK_LE: i32 = 285;
const TOK_NE: i32 = 286;
const TOK_SHL: i32 = 287;
const TOK_SHR: i32 = 288;
const TOK_DBCOLON: i32 = 289;
const TOK_TO_INTEGER: i32 = 290;
const TOK_TO_NUMBER: i32 = 291;
const TOK_TO_INTARRAY: i32 = 292;
const TOK_TO_NUMARRAY: i32 = 293;
const TOK_TO_TABLE: i32 = 294;
const TOK_TO_STRING: i32 = 295;
const TOK_TO_CLOSURE: i32 = 296;
const TOK_EOS: i32 = 297;
const TOK_FLT: i32 = 298;
const TOK_INT: i32 = 299;
const TOK_NAME: i32 = 300;
const TOK_STRING: i32 = 301;

const FIRST_RESERVED: i32 = TOK_OFS + 1;
const LAST_RESERVED: i32 = TOK_while - TOK_OFS;


/*
 * Lua strings can have embedded 0 bytes therefore we
 * need a string type that has a length associated with it.
 *
 * The compiler stores a single copy of each string so that strings
 * can be compared by equality.
 */
struct StringObject<'a> {
	len: i32,	  /* length of the string */
	reserved: i32, /* if is this a keyword then token id else -1 */
    hash: u32,	  /* hash value of the string */
    str: &'a [u8] /* The string data */
}

const CHUNK_SIZE: usize = 1024;
struct StringChunk {
    chunk: [u8; CHUNK_SIZE],
    pos: usize
}

struct StringAllocator {
    chunks: Vec<Box<StringChunk>>,
}

impl StringAllocator {
    fn new() -> Self {
        StringAllocator {
            chunks : vec![Box::new(StringChunk{ chunk: [0; CHUNK_SIZE], pos: 0 })],
        }
    }

    // fn alloc_string(&mut self, n: usize) -> Option<&mut [u8]> {
    //     let mut cur : Option<&mut Box<StringChunk>> = None;
    //     for i in 0..self.chunks.len() {
    //         let c = &mut self.chunks[i];
    //         if c.pos + n < 1024 {
    //             cur = Some(c);
    //         }
    //     }
    //     if cur.is_none() {
    //         self.chunks.push(Box::new(StringChunk{ chunk: [0; 1024], pos: 0 }));
    //         cur = self.chunks.last_mut();
    //     }
    //     match cur {
    //         None => None,
    //         Some(stringchunk) => {
    //             let pos = stringchunk.pos;
    //             stringchunk.pos = stringchunk.pos + n;
    //             Some(&mut stringchunk.chunk[pos .. n])
    //         }
    //     }
    // }

    fn alloc_string(&mut self, n: usize) -> Option<&mut [u8]> {
        if n > CHUNK_SIZE {
            return None;
        }
        let mut j: usize = self.chunks.len();
        for i in 0..self.chunks.len() {
            let c = &self.chunks[i];
            if c.pos + n <= CHUNK_SIZE {
                j = i;
            }
        }
        if j == self.chunks.len() {
            self.chunks.push(Box::new(StringChunk{ chunk: [0; CHUNK_SIZE], pos: 0 }));
        }
        let cur = &mut self.chunks[j];
        let pos = cur.pos;
        cur.pos = cur.pos + n;
        Some(&mut cur.chunk[pos .. n])
    }
}


#[cfg(test)]
mod tests {
    use crate::StringAllocator;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_string_alloc() {
        let mut alloc = StringAllocator::new();
        let slice1 = alloc.alloc_string(10);
        let slice2 = alloc.alloc_string(10);
        assert_eq!(1, alloc.chunks.len());
        assert_eq!(20, alloc.chunks[0].pos);
        let slice3 = alloc.alloc_string(1005);
        assert_eq!(2, alloc.chunks.len());
        assert_eq!(1005, alloc.chunks[1].pos);
        let slice4 = alloc.alloc_string(20);
        assert_eq!(2, alloc.chunks.len());
        assert_eq!(40, alloc.chunks[0].pos);
        assert_eq!(1005, alloc.chunks[1].pos);
    }
}

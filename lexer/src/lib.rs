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

struct StringChunk {
    chunk: [u8; 1024],
}

struct StringAllocator {
    chunks: Vec<Box<StringChunk>>,
    pos: usize
}

impl StringAllocator {
    fn new() -> Self {
        StringAllocator {
            chunks : vec![Box::new(StringChunk{ chunk: [0; 1024] })],
            pos: 0
        }
    }

    fn alloc_string(&mut self, n: usize) -> &mut [u8] {
        let cur = self.chunks.last_mut();
        match cur {
            None => {
                self.chunks.push(Box::new(StringChunk{ chunk: [0; 1024] }));
            }
            Some(p) => {
                if self.pos + n > p.chunk.len() {
                    self.chunks.push(Box::new(StringChunk{ chunk: [0; 1024] }));
                    self.pos = 0;
                }
            }
        }
        let i = self.chunks.len()-1;
        let top = &mut self.chunks[i];
        let pos = self.pos;
        self.pos = self.pos + n;
        &mut top.chunk[pos .. n]
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

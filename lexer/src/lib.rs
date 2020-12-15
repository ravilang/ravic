pub mod config;
pub mod source;

use crate::config::*;
use crate::source::Source;
use crate::source::EOZ;
use core::hash::Hash;
use core::hash::Hasher;
use std::collections::HashSet;
use std::fmt;
use std::string;

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

const CHAR_RET: i32 = '\r' as i32;
const CHAR_NL: i32 = '\n' as i32;
const CHAR_SPACE: i32 = ' ' as i32;
const CHAR_FF: i32 = 12;
const CHAR_HTAB: i32 = '\t' as i32;
const CHAR_VTAB: i32 = 11;

const luai_ctype_: [i32; 257] = [
    0x00, /* EOZ */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0. */
    0x00, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* 1. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, /* 2. */
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x16, 0x16, 0x16, 0x16, 0x16, 0x16, 0x16,
    0x16, /* 3. */
    0x16, 0x16, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15,
    0x05, /* 4. */
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, /* 5. */
    0x05, 0x05, 0x05, 0x04, 0x04, 0x04, 0x04, 0x05, 0x04, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15,
    0x05, /* 6. */
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, /* 7. */
    0x05, 0x05, 0x05, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* 8. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* 9. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* a. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* b. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* c. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* d. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* e. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* f. */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const ALPHABIT: i32 = 0;
const DIGITBIT: i32 = 1;
const PRINTBIT: i32 = 2;
const SPACEBIT: i32 = 3;
const XDIGITBIT: i32 = 4;

fn MASK(B: i32) -> i32 {
    1 << B
}

fn testprop(c: i32, p: i32) -> i32 {
    luai_ctype_[c as usize + 1] & p
}

fn lislalpha(c: i32) -> bool {
    testprop(c, MASK(ALPHABIT)) != 0
}
fn lislalnum(c: i32) -> bool {
    testprop(c, MASK(ALPHABIT) | MASK(DIGITBIT)) != 0
}
fn lisdigit(c: i32) -> bool {
    testprop(c, MASK(DIGITBIT)) != 0
}
fn lisspace(c: i32) -> bool {
    testprop(c, MASK(SPACEBIT)) != 0
}
fn lisxdigit(c: i32) -> bool {
    testprop(c, MASK(XDIGITBIT)) != 0
}

/*
 * Lua strings can have embedded 0 bytes therefore we
 * need a string type that has a length associated with it.
 *
 * The compiler stores a single copy of each string so that strings
 * can be compared by equality.
 */
pub struct StringObject<'a> {
    reserved: i32, /* if is this a keyword then token id else -1 */
    str: &'a [u8], /* The string data */
}

impl fmt::Debug for StringObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = string::String::from_utf8(self.str.to_vec());
        write!(f, "'{:?}'", str)
    }
}

impl fmt::Display for StringObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = string::String::from_utf8(self.str.to_vec());
        write!(f, "'{:?}'", str)
    }
}

impl PartialEq for StringObject<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.str.len() != other.str.len() {
            return false;
        }
        for i in 0..self.str.len() {
            if self.str[i] != other.str[i] {
                return false;
            }
        }
        true
    }
}
impl Eq for StringObject<'_> {}

impl Hash for StringObject<'_> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.str.hash(hasher)
    }
}

const CHUNK_SIZE: usize = 1024;
struct StringChunk {
    chunk: [u8; CHUNK_SIZE],
    pos: usize,
}

struct StringAllocator {
    chunks: Vec<Box<StringChunk>>,
}

impl StringAllocator {
    fn new() -> Self {
        StringAllocator {
            chunks: vec![Box::new(StringChunk {
                chunk: [0; CHUNK_SIZE],
                pos: 0,
            })],
        }
    }

    // fn alloc_string(&mut self, n: usize) -> Option<&mut [u8]> {
    //     let mut cur : Option<&mut Box<StringChunk>> = None;
    //     for i in 0..self.chunks.len() {
    //         let c = &self.chunks[i];
    //         if c.pos + n <= CHUNK_SIZE {
    //             cur = Some(&mut self.chunks[i]);
    //             break;
    //         }
    //     }
    //     if cur.is_none() {
    //         self.chunks.push(Box::new(StringChunk{ chunk: [0; CHUNK_SIZE], pos: 0 }));
    //         cur = self.chunks.last_mut();
    //     }
    //     match cur {
    //         None => None,
    //         Some(stringchunk) => {
    //             let pos = stringchunk.pos;
    //             stringchunk.pos = stringchunk.pos + n;
    //             Some(&mut stringchunk.chunk[pos .. stringchunk.pos])
    //         }
    //     }
    // }

    fn alloc_string<'a>(&'a mut self, n: usize) -> Option<&'a mut [u8]> {
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
            self.chunks.push(Box::new(StringChunk {
                chunk: [0; CHUNK_SIZE],
                pos: 0,
            }));
        }
        let cur = &mut self.chunks[j];
        let pos = cur.pos;
        cur.pos = cur.pos + n;
        Some(&mut cur.chunk[pos..cur.pos])
    }
}

struct StringCache<'a> {
    map: HashSet<Box<StringObject<'a>>>,
    allocator: StringAllocator,
}

impl<'a> StringCache<'a> {
    fn new() -> Self {
        StringCache {
            map: HashSet::new(),
            allocator: StringAllocator::new(),
        }
    }

    fn get(&'a mut self, buf: &'a std::vec::Vec<u8>) -> Option<&'a StringObject> {
        let slice = &buf[0..buf.len()];
        let str_obj = StringObject {
            reserved: -1,
            str: slice,
        };
        let tab = &mut self.map;
        if !tab.contains(&str_obj) {
            let s = self.allocator.alloc_string(slice.len());
            match s {
                Some(ss) => {
                    for i in 0..ss.len() {
                        ss[i] = slice[i];
                    }
                    let obj = Box::new(StringObject {
                        reserved: -1,
                        str: ss,
                    });
                    println!("allocated");
                    tab.insert(obj);
                }
                None => {}
            }
        }
        {
            let existing = tab.get(&str_obj);
            match existing {
                Some(e) => Some(e),
                None => None,
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SemInfo<'a> {
    Integer { i: lua_Integer },
    Number { r: lua_Number },
    String { str: &'a StringObject<'a> },
    None {},
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    token: i32, /* Token value or character value; token values start from FIRST_RESERVED which is 257, values < 256
                are characters */
    seminfo: SemInfo<'a>, /* Literal associated with the token, only valid when token is a literal or an identifier, i.e.
                          token is > TOK_EOS */
}

pub struct Lexer<'a> {
    current: i32,
    linenumber: i32,
    lastline: i32,
    t: Token<'a>,
    lookahead: Token<'a>,
    source: Source<'a>,
    buff: Vec<u8>,
    strCache: StringCache<'a>,
}

impl<'a> Lexer<'a> {
    fn new(mut source: Source<'a>) -> Self {
        Lexer {
            current: source.getc(),
            linenumber: 0,
            lastline: 0,
            source: source,
            lookahead: Token {
                token: TOK_EOS,
                seminfo: SemInfo::None {},
            },
            t: Token {
                token: 0,
                seminfo: SemInfo::None {},
            },
            buff: vec![],
            strCache: StringCache::new(),
        }
    }
    fn next_ch(&mut self) {
        self.current = self.source.getc();
    }

    fn currIsNewline(&self) -> bool {
        self.current == CHAR_NL || self.current == CHAR_RET
    }

    fn inclinenumber(&mut self) {
        let old = self.current;
        self.next_ch(); /* skip '\n' or '\r' */
        if self.currIsNewline() && self.current != old {
            self.next_ch(); /* skip '\n\r' or '\r\n' */
        }
        self.linenumber += 1;
        // if (++ls->linenumber >= INT_MAX)
        // lexerror(ls, "chunk has too many lines", 0);
    }

    fn llex(&mut self, lookingahead: bool) -> i32 {
        let seminfo = if lookingahead {
            &mut self.lookahead.seminfo
        } else {
            &mut self.t.seminfo
        };
        self.buff.clear();
        loop {
            match self.current {
                CHAR_RET | CHAR_NL => {
                    self.inclinenumber();
                }

                CHAR_SPACE | CHAR_FF | CHAR_HTAB | CHAR_VTAB => {
                    /* spaces */
                    self.next_ch();
                }

                EOZ => {
                    break;
                }

                _ => {
                    if lislalnum(self.current) {
                    } else {
                        let c = self.current;
                        self.next_ch();
                        return c;
                    }
                }
            }
        }
        0
    }

    fn next(&mut self) {
        self.lastline = self.linenumber;
        if self.lookahead.token == TOK_EOS {
            /* is there a look-ahead token? */
            self.t = self.lookahead; /* use this one */
            self.lookahead.token = TOK_EOS; /* and discharge it */
        } else {
            self.t.token = self.llex(false);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexer;
    use crate::Source;
    use crate::StringAllocator;
    use crate::StringCache;

    #[test]
    fn test_string_alloc() {
        let mut alloc = StringAllocator::new();
        let slice1 = alloc.alloc_string(10);
        assert_eq!(10, slice1.unwrap().len());
        let slice2 = alloc.alloc_string(10);
        assert_eq!(10, slice2.unwrap().len());
        assert_eq!(1, alloc.chunks.len());
        assert_eq!(20, alloc.chunks[0].pos);
        let slice3 = alloc.alloc_string(1005);
        assert_eq!(1005, slice3.unwrap().len());
        assert_eq!(2, alloc.chunks.len());
        assert_eq!(1005, alloc.chunks[1].pos);
        let slice4 = alloc.alloc_string(20);
        assert_eq!(20, slice4.unwrap().len());
        assert_eq!(2, alloc.chunks.len());
        assert_eq!(40, alloc.chunks[0].pos);
        assert_eq!(1005, alloc.chunks[1].pos);
    }

    #[test]
    fn test_string_cache() {
        let mut cache = StringCache::new();
        let buf1 = String::from("hello").into_bytes();
        let result = cache.get(&buf1);
        assert_eq!(buf1, result.unwrap().str);
        // let result2 = cache.get(&buf1);
        // assert_eq!(result.unwrap(), result2.unwrap());
    }

    #[test]
    fn test_lexer() {
        let mut source = Source::new("return 1");
        let mut lexer = Lexer::new(source);
    }
}

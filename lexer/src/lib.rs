pub mod config;
pub mod source;

use crate::config::*;
use crate::source::Source;
use crate::source::EOZ;
use crate::SemInfo::StringLit;
use std::fmt;

const TOK_OFS: i32 = 256;

const TOK_AND: i32 = 257;
const TOK_BREAK: i32 = 258;
const TOK_DO: i32 = 259;
const TOK_ELSE: i32 = 260;
const TOK_ELSEIF: i32 = 261;
const TOK_END: i32 = 262;
const TOK_FALSE: i32 = 263;
const TOK_FOR: i32 = 264;
const TOK_FUNCTION: i32 = 265;
const TOK_GOTO: i32 = 266;
const TOK_IF: i32 = 267;
const TOK_IN: i32 = 268;
const TOK_LOCAL: i32 = 269;
const TOK_DEFER: i32 = 270;
const TOK_NIL: i32 = 271;
const TOK_NOT: i32 = 272;
const TOK_OR: i32 = 273;
const TOK_REPEAT: i32 = 274;
const TOK_RETURN: i32 = 275;
const TOK_THEN: i32 = 276;
const TOK_TRUE: i32 = 277;
const TOK_UNTIL: i32 = 278;
const TOK_WHILE: i32 = 279;
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
const LAST_RESERVED: i32 = TOK_WHILE - TOK_OFS;

const CHAR_RET: i32 = '\r' as i32;
const CHAR_NL: i32 = '\n' as i32;
const CHAR_SPACE: i32 = ' ' as i32;
const CHAR_FF: i32 = 12;
const CHAR_HTAB: i32 = '\t' as i32;
const CHAR_VTAB: i32 = 11;
const CHAR_HYPEN: i32 = '-' as i32;
const CHAR_LBRACKET: i32 = '[' as i32;
const CHAR_RBRACKET: i32 = ']' as i32;
const CHAR_LBRACE: i32 = '{' as i32;
const CHAR_RBRACE: i32 = '}' as i32;
const CHAR_LPAREN: i32 = '(' as i32;
const CHAR_RPAREN: i32 = ')' as i32;
const CHAR_EQUAL: i32 = '=' as i32;
const CHAR_COMMA: i32 = ',' as i32;
const CHAR_LT: i32 = '<' as i32;
const CHAR_GT: i32 = '>' as i32;
const CHAR_FSLASH: i32 = '/' as i32;
const CHAR_TILDE: i32 = '~' as i32;
const CHAR_COLON: i32 = ':' as i32;

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

#[derive(Debug, Clone, PartialEq)]
pub enum SemInfo {
    IntegerLit(lua_Integer),
    NumberLit(lua_Number),
    StringLit(String),
}

impl fmt::Display for SemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SemInfo::IntegerLit(i) => {
                write!(f, "{}", i)
            }
            SemInfo::NumberLit(r) => {
                write!(f, "{}", r)
            }
            SemInfo::StringLit(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token: i32,
    /* Token value or character value; token values start from FIRST_RESERVED which is 257, values < 256
                   are characters */
    seminfo: Option<SemInfo>,
    /* Literal associated with the token, only valid when token is a literal or an identifier, i.e.
                                 token is > TOK_EOS */
}

fn tok(t: i32) -> Option<Token> {
    Some(Token { token: t, seminfo: None })
}

pub struct Lexer {
    current: i32,
    linenumber: i32,
    lastline: i32,
    t: Option<Token>,
    lookahead: Option<Token>,
    source: Source,
    buff: Vec<u8>,
}

impl Lexer {
    fn new(mut source: Source) -> Self {
        Lexer {
            current: source.getc(),
            linenumber: 0,
            lastline: 0,
            source: source,
            lookahead: None,
            t: None,
            buff: vec![],
        }
    }

    fn next_ch(&mut self) {
        self.current = self.source.getc();
    }

    fn save_and_next_ch(&mut self) {
        self.buff.push(self.current as u8);
        self.next_ch();
    }

    fn curr_is_new_line(&self) -> bool {
        self.current == CHAR_NL || self.current == CHAR_RET
    }

    fn inc_line_number(&mut self) {
        let old = self.current;
        self.next_ch(); /* skip '\n' or '\r' */
        if self.curr_is_new_line() && self.current != old {
            self.next_ch(); /* skip '\n\r' or '\r\n' */
        }
        self.linenumber += 1;
        // if (++ls->linenumber >= INT_MAX)
        // lexerror(ls, "chunk has too many lines", 0);
    }

    /*
     ** skip a sequence '[=*[' or ']=*]'; if sequence is well formed, return
     ** its number of '='s; otherwise, return a negative number (-1 iff there
     ** are no '='s after initial bracket)
     */
    fn skip_sep(&mut self) -> i32 {
        let mut count = 0;
        let s = self.current;
        //assert(s == '[' || s == ']');
        self.save_and_next_ch();
        while self.current == CHAR_EQUAL {
            self.save_and_next_ch();
            count += 1;
        }
        if self.current == s {
            count
        } else {
            (-count) - 1
        }
    }

    fn read_long_string(&mut self, save_seminfo: bool, sep: i32) -> Option<Token> {
        //let line = self.linenumber;
        self.save_and_next_ch(); /* skip 2nd '[' */
        if self.curr_is_new_line() {
            /* string starts with a newline? */
            self.inc_line_number(); /* skip it */
        }
        loop {
            match self.current {
                EOZ => {
                    /* error */
                    // TODO
                    break;
                }
                CHAR_RBRACKET => {
                    if self.skip_sep() == sep {
                        self.save_and_next_ch(); /* skip 2nd ']' */
                        break;
                    }
                }
                CHAR_RET | CHAR_NL => {
                    self.buff.push(CHAR_NL as u8);
                    self.inc_line_number();
                    if !save_seminfo {
                        self.buff.clear(); /* avoid wasting space */
                    }
                }
                _ => {
                    if save_seminfo {
                        self.save_and_next_ch();
                    } else {
                        self.next_ch();
                    }
                }
            }
        }
        if save_seminfo {
            let sep = sep as usize;
            let start = 2 + sep;
            let end = self.buff.len() - start;
            let range = start..end;
            let cl = self.buff[range].to_vec();
            Some(Token {
                token: TOK_STRING,
                seminfo: Some(StringLit(String::from_utf8(cl).expect("String expected"))),
            })
        } else {
            None
        }
    }

    fn check_next1(&mut self, c: i32) -> bool {
        if self.current == c {
            self.next_ch();
            true
        } else {
            false
        }
    }

    // Advance the lexer to next token
    fn llex(&mut self) -> Option<Token> {
        self.buff.clear();
        loop {
            match self.current {
                CHAR_RET | CHAR_NL => {
                    self.inc_line_number();
                }

                CHAR_SPACE | CHAR_FF | CHAR_HTAB | CHAR_VTAB => {
                    /* spaces */
                    self.next_ch();
                }

                EOZ => break tok(TOK_EOS),

                CHAR_HYPEN => {
                    /* '-' or '--' (comment) */
                    self.next_ch();
                    if self.current != CHAR_HYPEN {
                        break tok(CHAR_HYPEN);
                    }
                    /* else is a comment */
                    self.next_ch();
                    if self.current == CHAR_LBRACKET {
                        let sep = self.skip_sep();
                        self.buff.clear(); /* 'skip_sep' may dirty the buffer */
                        if sep >= 0 {
                            self.read_long_string(false, sep); /* skip long comment */
                            self.buff.clear(); /* previous call may dirty the buff. */
                            continue;
                        }
                    }
                    /* else short comment */
                    while !self.curr_is_new_line() && self.current != EOZ {
                        self.next_ch(); /* skip until end of line (or end of file) */
                    }
                }

                CHAR_LBRACKET => {
                    /* long string or simply '[' */
                    let sep = self.skip_sep();
                    if sep >= 0 {
                        break self.read_long_string(true, sep);
                    } else if sep != -1
                    /* '[=...' missing second bracket */
                    {
                        //lexerror(ls, "invalid long string delimiter", TOK_STRING);
                        break tok(TOK_EOS);
                    }
                    break tok(CHAR_LBRACKET);
                }

                CHAR_EQUAL => {
                    self.next_ch();
                    if self.check_next1(CHAR_EQUAL) {
                        break tok(TOK_EQ);
                    } else {
                        break tok(CHAR_EQUAL);
                    }
                }

                CHAR_LT => {
                    self.next_ch();
                    if self.check_next1(CHAR_EQUAL) {
                        break tok(TOK_LE);
                    } else if self.check_next1(CHAR_LT) {
                        break tok(TOK_SHL);
                    } else {
                        break tok(CHAR_LT);
                    }
                }

                CHAR_GT => {
                    self.next_ch();
                    if self.check_next1(CHAR_EQUAL) {
                        break tok(TOK_GE);
                    } else if self.check_next1(CHAR_GT) {
                        break tok(TOK_SHR);
                    } else {
                        break tok(CHAR_GT);
                    }
                }

                CHAR_FSLASH => {
                    self.next_ch();
                    if self.check_next1(CHAR_FSLASH) {
                        break tok(TOK_IDIV);
                    } else {
                        break tok(CHAR_FSLASH);
                    }
                }

                CHAR_TILDE => {
                    self.next_ch();
                    if self.check_next1(CHAR_EQUAL) {
                        break tok(TOK_NE);
                    } else {
                        break tok(CHAR_TILDE);
                    }
                }

                CHAR_COLON => {
                    self.next_ch();
                    if self.check_next1(CHAR_COLON) {
                        break tok(TOK_DBCOLON);
                    } else {
                        break tok(CHAR_COLON);
                    }
                }

                _ => {
                    let b = self.current as u8;
                    if lislalnum(self.current) {} else {
                        let c = self.current;
                        self.next_ch();
                        break tok(c);
                    }
                }
            }
        }
    }

    // Lex next token
    // token will be stored in self.t
    fn next_token(&mut self) {
        self.lastline = self.linenumber;
        if self.lookahead.is_some() {
            /* is there a look-ahead token? */
            self.t = self.lookahead.take(); /* use this one */
        } else {
            self.t = self.llex();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Lexer, StringLit, CHAR_COLON, CHAR_EQUAL, CHAR_FSLASH, CHAR_GT, CHAR_HYPEN, CHAR_LBRACE,
        CHAR_LT, CHAR_RBRACE, CHAR_TILDE, TOK_DBCOLON, TOK_EQ, TOK_GE, TOK_IDIV, TOK_LE, TOK_NE,
        TOK_SHL, TOK_SHR, TOK_STRING,
    };
    use crate::{Source, CHAR_COMMA, CHAR_LPAREN, CHAR_RPAREN, TOK_EOS};

    #[test]
    fn test_lexer() {
        let source_string = "
-- a comment
- { } ,
--[[ a
long string
]]
--[==[ another
multi line
 string
]==]
 ( )
  [[ a string ]]
= == < <= << > >= >> // / ~ ~= :: :
        ";

        let mut source = Source::new(source_string);
        let mut lexer = Lexer::new(source);
        lexer.next_token();
        assert_eq!(CHAR_HYPEN, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_LBRACE, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_RBRACE, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_COMMA, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_LPAREN, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_RPAREN, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_STRING, lexer.t.as_ref().unwrap().token);
        assert_eq!(
            StringLit(" a string ".to_string()),
            lexer.t.as_ref().unwrap().seminfo.as_ref().unwrap().clone()
        );
        lexer.next_token();
        assert_eq!(CHAR_EQUAL, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_EQ, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_LT, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_LE, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_SHL, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_GT, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_GE, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_SHR, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_IDIV, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_FSLASH, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_TILDE, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_NE, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_DBCOLON, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(CHAR_COLON, lexer.t.as_ref().unwrap().token);
        lexer.next_token();
        assert_eq!(TOK_EOS, lexer.t.as_ref().unwrap().token);
    }
}

#![no_std]
#![feature(iter_advance_by)]
#![feature(iter_next_chunk)]

extern crate alloc;
extern crate mos_alloc;

pub mod memory;
pub mod parse;

use alloc::string::String;
use mos_hardware::mega65::lpeek;

pub const STATUS_ADDR: u32 = 0x4ff07;
pub const RVS_ON: &str = "\x12";
pub const RVS_OFF: &str = "\u{0092}";
pub const TYPE_SUFFIX: [&str; 4] = ["", "%", "$", "&"];

// This is evaluated at compile time!
pub const BIN_CONV: [u16; 16] = {
    let mut arr = [0; 16];
    arr[0] = 1;
    let mut i = 1;
    while i < 16 {
        arr[i] = arr[i - 1] * 2;
        i += 1;
    }
    arr
};

pub const TOKENS: [&str; 190] = [
    "print",
    "input",
    "if",
    "then",
    "else",
    "do",
    "loop",
    "while",
    "until",
    "gosub",
    "goto",
    "open",
    "close",
    "dopen",
    "dclose",
    "for",
    "next",
    "getkey",
    "hex$",
    "dim",
    "peek",
    "poke",
    "wait",
    "dec",
    "chr$",
    "asc",
    "sgn",
    "sqr",
    "graphic",
    "clr",
    "screen",
    "def",
    "begin",
    "bend",
    "len",
    "mid$",
    "right$",
    "left$",
    "instr",
    "for",
    "next",
    "step",
    "trap",
    "border",
    "and",
    "foreground",
    "background",
    "set",
    "abs",
    "sin",
    "cos",
    "tan",
    "log",
    "fre",
    "cursor",
    "pixel",
    "window",
    "rwindow",
    "line",
    "box",
    "circle",
    "ellipse",
    "palette",
    "restore",
    "data",
    "err$",
    "er",
    "el",
    "cursor",
    "on",
    "off",
    "val",
    "scratch",
    "return",
    "rnd",
    "stop",
    "bank",
    "ti",
    "do",
    "or",
    "st",
    "if",
    "el",
    "er",
    "on",
    "to",
    "pen",
    "get",
    "end",
    "int",
    "not",
    "ds",
    "run",
    "using",
    "append",
    "atn",
    "auto",
    "backup",
    "bload",
    "boot",
    "bsave",
    "bump",
    "bverify",
    "catalog",
    "change",
    "char",
    "cmd",
    "collision",
    "color",
    "concat",
    "cont",
    "copy",
    "wpoke",
    "wpeek",
    "setbit",
    "dclear",
    "deffn",
    "delete",
    "fn",
    "dir",
    "disk",
    "dload",
    "dma",
    "dmode",
    "dpat",
    "dsave",
    "dverify",
    "edma",
    "envelope",
    "erase",
    "exit",
    "exp",
    "fast",
    "filter",
    "find",
    "go64",
    "header",
    "help",
    "highlight",
    "joy",
    "list",
    "load",
    "locate",
    "lpen",
    "mod",
    "monitor",
    "mouse",
    "movspr",
    "new",
    "paint",
    "play",
    "pointer",
    "polygon",
    "pos",
    "pot",
    "pudef",
    "rclr",
    "rdot",
    "read",
    "record",
    "rem",
    "rename",
    "resume",
    "rgraphic",
    "rmouse",
    "rplay",
    "rreg",
    "rspcolor",
    "rsppos",
    "rsprite",
    "save",
    "scnclr",
    "sleep",
    "slow",
    "sound",
    "spc",
    "sprcolor",
    "sprite",
    "sprsav",
    "sys",
    "tab",
    "tempo",
    "troff",
    "tron",
    "type",
    "usr",
    "verify",
    "vol",
    "xor",
    "key",
];

pub fn read_line(ca_addr: &mut memory::MemoryIterator) -> String {
    let line_length = ca_addr.next().unwrap() as usize;
    ca_addr
        .take(line_length)
        .map(|byte| char::from(byte))
        .collect::<String>()
}

/// Read filename from memory
pub fn get_filename() -> Option<String> {
    const LETTER_S: u8 = 83;
    const LETTER_K: u8 = 75;
    let mut mem = memory::MemoryIterator::new(0x4ff00);

    if mem.peek_bytes(2).as_slice() != [LETTER_S, LETTER_K] {
        return None;
    }
    mem.advance_by(16).unwrap();

    let filename: String = mem
        .take_while(|byte| *byte != 0)
        .map(|byte| char::from(byte))
        .collect();

    Some(filename)
}

/// Check status register if we should use verbose output
///
/// - BASIC: `7040 vb=peek(dec("ff07"))and8`
pub fn is_verbose() -> bool {
    lpeek(STATUS_ADDR) & 8 == 8
}

/// Determine if autoload should be done
///
/// # Notes
///
/// - BASIC: `7060 if peek(dec("ff07"))and 1 then return`
/// - this bit got referred to as an autoload bit?
/// - it gets set by '11.edit' in the gosub 7720 (save filename in mailbox ram)
/// ~~~
/// 7070 print "filename? "+f$:print"{up}";
/// 7080 bend
/// // NOTE: not sure how to do 'input' in rust yet, so skipping this part...
/// // (maybe something in mega65's libc could do it?)
/// 7090 input "filename";a$
/// 7100 if a$="" thenprint "no filename set":end
/// 7110 poke ba,asc("s"):poke ba+1,asc("k")
/// 7120 forr=1to16:poke ba+8+r-1,asc(mid$(a$,r,1)):nextr
/// 7130 f$=a$
/// 7140 return
/// ~~~
pub fn autoload() -> bool {
    lpeek(STATUS_ADDR) & 1 == 1
}

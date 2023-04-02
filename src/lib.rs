#![no_std]

extern crate alloc;
extern crate mos_alloc;

pub mod memory;
pub mod parse;

use alloc::string::String;
use alloc::string::ToString;
use mos_hardware::mega65::lpeek;
use ufmt_stdio::*;

pub const RVS_ON: &str = "\x12";
pub const RVS_OFF: &str = "\u{0092}";
pub const TYPE_SUFFIX: [&str; 4] = ["", "%", "$", "&"];

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

pub fn trim_left<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str {
    let mut i = 0;
    while i < line.len() && trim_chars.contains(&line.as_bytes()[i]) {
        i += 1;
    }
    &line[i..]
}

pub fn trim_right<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str {
    let mut i = (line.len() - 1) as i16;

    while i >= 0 && trim_chars.contains(&line.as_bytes()[i as usize]) {
        i = i - 1;
    }

    &line[..((i + 1) as usize)]
}

pub fn single_quote_comment_trim(current_line: &mut String) {
    //422
    if current_line.find('\'').is_none() || current_line.find('"').is_none() {
        return;
    }
    //423
    //424
    let mut quote_flag = false;
    let mut cut_tail_idx = None;
    //440
    for (in_line_idx, c) in current_line.chars().enumerate() {
        //let c = (*current_line).chars().nth(in_line_idx).unwrap();
        match c {
            '"' => quote_flag = !quote_flag,
            '\'' => {
                if !quote_flag {
                    cut_tail_idx = Some(in_line_idx);
                    break;
                }
            }
            _ => (),
        }
    }
    //540
    if cut_tail_idx.is_some() {
        *current_line = current_line[..cut_tail_idx.unwrap()].to_string();
    }
    //println!("'{}'", &(*current_line)[..]);
}

/// @todo: skip `current_line` as argument as it is zeroed
pub fn read_line(ca_addr: &mut memory::MemoryIterator) -> String {
    let line_length = ca_addr.value;
    ca_addr.next();
    let mut line = String::with_capacity(line_length as usize);
    ca_addr
        .take(line_length as usize)
        .for_each(|byte| line.push(byte as char));
    line
}

pub fn get_filename(verbose: &mut bool) -> String {
    println!("get-filename");
    let mut filename = String::new();
    let mut addr: u32 = 0x4ff00;
    // 7020 bank 4:ba=dec("ff00")
    // 7030 if peek(ba+0)=asc("s") and peek(ba+1)=asc("k") thenbegin
    const LETTER_S: u8 = 83;
    const LETTER_K: u8 = 75;
    if lpeek(addr) == LETTER_S && lpeek(addr + 1) == LETTER_K {
        // 7040   vb=peek(dec("ff07"))and8
        *verbose = lpeek(0x4ff07u32) & 8 == 8;
        if *verbose {
            println!("verbose");
        }
        // 7050   f$="":a=ba+16:dowhilepeek(a)<>0:f$=f$+chr$(peek(a)):a=a+1:loop:
        addr += 16;
        while lpeek(addr) != 0 {
            filename.push(lpeek(addr) as char);
            addr += 1;
        }

        // 7060   if peek(dec("ff07"))and1 thenreturn
        if lpeek(0x4ff07u32) & 1 == 1 {
            // this bit got referred to as an autoload bit?
            // it gets set by '11.edit' in the gosub 7720 (save filename in mailbox ram)
            return filename;
        }

        // 7070   print "filename? "+f$:print"{up}";
        println!("FILENAME? {}", &filename[..]);
        // 7080 bend
    }

    return filename;
    // NOTE: not sure how to do 'input' in rust yet, so skipping this part...
    // (maybe something in mega65's libc could do it?)

    // 7090 input "filename";a$
    // 7100 if a$="" thenprint "no filename set":end
    // 7110 poke ba,asc("s"):poke ba+1,asc("k")
    // 7120 forr=1to16:poke ba+8+r-1,asc(mid$(a$,r,1)):nextr
    // 7130 f$=a$
    // 7140 return
}

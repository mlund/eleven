//! Eleven interpreter for mega65
//!
//! Longer description...

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::{string::String, vec::Vec};
use core::panic::PanicInfo;
use eleven::memory::MemoryIterator;
use eleven::parse::Label;
use mos_hardware::mega65::libc::mega65_fast;
use mos_hardware::mega65::{lpeek, set_lower_case};
use ufmt_stdio::*;

/*fn print(s: String) {
    let cstr: Vec<u8> = Vec::with_capacity(s.len() + 1);
    let x = 0;
    let ptr = &s[..].as_ptr();

    for x in 0..s.len() {
        cstr[x] = *ptr;
        ptr += 1;
    }
}*/

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut current_line = String::new();
    let mut verbose = true;
    let mut pp_line: u16 = 0;
    let mut delete_line_flag: bool = false;
    let mut labels: Vec<Label> = Vec::with_capacity(200);

    eleven::memory::prepare_test_memory(&mut verbose);

    // ln%() = map_gen_line_to_orig_line[]
    let _map_gen_line_to_orig_line: [u16; 500] = [0; 500];

    set_lower_case();
    println!("testing TESTING 1, 2, 3...");

    // li$() = processed_lines
    // NOTE: Seems like rust chokes if this is too large?
    let _processed_lines: Vec<String> = Vec::with_capacity(200);

    set_lower_case();
    println!(
        "{}eleven PREPROCESSOR V0.4.7{}",
        eleven::RVS_ON,
        eleven::RVS_OFF
    );

    //unsafe { cputs("hello".as_ptr()); }
    println!();

    // tl$ = tl_string
    let mut _tl_string = String::new(); //String::from("                                                                                ");
                                        // bl$ = bl_string
                                        //let mut bl_string: String = String::new();
                                        //bl_string.push_str(&tl_string[..]);
                                        //bl_string.push_str(&tl_string[..]);
                                        //bl_string.push_str(&tl_string[..]);

    // tl_string = String::new();

    //for i in 0..tokens.len() {
    //    println!("{}", tokens[i]);
    //}

    //let mystring = String::from("test");
    //println!("{}", &mystring[..]);

    let filename = eleven::get_filename(&mut verbose).unwrap();

    unsafe {
        mega65_fast();
    }

    println!("{}", *filename);

    // ------------------- pass 1 ---------------
    // nl = next_line_flag
    let mut _next_line_flag = false;

    // wh$ = whitespace_chars
    const WHITESPACE_CHARS: [u8; 4] = [32, 160, 29, 9]; // space, shift+space, right, tab

    // clean up temporary files
    // NOTE: sl/source_line_counter and rl/current_line_index serve the same purpose
    // so I will remove 'current_line_index'
    //let mut source_line_counter = 0; // sl
    let mut _post_proc_line_counter = 0; // ln

    // TODO: 195 clr ti: rem keep start time for timing
    const CB_ADDR: u32 = 0x8010000;
    let mut ca_addr = MemoryIterator::new(CB_ADDR);

    println!("PASS 1 ");

    // rl = current_line_index (zero-indexed, increments by one)
    // removing this one, as it's equivalent to sl/soure_line_counter
    //let mut current_line_index = 0;
    // tl = total_lines

    // @todo Check endianess...
    let num_lines = u16::from_le_bytes([ca_addr.next().unwrap(), ca_addr.next().unwrap()]);

    pp_line = 0; // ln = index into li$ (current post-processed line)

    //200
    for line_number in 0..num_lines {
        current_line = eleven::read_line(&mut ca_addr);

        println!("l{}: {}", line_number, *current_line);

        // 340
        current_line = eleven::trim_left(&current_line, &WHITESPACE_CHARS).into();
        println!("{}", *current_line);

        eleven::single_quote_comment_trim(&mut current_line);

        //560-580
        if !current_line.is_empty() {
            current_line = eleven::trim_right(&current_line, &WHITESPACE_CHARS).into();
        }

        //585
        if current_line.len() > 0 {
            // dl = delete_line_flag
            delete_line_flag = false;
            if verbose {
                println!(
                    ">> {} {} {}",
                    _post_proc_line_counter, line_number, *current_line
                );
                // 600
                if current_line.chars().nth(0).unwrap() == '.' {
                    println!("dot!");
                    _next_line_flag = true;
                    eleven::parse::add_label(
                        verbose,
                        &current_line,
                        pp_line,
                        &mut delete_line_flag,
                        &mut labels,
                    );
                }
            }
        }

        // 750
        //source_line_counter += 1;
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("!");
    loop {}
}

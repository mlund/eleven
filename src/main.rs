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
use mos_hardware::mega65;
use ufmt_stdio::*;

const CB_ADDR: u32 = 0x8010000;

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
    let mut verbose = false;
    let mut pp_line: u16 = 0;
    let mut delete_line_flag: bool = false;
    let mut labels: Vec<Label> = Vec::with_capacity(200);

    mega65::speed_mode40();
    mega65::set_lower_case();

    println!(
        "{}eleven PREPROCESSOR V0.4.7{}",
        eleven::RVS_ON,
        eleven::RVS_OFF
    );

    eleven::memory::prepare_test_memory(&mut verbose);

    let filename = eleven::get_filename().unwrap();
    println!("{}", filename.as_str());

    let _autoload = eleven::auto_load();
    verbose = eleven::is_verbose();

    // ln%() = map_gen_line_to_orig_line[]
    let _map_gen_line_to_orig_line: [u16; 500] = [0; 500];

    println!("testing TESTING 1, 2, 3...");

    // Processed lines (`li`). Warning: Rust chokes if this is too large
    let _processed_lines: Vec<String> = Vec::with_capacity(200);

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

    // ------------------- pass 1 ---------------
    // nl = next_line_flag
    let mut _next_line_flag = false;

    // clean up temporary files
    // NOTE: sl/source_line_counter and rl/current_line_index serve the same purpose
    // so I will remove 'current_line_index'
    //let mut source_line_counter = 0; // sl
    let mut _post_proc_line_counter = 0; // ln

    // TODO: 195 clr ti: rem keep start time for timing

    // rl = current_line_index (zero-indexed, increments by one)
    // removing this one, as it's equivalent to sl/soure_line_counter
    //let mut current_line_index = 0;
    // tl = total_lines

    println!("PASS 1 ");

    let mut ca_addr = MemoryIterator::new(CB_ADDR);

    // @todo Check endianess...
    let num_lines = u16::from_le_bytes([ca_addr.next().unwrap(), ca_addr.next().unwrap()]);

    pp_line = 0; // ln = index into li$ (current post-processed line)

    for line_number in 0..num_lines {
        let mut line = eleven::read_line(&mut ca_addr);
        line = eleven::parse::trim_line(&line).into();

        if line.is_empty() {
            continue;
        }
        println!("l{}: {}", line_number, line.as_str());

        // dl = delete_line_flag
        delete_line_flag = false;
        if verbose {
            println!(
                ">> {} {} {}",
                _post_proc_line_counter,
                line_number,
                line.as_str()
            );
        }
        // 600
        if line.starts_with('.') {
            println!("dot!");
            _next_line_flag = true;
            eleven::parse::add_label(verbose, &line, pp_line, &mut delete_line_flag, &mut labels);
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

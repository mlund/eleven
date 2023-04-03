use alloc::vec::Vec;
use mos_hardware::mega65::{lcopy, lpeek};

/// Never-ending iterator to lpeek into 28-bit memory
///
/// The address is automatically pushed forward with every read.
///
/// # Examples
/// ~~~
/// const ADDRESS: u32 = 0x8010000;
/// let mut mem = MemoryIterator::new(ADDRESS);
/// let single_byte: u8 = mem.next().unwrap();
/// assert_eq!(mem.address, ADDRESS + 1);
/// let byte_vector = mem.peek_chunk(10);
/// for byte in mem.take(4) {
///     println!("{}", byte);
/// }
/// ~~~
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemoryIterator {
    /// Next address to be used
    pub address: u32,
}

impl MemoryIterator {
    pub fn new(address: u32) -> Self {
        Self { address: address }
    }

    /// Peek `n` bytes using fast Direct Memory Access (DMA) copy
    ///
    /// # Todo
    ///
    /// - Check that the DMA copy works as expected
    pub fn peek_chunk(&mut self, n: u16) -> Vec<u8> {
        //self.take(len).collect()
        let mut dst = Vec::<u8>::with_capacity(n as usize);
        unsafe {
            dst.set_len(n as usize);
            lcopy(self.address, dst.as_mut_slice().as_ptr() as u32, n);
        }
        self.address += n as u32;
        dst
    }
}

impl Iterator for MemoryIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let value = lpeek(self.address);
        self.address += 1;
        Some(value)
    }

    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        self.address += n as u32;
        Ok(())
    }
}

/// Fill memory with test data
///
/// This is used for development only.
/// - Make verbose global?
pub fn prepare_test_memory(verbose: &mut bool) {
    // turn on verbose flag
    // (in memory doesn't work yet, as I'd have to put dummy info into 0x4ff00 to be parsed by get_filename()
    // unsafe { lpoke(0x4ff07u32, 0x08u8); }
    // so for now, just hardcode the flag
    *verbose = true;

    const DATA: [u8; 97] = [
        0x08, 0x00, 0x0f, 0x23, 0x4f, 0x55, 0x54, 0x50, 0x55, 0x54, 0x20, 0x22, 0x48, 0x45, 0x4c,
        0x4c, 0x4f, 0x22, 0x00, 0x0a, 0x23, 0x44, 0x45, 0x43, 0x4c, 0x41, 0x52, 0x45, 0x20, 0x58,
        0x00, 0x05, 0x2e, 0x4d, 0x41, 0x49, 0x4e, 0x11, 0x20, 0x20, 0x46, 0x4f, 0x52, 0x20, 0x58,
        0x20, 0x3d, 0x20, 0x30, 0x20, 0x54, 0x4f, 0x20, 0x31, 0x35, 0x0b, 0x20, 0x20, 0x20, 0x20,
        0x50, 0x52, 0x49, 0x4e, 0x54, 0x20, 0x58, 0x1d, 0x20, 0x20, 0x4e, 0x45, 0x58, 0x54, 0x20,
        0x58, 0x20, 0x20, 0x20, 0x27, 0x20, 0x54, 0x52, 0x41, 0x49, 0x4c, 0x49, 0x4e, 0x47, 0x20,
        0x43, 0x4f, 0x4d, 0x4d, 0x45, 0x4e, 0x54,
    ];

    unsafe {
        lcopy(DATA.as_ptr() as u32, 0x8010000, DATA.len() as u16);
    }

    // functional style, yeah!
    // DATA.iter()
    //     .copied()
    //     .enumerate()
    //     .for_each(|(offset, byte)| unsafe { lpoke(0x8010000u32 + offset as u32, byte) });
}

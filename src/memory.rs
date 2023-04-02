use mos_hardware::mega65::lpeek;

/// Never-ending iterator to lpeek into 28-bit memory
///
/// # Examples
/// ~~~
/// const ADDRESS: u32 = 0x8010000;
/// let mem = MemoryIterator::new(ADDRESS);
/// assert_eq!(mem.address, ADDRESS);
/// let byte: u8 = mem.next().unwrap();
/// assert_eq!(mem.address, ADDRESS + 1);
/// assert_eq!(mem.value, byte);
/// for byte in mem.take(4) {
///     println!("{}", byte);
/// }
/// assert_eq!(mem.address, ADDRESS + 1 + 4);
/// ~~~
pub struct MemoryIterator {
    /// Current 28 bit address
    address: u32,
    /// Current value at address; updated by `new()` or `next()`.
    value: u8,
}

impl MemoryIterator {
    pub fn new(address: u32) -> Self {
        Self {
            address: address,
            value: lpeek(address),
        }
    }
}

impl Iterator for MemoryIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.address += 1;
        self.value = lpeek(self.address);
        Some(self.value)
    }
}

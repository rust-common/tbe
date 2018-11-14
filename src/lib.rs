fn for_each_size<F>(mut size: u8, mut f: F)
where F: FnMut(u8, u8) {
    let mut offset = 0;
    while size >= 8 {
        f(offset, 8);
        offset += 8;
        size -= 8;
    }
    if size > 0 {
        f(offset, size)
    }
}

pub trait BitWrite {
    fn write(&mut self, value: u8, size: u8);
    // Little-endian.
    fn write32(&mut self, value: u32, size: u8) {
        for_each_size(
            size,
            |o, s| { self.write((value >> o) as u8, s); }
        )
    }
}

trait BitRead {
    fn read(&mut self, size: u8) -> u8;
    // Little-endian.
    fn read32(&mut self, size: u8) -> u32 {
        let mut result: u32 = 0;
        for_each_size(
            size,
            |o, s| { result += (self.read(s) as u32) << o; }
        );
        result
    }
}

/// floor(log(2, v))
///
/// Examples
///
/// ```rust
/// assert_eq!(tbe::floor_log2(0), 0);
/// assert_eq!(tbe::floor_log2(1), 0);
/// assert_eq!(tbe::floor_log2(2), 1);
/// assert_eq!(tbe::floor_log2(3), 1);
/// assert_eq!(tbe::floor_log2(4), 2);
/// assert_eq!(tbe::floor_log2(8), 3);
/// ```
pub fn floor_log2(mut v: u32) -> u32 {
    let mut result = 0;
    loop {
        v /= 2;
        if v == 0 { break; }
        result += 1;
    }
    result
}

/// 2^p
///
/// Examples
///
/// ```rust
/// assert_eq!(tbe::pow2(0), 1);
/// assert_eq!(tbe::pow2(1), 2);
/// assert_eq!(tbe::pow2(2), 4);
/// assert_eq!(tbe::pow2(3), 8);
/// ```
pub fn pow2(p: u32) -> u32 {
    1 << p
}

pub struct Tbe {
    k: u32,
    u: u32,
}

impl Tbe {
    pub fn new(n: u32) -> Self {
        let k = floor_log2(n);
        Self { k: k, u: 2 * k - n }
    }
    /*
    pub fn write(&self, w: &mut BitWrite, value: u32) {

    }
    pub fn read(&self, r: &mut BitRead) -> u32 {

    }
    */
}

/*
fn write(write: &mut BitWrite, max: u8, value: u8) {
    // write.write(value: u8, size: u8)
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_write() {
    }
}

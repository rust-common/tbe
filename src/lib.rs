use std::io::Read;
use std::io::Write;

fn fold_size<R>(
    mut size: u8,
    f: &mut FnMut(u8, u8, R) -> std::io::Result<R>,
    mut result: R
) -> std::io::Result<R> {
    let mut offset = 0;
    while size >= 8 {
        result = f(offset, 8, result)?;
        offset += 8;
        size -= 8;
    }
    if size > 0 {
        result = f(offset, size, result)?;
    }
    Ok(result)
}

pub trait BitWrite {
    fn write(&mut self, value: u8, size: u8) -> std::io::Result<()>;
    // Little-endian.
    fn write32(&mut self, value: u32, size: u8) -> std::io::Result<()> {
        fold_size(
            size,
            &mut |o, s, _| self.write((value >> o) as u8, s),
            ()
        )
    }
}

pub trait BitRead {
    fn read(&mut self, size: u8) -> std::io::Result<u8>;
    // Little-endian.
    fn read32(&mut self, size: u8) -> std::io::Result<u32> {
        fold_size(
            size,
            &mut |o, s, r| Ok(r | ((self.read(s)? as u32) << o)),
            0
        )
    }
}

struct BitWriteAdapter<'t> {
    w: &'t mut Write,
    buffer: u8,
    // 0..7
    size: u8,
}

impl<'t> BitWriteAdapter<'t> {
    fn flush(&mut self) -> std::io::Result<()> {
        self.w.write_all(&[self.buffer])
    }
}

impl<'t> BitWrite for BitWriteAdapter<'t> {
    // size is in [0..8]
    fn write(&mut self, value: u8, size: u8) -> std::io::Result<()> {
        self.buffer |= value << self.size;
        self.size += size;
        if self.size >= 8 {
            self.flush()?;
            self.size -= 8;
            self.buffer = value >> (size - self.size)
        }
        Ok(())
    }
}

/// Creates a `BitWrite` object and pass it to the given scope function `f`.
///
/// ```
/// let mut v = vec![];
/// {
///     let mut c = std::io::Cursor::new(&mut v);
///     tbe::with_bit_writer(&mut c, &mut |w| {
///         w.write(0, 0); //  0
///         w.write(1, 1); //  1
///         w.write(2, 2); //  3
///         w.write(3, 3); //  6
///         w.write(4, 4); // 10
///         w.write(5, 5); // 15
///         w.write(6, 6); // 21
///         Ok(())
///     });
/// }
/// assert_eq!(v, [0b00_011_10_1, 0b0_00101_01, 0b00011]);
/// ```
pub fn with_bit_writer<R>(
    w: &mut Write,
    f: &mut Fn(&mut BitWrite) -> std::io::Result<R>
) -> std::io::Result<R> {
    let mut adapter = BitWriteAdapter { w: w, buffer: 0, size: 0 };
    let result = f(&mut adapter)?;
    if adapter.size > 0 {
        adapter.flush()?;
    }
    Ok(result)
}

/// Provides `BitRead` from a `Read`.
///
/// ```
/// use tbe::BitRead;
/// let mut c = std::io::Cursor::new(&[0b00_11_10_1_0, 0b1_110_101_1, 0b1101000]);
/// let mut r = tbe::BitReadAdapter::new(&mut c);
/// assert_eq!(r.read(0).unwrap(), 0);
/// assert_eq!(r.read(1).unwrap(), 0);
/// assert_eq!(r.read(1).unwrap(), 1);
/// assert_eq!(r.read(2).unwrap(), 2);
/// assert_eq!(r.read(2).unwrap(), 3);
/// assert_eq!(r.read(3).unwrap(), 4);
/// assert_eq!(r.read(3).unwrap(), 5);
/// assert_eq!(r.read(3).unwrap(), 6);
/// assert_eq!(r.read(8).unwrap(), 0b11010001);
/// ```
pub struct BitReadAdapter<'t> {
    r: &'t mut Read,
    buffer: u8,
    // 0..7
    size: u8,
}

impl<'t> BitReadAdapter<'t> {
    pub fn new(r: &'t mut Read) -> Self {
        Self { r: r, buffer: 0, size: 0 }
    }
}

impl<'t> BitRead for BitReadAdapter<'t> {
    fn read(&mut self, size: u8) -> std::io::Result<u8> {
        let b16 = if self.size >= size {
            self.buffer as u16
        } else {
            let mut b = [0];
            self.r.read_exact(&mut b)?;
            let result = ((b[0] as u16) << self.size) | (self.buffer as u16);
            self.size += 8;
            result
        };
        self.size -= size;
        self.buffer = (b16 >> size) as u8;
        Ok((b16 & mask(size as u32) as u16) as u8)
    }
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

extern crate base2;
extern crate int;
extern crate bitrw;

use base2::Base2;
use int::UInt;
use bitrw::BitRead;
use bitrw::BitWrite;

pub struct TbeStruct<T> {
    k: u8,
    u: T,
}

pub trait Tbe: Sized {
    fn tbe(self) -> TbeStruct<Self>;
}

impl<T> Tbe for T
    where T: Base2 + UInt
{
    /// ```
    /// use tbe::Tbe;
    ///
    /// 3_u8.tbe();
    /// ```
    fn tbe(self) -> TbeStruct<Self> {
        let k = self.floor_log2();
        let ek2 = Self::exp2(k + 1);
        TbeStruct { k: k, u: ek2 - self }
    }
}

pub trait TbeRead {
    fn read_tbe<T>(&mut self, tbe: TbeStruct<T>) -> std::io::Result<T>
        where T: UInt;
}

impl<R> TbeRead for R
    where R: BitRead
{
    /// ```
    /// extern crate bitrw;
    /// extern crate tbe;
    /// use tbe::Tbe;
    /// use tbe::TbeRead;
    /// use bitrw::BitRead;
    /// use bitrw::ReadBits;
    /// let v = [0b0_11_01_0_0_1_u8, 0b11_0];
    /// let mut c = std::io::Cursor::new(&v);
    /// let mut r = c.read_bits();
    /// assert_eq!(r.read_tbe(1_u8.tbe()).unwrap(), 0);
    /// assert_eq!(r.read_tbe(2_u8.tbe()).unwrap(), 1);
    /// assert_eq!(r.read_tbe(2_u8.tbe()).unwrap(), 0);
    /// assert_eq!(r.read_tbe(3_u8.tbe()).unwrap(), 0);
    /// assert_eq!(r.read_tbe(3_u8.tbe()).unwrap(), 1);
    /// assert_eq!(r.read_tbe(3_u8.tbe()).unwrap(), 2);
    /// assert_eq!(r.read_tbe(4_u8.tbe()).unwrap(), 0);
    /// assert_eq!(r.read_tbe(4_u8.tbe()).unwrap(), 3);
    /// ```
    fn read_tbe<T>(&mut self, tbe: TbeStruct<T>) -> std::io::Result<T>
        where T: UInt
    {
        let v = self.read(tbe.k)?;
        Ok(if v < tbe.u {
            v
        } else {
            ((v << 1_u8) | self.read(1)?) - tbe.u
        })
    }
}

pub trait TbeWrite {
    fn write_tbe<T>(&mut self, tbe: TbeStruct<T>, v: T) -> std::io::Result<()>
        where T: UInt;
}

impl<W> TbeWrite for W where W: BitWrite {
    /// ```
    /// extern crate bitrw;
    /// extern crate tbe;
    /// use tbe::Tbe;
    /// use tbe::TbeWrite;
    /// use bitrw::BitRead;
    /// use bitrw::WriteBits;
    /// let mut v = vec![];
    /// {
    ///     std::io::Cursor::new(&mut v).write_bits(&mut|w| {
    ///         w.write_tbe(1_u8.tbe(), 0);
    ///         w.write_tbe(2_u8.tbe(), 1);
    ///         w.write_tbe(2_u8.tbe(), 0);
    ///         w.write_tbe(5_u8.tbe(), 0);
    ///         w.write_tbe(5_u8.tbe(), 1);
    ///         w.write_tbe(5_u8.tbe(), 2);
    ///         w.write_tbe(5_u8.tbe(), 3);
    ///         Ok(())
    ///     });
    /// }
    /// assert_eq!(&v, &[0b10_01_00_01, 0b011]);
    /// ```
    fn write_tbe<T>(&mut self, tbe: TbeStruct<T>, v: T) -> std::io::Result<()>
        where T: UInt
    {
        if v < tbe.u {
            self.write(v, tbe.k)
        } else {
            let x = v + tbe.u;
            self.write(x >> 1_u8, tbe.k)?;
            self.write(x, 1)
        }
    }
}
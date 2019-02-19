extern crate base2;
extern crate int;
extern crate bitrw;

pub struct TbeStruct<T: Copy> {
    k: u8,
    u: T,
}

impl<T: Copy> TbeStruct<T> {
    pub fn get_k(&self) -> u8 { self.k }
    pub fn get_u(&self) -> T { self.u }
}

pub trait Tbe: int::UInt + base2::Base2 {
    fn tbe(self) -> TbeStruct<Self> {
        let k = self.floor_log2();
        let ek2 = Self::exp2(k + 1);
        TbeStruct { k: k, u: ek2 - self }
    }
}

impl<T: int::UInt + base2::Base2> Tbe for T {}

pub trait TbeRead {
    fn read_tbe<T: int::UInt>(&mut self, tbe: TbeStruct<T>) -> std::io::Result<T>;
}

impl TbeRead for bitrw::BitRead<'_> {
    /// ```
    /// extern crate bitrw;
    /// extern crate tbe;
    /// use tbe::Tbe;
    /// use tbe::TbeRead;
    /// use bitrw::BitRead;
    /// use bitrw::UseBitRead;
    /// let v = [0b0_11_01_0_0_1_u8, 0b11_0];
    /// let mut c = std::io::Cursor::new(&v);
    /// let mut r = c.use_bit_read();
    /// assert_eq!(r.read_tbe(1_u8.tbe()).unwrap(), 0);
    /// assert_eq!(r.read_tbe(2_u8.tbe()).unwrap(), 1);
    /// assert_eq!(r.read_tbe(2_u8.tbe()).unwrap(), 0);
    /// assert_eq!(r.read_tbe(3_u8.tbe()).unwrap(), 0);
    /// assert_eq!(r.read_tbe(3_u8.tbe()).unwrap(), 1);
    /// assert_eq!(r.read_tbe(3_u8.tbe()).unwrap(), 2);
    /// assert_eq!(r.read_tbe(4_u8.tbe()).unwrap(), 0);
    /// assert_eq!(r.read_tbe(4_u8.tbe()).unwrap(), 3);
    /// ```
    fn read_tbe<T: int::UInt>(&mut self, tbe: TbeStruct<T>) -> std::io::Result<T> {
        let v = self.read(tbe.k)?;
        Ok(if v < tbe.u {
            v
        } else {
            ((v << 1_u8) | self.read(1)?) - tbe.u
        })
    }
}

pub trait TbeWrite {
    fn write_tbe<T: int::UInt>(&mut self, tbe: TbeStruct<T>, v: T) -> std::io::Result<()>;
}

impl TbeWrite for bitrw::BitWrite<'_> {
    /// ```
    /// extern crate bitrw;
    /// extern crate tbe;
    /// use tbe::Tbe;
    /// use tbe::TbeWrite;
    /// use bitrw::BitRead;
    /// use bitrw::UseBitWrite;
    /// let mut v = vec![];
    /// {
    ///     std::io::Cursor::new(&mut v).use_bit_write(&mut|w| {
    ///         w.write_tbe(0_u8.tbe(), 0);
    ///         w.write_tbe(1_u8.tbe(), 0);
    ///         w.write_tbe(2_u8.tbe(), 1);
    ///         w.write_tbe(2_u8.tbe(), 0);
    ///         w.write_tbe(5_u8.tbe(), 0);
    ///         w.write_tbe(5_u8.tbe(), 1);
    ///         w.write_tbe(5_u8.tbe(), 2);
    ///         w.write_tbe(5_u8.tbe(), 3);
    ///         w.write_tbe(256_u16.tbe(), 0).unwrap();
    ///         Ok(())
    ///     });
    /// }
    /// assert_eq!(&v, &[0b10_01_00_01, 0b000_0000_011, 0b0]);
    /// ```
    fn write_tbe<T: int::UInt>(&mut self, tbe: TbeStruct<T>, v: T) -> std::io::Result<()> {
        if v < tbe.u {
            self.write(v, tbe.k)
        } else {
            let x = v + tbe.u;
            self.write(x >> 1_u8, tbe.k)?;
            self.write(x, 1)
        }
    }
}

use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Take;
use std::mem;
use std::slice;

pub struct DwarfReader<R>(pub R);

macro_rules! read_impl {
    ($name: ident, $typ: ty) => {
        #[allow(dead_code)]
        pub fn $name(&mut self) -> io::Result<$typ> {
            self.read_unsafe::<$typ>()
        }
    }
}

impl<R: io::BufRead> DwarfReader<R> {
    fn read_unsafe<T>(&mut self) -> io::Result<T> {
        let mut result: T = unsafe { mem::zeroed() };
        let slice: &mut [u8] = unsafe {
            let p: *mut u8 = mem::transmute(&mut result);
            slice::from_raw_parts_mut(p, mem::size_of::<T>())
        };
        try!(self.0.read_exact(slice));
        Ok(result)
    }

    read_impl!(read_u8, u8);
    read_impl!(read_u16, u16);
    read_impl!(read_u32, u32);
    read_impl!(read_u64, u64);
    read_impl!(read_i8, i8);
    read_impl!(read_i16, i16);
    read_impl!(read_i32, i32);
    read_impl!(read_i64, i64);

    pub fn read_uleb128(&mut self) -> io::Result<u64> {
        let mut shift: usize = 0;
        let mut result: u64 = 0;
        let mut byte: u8;
        loop {
            byte = try!(self.read_u8());
            result |= ((byte & 0x7F) as u64) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(result)
    }

    pub fn read_sleb128(&mut self) -> io::Result<i64> {
        let mut shift: usize = 0;
        let mut result: u64 = 0;
        let mut byte: u8;
        loop {
            byte = try!(self.read_u8());
            result |= ((byte & 0x7F) as u64) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        // sign-extend
        if shift < 8 * mem::size_of::<u64>() && (byte & 0x40) != 0 {
            result |= (!0 as u64) << shift;
        }
        Ok(result as i64)
    }

    pub fn read_utf8(&mut self) -> io::Result<Vec<u8>> {
        let mut vec = Vec::new();
        try!(self.0.read_until(0, &mut vec));
        Ok(vec)
    }

    pub fn take(self, limit: u64) -> DwarfReader<Take<R>>
        where Self: Sized
    {
        DwarfReader(self.0.take(limit))
    }
}

impl<R: io::Read> io::Read for DwarfReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    KeySizeError,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::KeySizeError => return write!(f, "KeySizeError"),
        };
    }
}

pub struct Cipher {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Cipher {
    pub fn new(key: &[u8]) -> Result<Cipher, Error> {
        let k = key.len();
        #[allow(clippy::manual_range_contains)]
        if k < 1 || k > 256 {
            return Err(Error::KeySizeError);
        };
        let mut c = Cipher {
            s: [0; 256],
            i: 0,
            j: 0,
        };
        for i in 0..256 {
            c.s[i] = i as u8;
        }

        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.overflowing_add(c.s[i]).0.overflowing_add(key[i % k]).0;
            let sj = c.s[j as usize];
            let si = c.s[i];
            c.s[i] = sj;
            c.s[j as usize] = si;
        }
        Ok(c)
    }

    pub fn xor(&mut self, src: &[u8], dst: &mut [u8]) {
        let (mut i, mut j) = (self.i, self.j);
        for (k, v) in src.iter().enumerate() {
            i = i.overflowing_add(1).0;
            let x = self.s[i as usize];
            j = j.overflowing_add(x).0;
            let y = self.s[j as usize];
            self.s[i as usize] = y;
            self.s[j as usize] = x;
            dst[k] = v ^ self.s[x.overflowing_add(y).0 as usize];
        }
        self.i = i;
        self.j = j;
    }
}

pub struct Reader<T: std::io::Read> {
    reader: T,
    cipher: Cipher,
}

impl<T: std::io::Read> Reader<T> {
    pub fn new(r: T, key: &[u8]) -> Result<Self, Error> {
        let cipher = Cipher::new(key)?;
        let reader = Reader { reader: r, cipher };
        Ok(reader)
    }
}

impl<T: std::io::Read> std::io::Read for Reader<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut src: Vec<u8> = vec![0; buf.len()];
        let n = self.reader.read(&mut src[..])?;
        self.cipher.xor(&src[..n], &mut buf[..n]);
        Ok(n)
    }
}

use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    data: R,
    read_calls: usize,
    bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: _wrapped,
            bytes_through: 0,
            read_calls: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.read_calls
        // todo!()
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.data.read(buf)?;
        self.read_calls += 1;
        self.bytes_through += n;
        Ok(n)
    }
}

pub struct WriteStats<W> {
    data: W,
    write_calls: usize,
    bytes_through: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: _wrapped,
            write_calls: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
        // todo!()
    }

    pub fn writes(&self) -> usize {
        self.write_calls
        // todo!()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n=self.data.write(buf)?;
        self.write_calls+=1;
        self.bytes_through+=n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
        // todo!()
    }
}

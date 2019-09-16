use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    bytes_size: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped: wrapped,
            bytes_size: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_size
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.wrapped.read(buf)?;
        self.reads += 1;
        self.bytes_size += bytes_read;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_size: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped: wrapped,
            bytes_size: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_size
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.wrapped.write(buf)?;
        self.writes += 1;
        self.bytes_size += bytes_written;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}

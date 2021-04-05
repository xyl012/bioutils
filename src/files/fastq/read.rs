// Copyright 2021 Christopher Sugai



/// Read a reference with seq_io, create a suffix array with rust bio


use std::io::{self, Cursor, Read};
use seq_io::fastq::Reader;

struct IteratorAsRead<I>
where
    I: Iterator,
{
    iter: I,
    cursor: Option<Cursor<I::Item>>,
}

impl<I> IteratorAsRead<I>
where
    I: Iterator,
{
    pub fn new<T>(iter: T) -> Self
    where
        T: IntoIterator<IntoIter = I, Item = I::Item>,
    {
        let mut iter = iter.into_iter();
        let cursor = iter.next().map(Cursor::new);
        IteratorAsRead { iter, cursor }
    }
}

impl<I> Read for IteratorAsRead<I>
where
    I: Iterator,
    Cursor<I::Item>: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        while let Some(ref mut cursor) = self.cursor {
            let read = cursor.read(buf)?;
            if read > 0 {
                return Ok(read);
            }
            self.cursor = self.iter.next().map(Cursor::new);
        }
        Ok(0)
    }
}


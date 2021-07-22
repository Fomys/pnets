use std::error::Error;
use std::io::BufRead;

use crate::parser::Position;
use crate::ParserError;

/// Reader is an utf-8 reader.
pub struct Reader<R: BufRead> {
    reader: R,
    pub current_position: Position,
    pub next_position: Position,
    next_char: Option<char>,
}

impl<R: BufRead> Reader<R> {
    /// Create a new Reader for a Read object.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_position: Position { line: 1, column: 0 },
            next_position: Position { line: 1, column: 1 },
            next_char: None,
        }
    }

    /// Read next u8 from reader
    ///
    /// Can return `Error::IoError` if the reader return an error
    fn read_u8(&mut self) -> Result<u8, Box<dyn Error>> {
        let mut c = [0_u8];
        if self.reader.read(&mut c)? == 0 {
            Ok(0)
        } else {
            Ok(c[0])
        }
    }

    /// Read next utf8 char
    ///
    /// Can return [`Error::IoError`] if the reader return an error
    fn read_utf8(&mut self) -> Result<char, Box<dyn Error>> {
        // Convert u8 to char
        match match self.read_u8()? {
            c if c <= 0b1000_0000 => {
                // ASCII char
                Some(c as char)
            }
            c if (c & 0b1110_0000) == 0b1100_0000 => {
                // 2 bytes
                char::from_u32(
                    ((u32::from(c) & 0b0001_1111) << 6)
                        | (u32::from(self.read_u8()?) & 0b0011_1111),
                )
            }
            c if (c & 0b1111_0000) == 0b1110_0000 => {
                // 3 bytes
                char::from_u32(
                    ((u32::from(c) & 0b0000_1111) << 12)
                        | (u32::from(self.read_u8()?) & 0b0011_1111) << 6
                        | (u32::from(self.read_u8()?) & 0b0011_1111),
                )
            }
            c if (c & 0b1111_1000) == 0b1111_0000 => {
                // 4 bytes
                char::from_u32(
                    ((u32::from(c) & 0b0000_0111) << 18)
                        | (u32::from(self.read_u8()?) & 0b0011_1111) << 12
                        | (u32::from(self.read_u8()?) & 0b0011_1111) << 6
                        | (u32::from(self.read_u8()?) & 0b0011_1111),
                )
            }
            _ => return Err(Box::new(ParserError::Utf8Error(self.current_position))),
        } {
            None => Err(Box::new(ParserError::Utf8Error(self.current_position))),
            Some(c) => Ok(c),
        }
    }

    /// Increment position according to character pased in parameters
    fn increment_position(&mut self, ch: char) {
        self.current_position = self.next_position;
        if ch == '\n' {
            self.next_position.line += 1;
            self.next_position.column = 0;
        }
        self.next_position.column += 1;
    }

    /// Read next char from reader
    ///
    /// If this char was read with `peek` it will be used.
    pub fn read(&mut self) -> Result<char, Box<dyn Error>> {
        let ch = self.peek()?;
        self.next_char = None;
        self.increment_position(ch);
        Ok(ch)
    }

    /// Peek next character from reader
    ///
    /// I this char exist in buffer, it will be used
    pub fn peek(&mut self) -> Result<char, Box<dyn Error>> {
        match self.next_char {
            None => self.next_char = Some(self.read_utf8()?),
            Some(_) => (),
        }
        Ok(self.next_char.unwrap())
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::reader::Reader;

    #[test]
    fn read_ascii_test() {
        for i in 0..127 {
            match char::from_u32(i as u32) {
                None => (),
                Some(c) => {
                    let tmp = c.to_string();
                    let mut reader: Reader<&[u8]> = Reader::new(tmp.as_ref());
                    let result = reader.read();
                    assert_eq!(result.is_ok(), true);
                    assert_eq!(result.unwrap(), c);
                }
            }
        }
    }

    #[test]
    fn read_two_bytes_test() {
        for i in 128..0xFFFF {
            match char::from_u32(i as u32) {
                None => (),
                Some(c) => {
                    let tmp = c.to_string();
                    let mut reader: Reader<&[u8]> = Reader::new(tmp.as_ref());
                    let result = reader.read();
                    assert_eq!(result.is_ok(), true);
                    assert_eq!(result.unwrap(), c);
                }
            }
        }
    }

    #[test]
    fn read_three_bytes_test() {
        for i in 0xFFFF..0xFFFFFF {
            match char::from_u32(i as u32) {
                None => (),
                Some(c) => {
                    let tmp = c.to_string();
                    let mut reader: Reader<&[u8]> = Reader::new(tmp.as_ref());
                    let result = reader.read();
                    assert_eq!(result.is_ok(), true);
                    assert_eq!(result.unwrap(), c);
                }
            }
        }
    }

    #[test]
    fn read_four_bytes_test() {
        for i in 0xFFFFFF..0x0FFFFFFF {
            match char::from_u32(i as u32) {
                None => (),
                Some(c) => {
                    let tmp = c.to_string();
                    let mut reader: Reader<&[u8]> = Reader::new(tmp.as_ref());
                    let result = reader.read();
                    assert_eq!(result.is_ok(), true);
                    assert_eq!(result.unwrap(), c);
                }
            }
        }
    }
}

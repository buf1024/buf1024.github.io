//! Provides a type representing a Redis protocol frame as well as utilities for
//! parsing frames from a byte array.


use bytes::{Bytes};
use std::fmt;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;
use nom::{IResult};
use nom::bytes::streaming::{take_until1, tag};
use nom::sequence::{delimited, terminated};
use std::str;
use nom::combinator::{map_res, rest, map};
use nom::branch::alt;
use nom::multi::{length_value, length_count};

/// A frame in the Redis protocol.
/// 协议格式
#[derive(Clone, Debug)]
pub enum Frame {
    // +xxx\r\n 简单的string
    Simple(String),
    // -xxx\r\n 错误的string
    Error(String),
    // :ddd\r\n u64数值
    Integer(u64),
    // $dd\r\nbbbbb\r\n 有内容的chunk
    Bulk(Bytes),
    // $-1\r\n 空Chunk
    Null,
    // *dd\r\nxxx\r\n array dd 我数量
    Array(Vec<Frame>),
}

#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    Incomplete,

    /// Invalid message encoding
    Other(crate::Error),
}

impl Frame {
    /// Returns an empty array
    pub(crate) fn array() -> Frame {
        Frame::Array(vec![])
    }

    /// Push a "bulk" frame into the array. `self` must be an Array frame.
    ///
    /// # Panics
    ///
    /// panics if `self` is not an array
    pub(crate) fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Bulk(bytes));
            }
            _ => panic!("not an array frame"),
        }
    }

    /// Push an "integer" frame into the array. `self` must be an Array frame.
    ///
    /// # Panics
    ///
    /// panics if `self` is not an array
    pub(crate) fn push_int(&mut self, value: u64) {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Integer(value));
            }
            _ => panic!("not an array frame"),
        }
    }

    /// Checks if an entire message can be decoded from `src`
    // pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
    //     match get_u8(src)? {
    //         b'+' => {
    //             get_line(src)?;
    //             Ok(())
    //         }
    //         b'-' => {
    //             get_line(src)?;
    //             Ok(())
    //         }
    //         b':' => {
    //             let _ = get_decimal(src)?;
    //             Ok(())
    //         }
    //         b'$' => {
    //             if b'-' == peek_u8(src)? {
    //                 // Skip '-1\r\n'
    //                 skip(src, 4)
    //             } else {
    //                 // Read the bulk string
    //                 let len: usize = get_decimal(src)?.try_into()?;
    //
    //                 // skip that number of bytes + 2 (\r\n).
    //                 skip(src, len + 2)
    //             }
    //         }
    //         b'*' => {
    //             let len = get_decimal(src)?;
    //
    //             for _ in 0..len {
    //                 Frame::check(src)?;
    //             }
    //
    //             Ok(())
    //         }
    //         actual => Err(format!("protocol error; invalid frame type byte `{}`", actual).into()),
    //     }
    // }

    /// The message has already been validated with `check`.
    // pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
    //     match get_u8(src)? {
    //         b'+' => {
    //             // Read the line and convert it to `Vec<u8>`
    //             let line = get_line(src)?.to_vec();
    //
    //             // Convert the line to a String
    //             let string = String::from_utf8(line)?;
    //
    //             Ok(Frame::Simple(string))
    //         }
    //         b'-' => {
    //             // Read the line and convert it to `Vec<u8>`
    //             let line = get_line(src)?.to_vec();
    //
    //             // Convert the line to a String
    //             let string = String::from_utf8(line)?;
    //
    //             Ok(Frame::Error(string))
    //         }
    //         b':' => {
    //             let len = get_decimal(src)?;
    //             Ok(Frame::Integer(len))
    //         }
    //         b'$' => {
    //             if b'-' == peek_u8(src)? {
    //                 let line = get_line(src)?;
    //
    //                 if line != b"-1" {
    //                     return Err("protocol error; invalid frame format".into());
    //                 }
    //
    //                 Ok(Frame::Null)
    //             } else {
    //                 // Read the bulk string
    //                 let len = get_decimal(src)?.try_into()?;
    //                 let n = len + 2;
    //
    //                 if src.remaining() < n {
    //                     return Err(Error::Incomplete);
    //                 }
    //
    //                 let data = Bytes::copy_from_slice(&src.chunk()[..len]);
    //
    //                 // skip that number of bytes + 2 (\r\n).
    //                 skip(src, n)?;
    //
    //                 Ok(Frame::Bulk(data))
    //             }
    //         }
    //         b'*' => {
    //             let len = get_decimal(src)?.try_into()?;
    //             let mut out = Vec::with_capacity(len);
    //
    //             for _ in 0..len {
    //                 out.push(Frame::parse(src)?);
    //             }
    //
    //             Ok(Frame::Array(out))
    //         }
    //         _ => unimplemented!(),
    //     }
    // }

    /// Converts the frame to an "unexpected frame" error
    pub(crate) fn to_error(&self) -> crate::Error {
        format!("unexpected frame: {}", self).into()
    }


    fn parse_simple(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) = delimited(tag("+"), take_until1("\r\n"), tag("\r\n"))(src)?;
        Ok((input, (Frame::Simple(String::from(output)), src.len() - input.len())))
    }

    fn parse_error(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) = delimited(tag("-"), take_until1("\r\n"), tag("\r\n"))(src)?;
        Ok((input, (Frame::Error(String::from(output)), src.len() - input.len())))
    }

    fn parse_decimal(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) = map_res(
            delimited(tag(":"), take_until1("\r\n"), tag("\r\n")),
            |s: &str| u64::from_str_radix(s, 10),
        )(src)?;
        Ok((input, (Frame::Integer(output), src.len() - input.len())))
    }

    fn parse_bulk(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) = alt((
            map(tag("$-1\r\n"), |_| Frame::Null),
            map(terminated(length_value(
                map_res(
                    delimited(tag("$"), take_until1("\r\n"), tag("\r\n")),
                    |s: &str| u64::from_str_radix(s, 10)),
                rest),
                           tag("\r\n"),
            ), |out| {
                let data = Bytes::copy_from_slice(out.as_bytes());
                Frame::Bulk(data)
            }))
        )(src)?;
        Ok((input, (output, src.len() - input.len())))
    }

    fn parse_array(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) =
            map(length_count(
                map_res(
                    delimited(tag("*"), take_until1("\r\n"), tag("\r\n")),
                    |s: &str| {
                        u64::from_str_radix(s, 10)
                    }),
                Frame::parse,
            ), |out| {
                let data = out.iter().map(|item| item.0.clone()).collect();
                Frame::Array(data)
            },
            )(src)?;
        Ok((input, (output, src.len() - input.len())))
    }

    pub fn parse(src: &str) -> IResult<&str, (Frame, usize)>
    {
        alt((Frame::parse_simple, Frame::parse_error, Frame::parse_decimal, Frame::parse_bulk, Frame::parse_array))(src)
    }
}


#[test]
fn test_parse() {
    use bytes::{Buf, BytesMut};

    let mut buffer = BytesMut::with_capacity(4 * 1024);
    buffer.extend_from_slice(b"*3\r\n$3\r\nset\r\n$3\r\nabc\r\n$3\r\n123\r\n");
    let string = str::from_utf8(buffer.as_ref()).unwrap();
    let res = Frame::parse(string);
    match &res {
        Ok((_, output)) => {
            let (frame, size) = output;
            println!("frame: {:?}, n: {}", *frame, *size);

            buffer.advance(*size);
            let vec = buffer.clone().to_vec();
            println!("remain: {:?}", String::from_utf8(vec))
        }
        Err(err) => {
            match err {
                nom::Err::Incomplete(_) => {
                    println!("incomplete");
                }
                nom::Err::Error(_) => {
                    println!("error");
                }
                nom::Err::Failure(_) => {
                    println!("failure");
                }
            }
        }
    }
}


impl PartialEq<&str> for Frame {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Frame::Simple(s) => s.eq(other),
            Frame::Bulk(s) => s.eq(other),
            _ => false,
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Frame::Simple(response) => response.fmt(fmt),
            Frame::Error(msg) => write!(fmt, "error: {}", msg),
            Frame::Integer(num) => num.fmt(fmt),
            Frame::Bulk(msg) => match str::from_utf8(msg) {
                Ok(string) => string.fmt(fmt),
                Err(_) => write!(fmt, "{:?}", msg),
            },
            Frame::Null => "(nil)".fmt(fmt),
            Frame::Array(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(fmt, " ")?;
                        part.fmt(fmt)?;
                    }
                }

                Ok(())
            }
        }
    }
}

// fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
//     if !src.has_remaining() {
//         return Err(Error::Incomplete);
//     }
//
//     Ok(src.chunk()[0])
// }

// fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
//     if !src.has_remaining() {
//         return Err(Error::Incomplete);
//     }
//
//     Ok(src.get_u8())
// }

// fn skip(src: &mut Cursor<&[u8]>, n: usize) -> Result<(), Error> {
//     if src.remaining() < n {
//         return Err(Error::Incomplete);
//     }
//
//     src.advance(n);
//     Ok(())
// }

/// Read a new-line terminated decimal
// fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<u64, Error> {
//     use atoi::atoi;
//
//     let line = get_line(src)?;
//
//     atoi::<u64>(line).ok_or_else(|| "protocol error; invalid frame format".into())
// }

/// Find a line
// fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
//     // Scan the bytes directly
//     let start = src.position() as usize;
//     // Scan to the second to last byte
//     let end = src.get_ref().len() - 1;
//
//     for i in start..end {
//         if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
//             // We found a line, update the position to be *after* the \n
//             src.set_position((i + 2) as u64);
//
//             // Return the line
//             return Ok(&src.get_ref()[start..i]);
//         }
//     }
//
//     Err(Error::Incomplete)
// }

impl From<String> for Error {
    fn from(src: String) -> Error {
        Error::Other(src.into())
    }
}

impl From<&str> for Error {
    fn from(src: &str) -> Error {
        src.to_string().into()
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_src: FromUtf8Error) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => "stream ended early".fmt(fmt),
            Error::Other(err) => err.fmt(fmt),
        }
    }
}

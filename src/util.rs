use nom::bytes::complete::{take, take_while};
use nom::combinator::{map, map_parser, map_res};
use nom::error::{ErrorKind, ParseError};
use nom::multi::length_count;
use nom::number::complete::{le_u16, le_u32, le_u8};
use nom::{IResult, Parser};

pub(crate) fn bool32(input: &[u8]) -> IResult<&[u8], bool> {
    map(le_u32, |it| it != 0)(input)
}

pub(crate) fn read_utf8<'a>(input: &'a [u8]) -> IResult<&'a [u8], String> {
    match std::str::from_utf8(input) {
        Ok(data) => Ok((&input[0..0], String::from(data))),
        Err(_) => Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
            input,
            ErrorKind::MapRes,
        ))),
    }
}

pub(crate) fn read_tag<'a>(length: usize) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], String> {
    map_res(take(length), |data| {
        std::str::from_utf8(data).map(String::from)
    })
}

pub(crate) fn read_utf8_until_null(input: &[u8]) -> IResult<&[u8], String> {
    map_parser(take(8usize), map_parser(take_while(|c| c != 0), read_utf8))(input)
}

pub(crate) fn read_length_utf16(input: &[u8]) -> IResult<&[u8], String> {
    map_res(length_count(le_u8, le_u16), |it| {
        String::from_utf16(it.as_slice())
    })(input)
}

pub(crate) fn read_from_offsets<'a, O, E, A>(
    offsets: Vec<u32>,
    total_length: u32,
    mut f: A,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<O>, E>
where
    A: Parser<&'a [u8], O, E>,
    E: ParseError<&'a [u8]>,
{
    move |input: &[u8]| {
        let mut pieces = Vec::new();
        for offset in offsets.clone() {
            match f.parse(&input[offset as usize..total_length as usize]) {
                Ok((_, o)) => {
                    pieces.push(o);
                }
                Err(nom::Err::Error(e)) => {
                    return Err(nom::Err::Error(E::append(input, ErrorKind::Count, e)));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok((&input[total_length as usize..], pieces))
    }
}

use nom::combinator::{map, opt};
use nom::multi::{length_data, length_value};
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;
use nom::IResult;

use crate::file_variant::Variant;
use crate::generator::Generator;
use crate::util::{bool32, read_length_utf16, read_tag, read_utf8_until_null};
use crate::{file_variant, generator};

#[derive(Debug)]
pub struct FileHeader {
    pub variant: Variant,
    pub version: String,
    pub content_id: u32,
    pub short_name: String,
    pub long_name: String,
    pub author_name: String,
    pub copyright: String,
    pub thumbnail_block_size_width: u8,
    pub thumbnail_block_size_height: u8,
    pub thumbnail: Vec<u8>,
    pub generator: Generator,
    pub print_to_cut: Option<bool>,
}

pub(crate) fn read_file_header(input: &[u8]) -> IResult<&[u8], FileHeader> {
    map(
        tuple((
            // static header
            file_variant::read_variant,
            read_tag(4usize),
            le_u32,
            // dynamic header
            length_value(
                le_u32,
                tuple((
                    read_utf8_until_null,
                    read_length_utf16,
                    read_length_utf16,
                    read_length_utf16,
                    le_u8,
                    le_u8,
                    map(length_data(le_u32), Vec::from),
                    generator::read_generator,
                    opt(bool32),
                )),
            ),
        )),
        |(
            variant,
            version,
            content_id,
            (
                short_name,
                long_name,
                author_name,
                copyright,
                thumbnail_block_size_width,
                thumbnail_block_size_height,
                thumbnail,
                generator,
                print_to_cut,
            ),
        )| FileHeader {
            variant,
            version: version,
            content_id,
            short_name,
            long_name,
            author_name,
            copyright,
            thumbnail_block_size_width,
            thumbnail_block_size_height,
            thumbnail,
            generator,
            print_to_cut,
        },
    )(input)
}

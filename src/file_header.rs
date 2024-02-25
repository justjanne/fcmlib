use std::io::Write;

use nom::combinator::{map, opt};
use nom::multi::{length_data, length_value};
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;
use nom::IResult;

use crate::encode::Encode;
use crate::file_variant::FileVariant;
use crate::generator::Generator;
use crate::util::{bool32, read_length_utf16, read_tag, read_utf8_until_null};
use crate::{file_variant, generator, util};

#[derive(Debug)]
pub struct FileHeader {
    pub variant: FileVariant,
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
            version,
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

impl Encode for FileHeader {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        self.variant.encode(buffer)?;
        buffer.write(&self.version.as_bytes()[0..4])?;
        self.content_id.encode(buffer)?;

        let mut variable_header: Vec<u8> = vec![];

        util::write_utf8_fixed(&self.short_name, &mut variable_header)?;
        util::write_utf16_str(&self.long_name, &mut variable_header)?;
        util::write_utf16_str(&self.author_name, &mut variable_header)?;
        util::write_utf16_str(&self.copyright, &mut variable_header)?;

        self.thumbnail_block_size_width
            .encode(&mut variable_header)?;
        self.thumbnail_block_size_height
            .encode(&mut variable_header)?;
        (self.thumbnail.len() as u32).encode(&mut variable_header)?;
        variable_header.write(&self.thumbnail)?;

        self.generator.encode(&mut variable_header)?;
        if let Some(print_to_cut) = &self.print_to_cut {
            (*print_to_cut as u32).encode(&mut variable_header)?;
        }

        (variable_header.len() as u32).encode(buffer)?;
        buffer.write(&variable_header)?;

        Ok(())
    }
}

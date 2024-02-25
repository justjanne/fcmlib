use std::io::Write;

use nom::bytes::complete::take;
use nom::combinator::{cond, flat_map, map, map_res};
use nom::multi::{length_count, length_data, length_value};
use nom::number::complete::{le_f32, le_u32};
use nom::sequence::tuple;
use nom::IResult;

use crate::encode::Encode;
use crate::path::Path;
use crate::piece_restrictions::PieceRestrictions;
use crate::util::bool32;
use crate::{path, piece_restrictions};

#[derive(Debug)]
pub struct Piece {
    pub width: u32,
    pub height: u32,
    pub transform: Option<(f32, f32, f32, f32, f32, f32)>,
    pub expansion_limit_value: u32,
    pub reduction_limit_value: u32,
    pub restriction_flags: PieceRestrictions,
    pub label: String,
    pub paths: Vec<Path>,
}

pub(crate) fn read_piece(input: &[u8]) -> IResult<&[u8], Piece> {
    map(
        tuple((
            take(8usize),
            le_u32,
            le_u32,
            flat_map(bool32, |has_transform| {
                cond(
                    has_transform,
                    tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
                )
            }),
            le_u32,
            le_u32,
            piece_restrictions::read_piece_restrictions,
            read_piece_label,
            length_count(le_u32, length_value(le_u32, path::read_path)),
        )),
        |(
            _,
            width,
            height,
            transform,
            expansion_limit_value,
            reduction_limit_value,
            restriction_flags,
            label,
            paths,
        )| Piece {
            width,
            height,
            transform,
            expansion_limit_value,
            reduction_limit_value,
            restriction_flags,
            label,
            paths,
        },
    )(input)
}

fn read_piece_label(input: &[u8]) -> IResult<&[u8], String> {
    map_res(length_data(le_u32), |label_data: &[u8]| {
        if label_data[0] == 1 {
            std::str::from_utf8(&label_data[1..4]).map(String::from)
        } else {
            Ok(String::new())
        }
    })(input)
}

impl Encode for Piece {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        0u32.encode(buffer)?;
        0u32.encode(buffer)?;
        self.width.encode(buffer)?;
        self.height.encode(buffer)?;
        if let Some(transform) = self.transform {
            1u32.encode(buffer)?;
            transform.0.encode(buffer)?;
            transform.1.encode(buffer)?;
            transform.2.encode(buffer)?;
            transform.3.encode(buffer)?;
            transform.4.encode(buffer)?;
            transform.5.encode(buffer)?;
        } else {
            0u32.encode(buffer)?;
        }
        self.expansion_limit_value.encode(buffer)?;
        self.reduction_limit_value.encode(buffer)?;
        self.restriction_flags.encode(buffer)?;

        4u32.encode(buffer)?;
        if self.label.is_empty() {
            0u32.encode(buffer)?;
        } else {
            1u8.encode(buffer)?;
            buffer.write(&self.label.as_bytes()[0..3])?;
        }

        let mut path_data: Vec<Vec<u8>> = vec![];
        for path in &self.paths {
            path_data.push(path.encode_to_vec()?);
        }

        (path_data.len() as u32).encode(buffer)?;
        for path in path_data {
            (path.len() as u32).encode(buffer)?;
            buffer.write(&path)?;
        }

        Ok(())
    }
}

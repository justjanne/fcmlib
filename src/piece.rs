use bitflags::bitflags;
use nom::bytes::complete::take;
use nom::combinator::{cond, flat_map, map, map_opt, map_res};
use nom::multi::{length_count, length_data, length_value};
use nom::number::complete::{le_f32, le_u16, le_u32};
use nom::sequence::tuple;
use nom::IResult;

use crate::path;
use crate::path::Path;
use crate::util::{bool32, read_from_offsets};

pub fn read_piece_table(input: &[u8]) -> IResult<&[u8], Vec<(u16, Piece)>> {
    flat_map(
        tuple((
            length_count(le_u32, le_u32),
            le_u32,
            length_count(le_u32, le_u16),
        )),
        move |(offsets, total_length, ids)| {
            map(
                read_from_offsets(offsets, total_length, read_piece),
                move |pieces| ids.clone().into_iter().zip(pieces).collect(),
            )
        },
    )(input)
}

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
            read_piece_restrictions,
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

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct PieceRestrictions: u32 {
            const LICENSE_DESIGN = 0x0001;
            const SEAM_ALLOWANCE = 0x0002;
            const PROHIBITION_OF_SEAM_ALLOWANCE_SETTING = 0x0004;
            const NO_ASPECT_RATIO_CHANGE_PROHIBITED = 0x0020;
            const JUDGE_BY_USING_PERFECT_MASK_AT_AUTO_LAYOUT = 0x0020;
            const TEST_PATTERN = 0x0040;
            const PROHIBITION_OF_EDIT = 0x0080;
            const PROHIBITION_OF_TOOL = 0x0100;
            const _ = !0;
    }
}

fn read_piece_restrictions(input: &[u8]) -> IResult<&[u8], PieceRestrictions> {
    map_opt(le_u32, PieceRestrictions::from_bits)(input)
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

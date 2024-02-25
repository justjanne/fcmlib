use nom::combinator::{cond, flat_map, map};
use nom::multi::length_count;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom::IResult;

use crate::file_type::FileType;
use crate::point::Point;
use crate::util::bool32;
use crate::{file_type, point};

#[derive(Debug)]
pub struct CutData {
    pub file_type: FileType,
    pub mat_id: u32,
    pub cut_width: u32,
    pub cut_height: u32,
    pub seam_allowance_width: u32,
    pub align_needed: Option<bool>,
    pub align_marks: Option<Vec<Point>>,
}

pub(crate) fn read_cut_data(input: &[u8]) -> IResult<&[u8], CutData> {
    flat_map(file_type::read_file_type, |file_type| {
        map(
            tuple((
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                cond(file_type == FileType::PrintAndCut, bool32),
                cond(
                    file_type == FileType::PrintAndCut,
                    length_count(le_u32, point::read_point),
                ),
            )),
            move |(
                mat_id,
                cut_width,
                cut_height,
                seam_allowance_width,
                align_needed,
                align_marks,
            )| CutData {
                file_type,
                mat_id,
                cut_width,
                cut_height,
                seam_allowance_width,
                align_needed,
                align_marks,
            },
        )
    })(input)
}

use nom::combinator::{cond, flat_map, map};
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom::IResult;

use crate::alignment_data::{read_alignment_data, AlignmentData};
use crate::encode::Encode;
use crate::file_type;
use crate::file_type::FileType;

#[derive(Debug)]
pub struct CutData {
    pub file_type: FileType,
    pub mat_id: u32,
    pub cut_width: u32,
    pub cut_height: u32,
    pub seam_allowance_width: u32,
    pub alignment: Option<AlignmentData>,
}

pub(crate) fn read_cut_data(input: &[u8]) -> IResult<&[u8], CutData> {
    flat_map(file_type::read_file_type, |file_type| {
        map(
            tuple((
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                cond(file_type == FileType::PrintAndCut, read_alignment_data),
            )),
            move |(mat_id, cut_width, cut_height, seam_allowance_width, alignment)| CutData {
                file_type,
                mat_id,
                cut_width,
                cut_height,
                seam_allowance_width,
                alignment,
            },
        )
    })(input)
}

impl Encode for CutData {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        self.file_type.encode(buffer)?;
        self.mat_id.encode(buffer)?;
        self.cut_width.encode(buffer)?;
        self.cut_height.encode(buffer)?;
        self.seam_allowance_width.encode(buffer)?;
        if let Some(alignment) = &self.alignment {
            alignment.encode(buffer)?;
        }

        Ok(())
    }
}

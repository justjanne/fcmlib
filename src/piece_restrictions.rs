use std::io::Write;

use bitflags::bitflags;
use nom::combinator::map_opt;
use nom::number::complete::le_u32;
use nom::IResult;

use crate::encode::Encode;

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
pub(crate) fn read_piece_restrictions(input: &[u8]) -> IResult<&[u8], PieceRestrictions> {
    map_opt(le_u32, PieceRestrictions::from_bits)(input)
}

impl Encode for PieceRestrictions {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        buffer.write_all(&self.bits().to_le_bytes())?;
        Ok(())
    }
}

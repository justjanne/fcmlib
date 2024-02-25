use std::io::Write;

use bitflags::bitflags;
use nom::combinator::map_opt;
use nom::number::complete::le_u32;
use nom::IResult;

use crate::encode::Encode;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct PathTool: u32 {
            const PATH_OPEN = 0x0001;
            const TOOL_CUT = 0x0002;
            const TOOL_DRAW = 0x0004;
            const SEAM_ALLOWANCE = 0x0008;
            const TOOL_RHINESTONE = 0x0010;
            const FILL = 0x0020;
            const AUTO_ALIGN = 0x0040;
            const TOOL_DRAW_ONLY = 0x1000;
            const TOOL_EMBOSS = 0x2000;
            const TOOL_FOIL = 0x4000;
            const TOOL_PERFORATING = 0x8000;
            const _ = !0;
    }
}
pub fn read_path_tool(input: &[u8]) -> IResult<&[u8], PathTool> {
    map_opt(le_u32, PathTool::from_bits)(input)
}

impl Encode for PathTool {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        buffer.write_all(&4u32.to_le_bytes())?;
        buffer.write_all(&self.bits().to_le_bytes())?;
        Ok(())
    }
}

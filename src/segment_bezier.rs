use crate::encode::Encode;
use crate::point::{read_point, Point};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Copy, Clone)]
pub struct SegmentBezier {
    pub control1: Point,
    pub control2: Point,
    pub end: Point,
}

pub(crate) fn read_segment_bezier(input: &[u8]) -> IResult<&[u8], SegmentBezier> {
    map(
        tuple((read_point, read_point, read_point)),
        |(control1, control2, end)| SegmentBezier {
            control1,
            control2,
            end,
        },
    )(input)
}

impl Encode for SegmentBezier {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        self.control1.encode(buffer)?;
        self.control2.encode(buffer)?;
        self.end.encode(buffer)?;
        Ok(())
    }
}

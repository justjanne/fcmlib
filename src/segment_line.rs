use crate::point::{read_point, Point};
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, Copy, Clone)]
pub struct SegmentLine {
    pub end: Point,
}

pub(crate) fn read_segment_line(input: &[u8]) -> IResult<&[u8], SegmentLine> {
    map(read_point, |end| SegmentLine { end })(input)
}

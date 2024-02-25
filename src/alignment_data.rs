use crate::encode::Encode;
use crate::point;
use crate::point::Point;
use crate::util::bool32;
use nom::combinator::map;
use nom::multi::length_count;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct AlignmentData {
    pub needed: bool,
    pub marks: Vec<Point>,
}

pub(crate) fn read_alignment_data(input: &[u8]) -> IResult<&[u8], AlignmentData> {
    map(
        tuple((bool32, length_count(le_u32, point::read_point))),
        |(needed, marks)| AlignmentData { needed, marks },
    )(input)
}

impl Encode for AlignmentData {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        (self.needed as u32).encode(buffer)?;
        (self.marks.len() as u32).encode(buffer)?;
        for mark in &self.marks {
            mark.encode(buffer)?;
        }
        Ok(())
    }
}

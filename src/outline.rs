use std::io::Write;

use nom::combinator::{flat_map, map};
use nom::multi::length_count;
use nom::number::complete::le_u32;
use nom::IResult;

use crate::encode::Encode;
use crate::outline_tag::OutlineTag;
use crate::segment_bezier::SegmentBezier;
use crate::segment_line::SegmentLine;
use crate::{outline_tag, segment_bezier, segment_line};

#[derive(Debug, Clone)]
pub enum Outline {
    Line(Vec<SegmentLine>),
    Bezier(Vec<SegmentBezier>),
}

pub(crate) fn read_outline(input: &[u8]) -> IResult<&[u8], Outline> {
    flat_map(outline_tag::read_outline_tag, |tag| match tag {
        OutlineTag::Line => read_outline_line,
        OutlineTag::Bezier => read_outline_bezier,
    })(input)
}

fn read_outline_line(input: &[u8]) -> IResult<&[u8], Outline> {
    map(
        length_count(le_u32, segment_line::read_segment_line),
        |segments| Outline::Line(segments),
    )(input)
}

fn read_outline_bezier(input: &[u8]) -> IResult<&[u8], Outline> {
    map(
        length_count(le_u32, segment_bezier::read_segment_bezier),
        |segments| Outline::Bezier(segments),
    )(input)
}

impl Encode for Outline {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        match self {
            Outline::Line(segments) => {
                OutlineTag::Line.encode(buffer)?;
                buffer.write(&(segments.len() as u32).to_le_bytes())?;
                for segment in segments {
                    segment.encode(buffer)?;
                }
            }
            Outline::Bezier(segments) => {
                OutlineTag::Bezier.encode(buffer)?;
                buffer.write(&(segments.len() as u32).to_le_bytes())?;
                for segment in segments {
                    segment.encode(buffer)?;
                }
            }
        }
        Ok(())
    }
}

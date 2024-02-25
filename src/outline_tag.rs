use nom::combinator::map_res;
use nom::number::complete::le_u32;
use nom::IResult;

#[derive(Debug, Copy, Clone)]
pub(crate) enum OutlineTag {
    Line,
    Bezier,
}

pub(crate) fn read_outline_tag(input: &[u8]) -> IResult<&[u8], OutlineTag> {
    map_res(le_u32, |tag| match tag {
        0 => Ok(OutlineTag::Line),
        1 => Ok(OutlineTag::Bezier),
        _ => Err(nom::Err::Failure(format!("Unexpected outline tag: {tag}"))),
    })(input)
}

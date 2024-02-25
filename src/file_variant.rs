use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug)]
pub enum Variant {
    FCM,
    VCM,
}

pub(crate) fn read_variant(input: &[u8]) -> IResult<&[u8], Variant> {
    alt((
        map(tag("#FCM"), |_| Variant::FCM),
        map(tag("#VCM"), |_| Variant::VCM),
    ))(input)
}

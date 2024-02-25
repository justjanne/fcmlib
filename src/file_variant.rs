use std::io::Write;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

use crate::encode::Encode;

#[derive(Debug)]
pub enum FileVariant {
    FCM,
    VCM,
}

pub(crate) fn read_variant(input: &[u8]) -> IResult<&[u8], FileVariant> {
    alt((
        map(tag("#FCM"), |_| FileVariant::FCM),
        map(tag("#VCM"), |_| FileVariant::VCM),
    ))(input)
}

impl Encode for FileVariant {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        match self {
            FileVariant::FCM => buffer.write("#FCM".as_bytes())?,
            FileVariant::VCM => buffer.write("#VCM".as_bytes())?,
        };
        Ok(())
    }
}

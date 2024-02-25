use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub enum Generator {
    App(u32),
    Web(u32),
    Device(u32, u32),
}

pub(crate) fn read_generator(input: &[u8]) -> IResult<&[u8], Generator> {
    alt((
        read_generator_app,
        read_generator_web,
        read_generator_device,
    ))(input)
}

fn read_generator_app(input: &[u8]) -> IResult<&[u8], Generator> {
    map(tuple((tag("1APP"), le_u32)), |(_, version)| {
        Generator::App(version)
    })(input)
}

fn read_generator_web(input: &[u8]) -> IResult<&[u8], Generator> {
    map(tuple((tag("1WEB"), le_u32)), |(_, version)| {
        Generator::Web(version)
    })(input)
}

fn read_generator_device(input: &[u8]) -> IResult<&[u8], Generator> {
    map(tuple((le_u32, le_u32)), |(device, version)| {
        Generator::Device(device, version)
    })(input)
}

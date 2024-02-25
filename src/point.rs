use nom::combinator::map;
use nom::number::complete::le_i32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Default, Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub(crate) fn read_point(input: &[u8]) -> IResult<&[u8], Point> {
    map(tuple((le_i32, le_i32)), |(x, y)| Point { x, y })(input)
}

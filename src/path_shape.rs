use nom::combinator::{cond, map};
use nom::multi::count;
use nom::sequence::tuple;
use nom::IResult;

use crate::outline::{read_outline, Outline};
use crate::point::{read_point, Point};

#[derive(Debug)]
pub struct PathShape {
    pub start: Point,
    pub outlines: Vec<Outline>,
}

pub fn read_path_shape<'a>(
    outline_count: usize,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Option<PathShape>> {
    cond(
        outline_count > 0,
        map(
            tuple((read_point, count(read_outline, outline_count))),
            move |(start, outlines)| PathShape { start, outlines },
        ),
    )
}

use nom::combinator::{cond, map};
use nom::multi::count;
use nom::IResult;

use crate::point::{read_point, Point};

#[derive(Debug)]
pub struct PathRhinestones {
    pub diameter: u32,
    pub points: Vec<Point>,
}

pub fn read_path_rhinestones<'a>(
    rhinestone_count: usize,
    diameter: u32,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Option<PathRhinestones>> {
    cond(
        rhinestone_count > 0,
        map(
            count(read_point, rhinestone_count as usize),
            move |points| PathRhinestones { diameter, points },
        ),
    )
}

use std::io;
use std::io::Write;

use nom::combinator::{flat_map, map};
use nom::IResult;
use nom::multi::{count, length_value};
use nom::number::complete::le_u32;
use nom::sequence::tuple;

use crate::{path_shape, path_tool};
use crate::encode::Encode;
use crate::path_shape::PathShape;
use crate::path_tool::PathTool;
use crate::point::{Point, read_point};

#[derive(Debug)]
pub struct Path {
    pub tool: PathTool,
    pub shape: Option<PathShape>,
    pub rhinestone_diameter: Option<u32>,
    pub rhinestones: Vec<Point>,
}

fn read_rhinestone_diameter(input: &[u8]) -> IResult<&[u8], Option<u32>> {
    map(le_u32, |diameter| {
        if diameter == 0x3f000000 {
            None
        } else {
            Some(diameter)
        }
    })(input)
}

pub(crate) fn read_path(input: &[u8]) -> IResult<&[u8], Path> {
    flat_map(
        tuple((
            length_value(le_u32, path_tool::read_path_tool),
            le_u32,
            le_u32,
            read_rhinestone_diameter,
        )),
        |(tool, outline_count, rhinestone_count, rhinestone_diameter)| {
            map(
                tuple((
                    path_shape::read_path_shape(outline_count as usize),
                    count(read_point, rhinestone_count as usize),
                )),
                move |(shape, rhinestones)| Path {
                    tool,
                    shape,
                    rhinestone_diameter,
                    rhinestones,
                },
            )
        },
    )(input)
}

impl Encode for Path {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        self.tool.encode(buffer)?;
        buffer.write(
            &self
                .shape
                .as_ref()
                .map(|it| it.outlines.len() as u32)
                .unwrap_or_default()
                .to_le_bytes(),
        )?;
        (self.rhinestones.len() as u32).encode(buffer)?;
        if let Some(rhinestone_diameter) = &self.rhinestone_diameter {
            rhinestone_diameter.encode(buffer)?;
        } else {
            0x3f000000u32.encode(buffer)?;
        }
        if let Some(shape) = &self.shape {
            shape.start.encode(buffer)?;
            for outline in &shape.outlines {
                outline.encode(buffer)?;
            }
        }
        for rhinestone in &self.rhinestones {
            rhinestone.encode(buffer)?;
        }
        Ok(())
    }
}

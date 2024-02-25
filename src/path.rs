use std::io;
use std::io::Write;

use nom::combinator::{flat_map, map};
use nom::multi::length_value;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom::IResult;

use crate::encode::Encode;
use crate::path_rhinestones::PathRhinestones;
use crate::path_shape::PathShape;
use crate::path_tool::PathTool;
use crate::{path_rhinestones, path_shape, path_tool};

#[derive(Debug)]
pub struct Path {
    pub tool: PathTool,
    pub shape: Option<PathShape>,
    pub rhinestones: Option<PathRhinestones>,
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
        buffer.write(
            &self
                .rhinestones
                .as_ref()
                .map(|it| it.points.len() as u32)
                .unwrap_or_default()
                .to_le_bytes(),
        )?;
        buffer.write(
            &self
                .rhinestones
                .as_ref()
                .map(|it| it.diameter)
                .unwrap_or_default()
                .to_le_bytes(),
        )?;
        if let Some(shape) = &self.shape {
            shape.start.encode(buffer)?;
            for outline in &shape.outlines {
                outline.encode(buffer)?;
            }
        }
        if let Some(rhinestones) = &self.rhinestones {
            for rhinestone in &rhinestones.points {
                rhinestone.encode(buffer)?;
            }
        }
        Ok(())
    }
}

pub(crate) fn read_path(input: &[u8]) -> IResult<&[u8], Path> {
    flat_map(
        tuple((
            length_value(le_u32, path_tool::read_path_tool),
            le_u32,
            le_u32,
            le_u32,
        )),
        |(tool, outline_count, rhinestone_count, rhinestone_diameter)| {
            map(
                tuple((
                    path_shape::read_path_shape(outline_count as usize),
                    path_rhinestones::read_path_rhinestones(
                        rhinestone_count as usize,
                        rhinestone_diameter,
                    ),
                )),
                move |(shape, rhinestones)| Path {
                    tool,
                    shape,
                    rhinestones,
                },
            )
        },
    )(input)
}

use std::io::Write;

use nom::combinator::{flat_map, map};
use nom::multi::length_count;
use nom::number::complete::{le_u16, le_u32};
use nom::sequence::tuple;
use nom::IResult;

use crate::encode::Encode;
use crate::piece;
use crate::piece::Piece;
use crate::util::read_from_offsets;

#[derive(Debug)]
pub struct PieceTable {
    pieces: Vec<(u16, Piece)>,
}

pub fn read_piece_table(input: &[u8]) -> IResult<&[u8], PieceTable> {
    flat_map(
        tuple((
            length_count(le_u32, le_u32),
            le_u32,
            length_count(le_u32, le_u16),
        )),
        move |(offsets, total_length, ids)| {
            map(
                read_from_offsets(offsets, total_length, piece::read_piece),
                move |pieces| PieceTable {
                    pieces: ids.clone().into_iter().zip(pieces).collect(),
                },
            )
        },
    )(input)
}

impl Encode for PieceTable {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        let mut piece_data: Vec<(u16, Vec<u8>)> = vec![];
        for (id, piece) in &self.pieces {
            piece_data.push((*id, piece.encode_to_vec()?));
        }

        (piece_data.len() as u32).encode(buffer)?;
        let mut offset: u32 = 0;
        for (_, data) in &piece_data {
            offset.encode(buffer)?;
            offset += data.len() as u32;
        }
        offset.encode(buffer)?;
        (piece_data.len() as u32).encode(buffer)?;
        for (id, _) in &piece_data {
            id.encode(buffer)?;
        }
        for (_, data) in &piece_data {
            buffer.write_all(data)?;
        }

        Ok(())
    }
}

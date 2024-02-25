use std::fs;

use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

use crate::cut_data::CutData;
use crate::error::Error;
use crate::file_header::FileHeader;
use crate::piece::Piece;
use crate::{cut_data, file_header, piece};

#[derive(Debug)]
pub struct FcmFile {
    pub file_header: FileHeader,
    pub cut_data: CutData,
    pub pieces: Vec<(u16, Piece)>,
}

impl FcmFile {
    pub fn from_bytes(data: &[u8]) -> Result<FcmFile, Error> {
        let (_, file) = read_fcm_file(data).map_err(|e| Error {
            message: format!("Could not parse file: {0}", e),
        })?;
        Ok(file)
    }

    pub fn from_file<T: AsRef<std::path::Path>>(file: T) -> Result<FcmFile, Error> {
        let data = fs::read(file.as_ref()).map_err(|e| Error {
            message: format!("Could not open file: {0}", e),
        })?;
        let (_, file) = read_fcm_file(data.as_slice()).map_err(|e| Error {
            message: format!("Could not parse file: {0}", e),
        })?;
        Ok(file)
    }
}

pub(crate) fn read_fcm_file(input: &[u8]) -> IResult<&[u8], FcmFile> {
    map(
        tuple((
            file_header::read_file_header,
            cut_data::read_cut_data,
            piece::read_piece_table,
        )),
        |(file_header, cut_data, pieces)| FcmFile {
            file_header,
            cut_data,
            pieces,
        },
    )(input)
}

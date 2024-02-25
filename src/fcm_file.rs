use std::fs;

use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

use crate::cut_data::CutData;
use crate::encode::Encode;
use crate::error::Error;
use crate::file_header::FileHeader;
use crate::piece_table::PieceTable;
use crate::{cut_data, file_header, piece_table};

#[derive(Debug)]
pub struct FcmFile {
    pub file_header: FileHeader,
    pub cut_data: CutData,
    pub piece_table: PieceTable,
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
        FcmFile::from_bytes(data.as_slice())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        self.encode_to_vec().map_err(|e| Error {
            message: format!("Could not serialize file: {0}", e),
        })
    }

    pub fn to_file<T: AsRef<std::path::Path>>(&self, file: T) -> Result<(), Error> {
        fs::write(file, self.to_bytes()?.as_slice()).map_err(|e| Error {
            message: format!("Could not write to file: {0}", e),
        })
    }
}

pub(crate) fn read_fcm_file(input: &[u8]) -> IResult<&[u8], FcmFile> {
    map(
        tuple((
            file_header::read_file_header,
            cut_data::read_cut_data,
            piece_table::read_piece_table,
        )),
        |(file_header, cut_data, piece_table)| FcmFile {
            file_header,
            cut_data,
            piece_table,
        },
    )(input)
}

impl Encode for FcmFile {
    fn encode(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        self.file_header.encode(buffer)?;
        self.cut_data.encode(buffer)?;
        self.piece_table.encode(buffer)?;
        Ok(())
    }
}

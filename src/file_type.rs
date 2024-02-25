use nom::combinator::map_res;
use nom::number::complete::le_u32;
use nom::IResult;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FileType {
    Cut,
    PrintAndCut,
}

pub(crate) fn read_file_type(input: &[u8]) -> IResult<&[u8], FileType> {
    map_res(le_u32, |data| match data {
        0x10 => Ok(FileType::Cut),
        0x38 => Ok(FileType::PrintAndCut),
        _ => Err(format!("Unable to parse file type: {data}")),
    })(input)
}

pub mod lib;
use nom::IResult;
use nom::number::complete::be_u16;
use nom::bytes::complete::take;


/// Based on POSIX specification https://pubs.opengroup.org/onlinepubs/9699919799.2018edition/utilities/V3_chap02.html

pub fn length_value(input: &[u8]) -> IResult<&[u8],&[u8]> {
    let (input, length) = be_u16(input)?;
    take(length)(input)
}

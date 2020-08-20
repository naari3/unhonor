use nom::bytes::complete::{tag, take};
use nom::multi::count;
use nom::number::complete::*;
use nom::IResult;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct LinkResHeader {
    pub file_count: u32,
    resource_headers: Vec<ResHeader>,
}

#[derive(Debug)]
struct ResHeader {
    offset: u32,
    size: u32,
}

pub struct LinkResource {
    pub link_res_header: LinkResHeader,
    pub resources: Vec<Vec<u8>>,
}

pub fn read_file<P: AsRef<Path>>(file_path: P) -> Vec<u8> {
    let mut file = std::fs::File::open(file_path).expect("file open failed");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("file read failed");
    buf
}

fn read_link_res_header(input: &[u8]) -> IResult<&[u8], LinkResHeader> {
    let (input, _) = tag(b"LINK")(input)?;
    let (input, file_count) = le_u32(input)?;
    let (input, _) = take(8u32)(input)?; // 謎
    let (input, resources) = count(read_res_header, file_count as usize)(input)?;
    Ok((
        input,
        LinkResHeader {
            file_count,
            resource_headers: resources,
        },
    ))
}

fn read_res_header(input: &[u8]) -> IResult<&[u8], ResHeader> {
    let (input, offset) = le_u32(input)?;
    let (input, size) = le_u32(input)?;
    Ok((input, ResHeader { offset, size }))
}

pub fn read_link_resources(input: &[u8]) -> IResult<&[u8], LinkResource> {
    let (input, link_res_header) = read_link_res_header(input)?;
    let mut header_offset = (0x10 + link_res_header.resource_headers.len() * 0x08) as u32;
    let mut i = input;
    let mut resources: Vec<Vec<u8>> = Vec::new();
    for res_header in &link_res_header.resource_headers {
        i = take(res_header.offset - header_offset)(i)?.0; // seek
        let binary: &[u8];
        let result = take(res_header.size)(i)?; // seek
        i = result.0;
        binary = result.1;
        header_offset = res_header.offset + res_header.size;
        resources.push(binary.to_vec());
    }
    Ok((i, LinkResource { link_res_header, resources }))
}

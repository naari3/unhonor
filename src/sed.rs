use std::io::Error;

use std::io::Write;

use crate::link_resource::LinkResource;

fn create_sed(dbw_and_hbw: &[Vec<u8>]) -> Result<Vec<u8>, Error> {
    use byteorder::{LittleEndian, WriteBytesExt};

    let mut sed: Vec<u8> = Vec::new();

    let dbw = dbw_and_hbw.first().expect("Can not read dbw");
    let hbw = dbw_and_hbw.last().expect("Can not read hbw");

    sed.write(b"WHD1")?;
    sed.write_u32::<LittleEndian>(0x032E)?; // 0x04 fixed value?
    sed.write_u32::<LittleEndian>(0x01010000)?; // 0x08 version?
    sed.write_u32::<LittleEndian>(0x30)?; // 0x0c start
    sed.write_u32::<LittleEndian>(0x0)?; // 0x10 file size (invalid size now)
    sed.write_u32::<LittleEndian>(0x0)?; // 0x14 subfiles? (invalid size now)
    sed.write_u32::<LittleEndian>(0x0)?; // 0x18 subfiles? (invalid size now)
    sed.write_u32::<LittleEndian>(0x0)?; // 0x1c null
    sed.write_u32::<LittleEndian>(0x0)?; // 0x20 some size? (invalid size now)
    sed.write_u32::<LittleEndian>(0x0)?; // 0x24 some size? (invalid size now)
    sed.write_u64::<LittleEndian>(0x0)?; // 0x28 null
    sed.write_u32::<LittleEndian>(0x50)?; // 0x30 head offset
    let after_head_padding = {
        if hbw.len() % 10 != 0 {
            10 - hbw.len() % 10
        } else {
            0
        }
    };
    let body_offset = 0x50 + (hbw.len() + after_head_padding) as u32;
    sed.write_u32::<LittleEndian>(body_offset)?; // 0x34 body offset
    sed.write_u64::<LittleEndian>(0x0)?; // 0x38 null
    sed.write_u32::<LittleEndian>(hbw.len() as u32)?; // 0x40 head size
    sed.write_u32::<LittleEndian>(dbw.len() as u32)?; // 0x44 body size
    sed.write_u64::<LittleEndian>(0x0)?; // 0x48 null
    sed.write(hbw)?; // head
    sed.write(&vec![0u8; after_head_padding])?; // padding
    sed.write(dbw)?; // body

    Ok(sed)
}

pub fn link_resource_to_seds(link_resource: LinkResource) -> Result<Vec<Vec<u8>>, Error> {
    let mut seds: Vec<Vec<u8>> =
        Vec::with_capacity((link_resource.link_res_header.file_count / 2) as usize);
    for dbw_and_hbw in link_resource.resources.chunks_exact(2) {
        seds.push(create_sed(dbw_and_hbw)?);
    }
    Ok(seds)
}

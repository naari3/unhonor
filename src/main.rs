use std::io::Write;

use std::fs::File;

mod link_resource;
use link_resource::{read_file, read_link_resources};

mod sed;
use sed::link_resource_to_seds;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_file("res_snd.bin");
    let (_, link_resource) = read_link_resources(&data).expect("Could not parse resource file");

    println!("YO");
    let seds = link_resource_to_seds(link_resource)?;
    for (i, sed) in seds.iter().enumerate() {
        let mut file = File::create(format!("{}{}{}", "tmp/", i, ".sed"))?;
        let sed_size = sed.len();
        file.write_all(sed)?;
        file.flush()?;
        println!(
            "created: {}{}{}, filesize: {} byte",
            "tmp/", i, ".sed", sed_size
        );
    }

    Ok(())
}

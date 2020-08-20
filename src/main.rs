use std::io::Write;

use std::fs::File;

use regex::Regex;

mod link_resource;
use link_resource::{read_file, read_link_resources};

mod sed;
use sed::link_resource_to_seds;

use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_file("res_snd.bin");
    let (_, link_resource) = read_link_resources(&data).expect("Could not parse resource file");

    println!("YO");
    let seds = link_resource_to_seds(link_resource)?;
    let mut sed_paths = Vec::with_capacity(seds.len());
    for (i, sed) in seds.iter().enumerate() {
        let sed_path = format!("{}{}{}", "tmp/seds/", i, ".sed");
        let mut file = File::create(&sed_path)?;
        let sed_size = sed.len();
        file.write_all(sed)?;
        file.flush()?;
        println!("created: {}, filesize: {} byte", &sed_path, sed_size);
        sed_paths.push(sed_path);
    }

    for (i, sed_path) in sed_paths.iter().enumerate() {
        let output = Command::new(".\\test\\test.exe")
            .args(&["-m", "-o", "test.wav", &sed_path])
            .output()?;
        let metainfo_string = String::from_utf8_lossy(&output.stdout).to_string();

        let re = Regex::new(r"stream count: (\d+)")?;
        let caps = re.captures(&metainfo_string).unwrap();
        let stream_count: u32 = caps.get(1).unwrap().as_str().parse()?;
        for j in 1..stream_count + 1 {
            Command::new(".\\test\\test.exe")
                .args(&[
                    "-s",
                    &j.to_string(),
                    "-o",
                    &format!(".\\tmp\\wavs\\test_{}_{}.wav", i, j),
                    &sed_path,
                ])
                .spawn()?;
        }
    }

    Ok(())
}

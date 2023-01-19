mod err;

use crate::err::PatchError;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const SIZE: u64 = 6854144; // file size in bytes
const OFFSET: u64 = 0x3C1F7; // sprite encoder image type
const ORIGINAL: u8 = 0x06; // value to replace (=RGBA4444)
const PATCHED: u8 = 0x00; // value to replace with (=RGBA8888)

fn main() {
    //let exec_path = std::env::current_exe().unwrap();
    //let directory = exec_path.parent().unwrap();

    let directory = std::env::current_dir().unwrap();

    if !directory.as_path().is_dir() {
        panic!("directory is not a directory (wtf how)");
    }

    let files = std::fs::read_dir(&directory).unwrap();

    let mut has_gd_exe = false;
    let mut has_resources = false;

    for file in files {
        let file = file.unwrap();
        let name = &file.file_name().to_str().unwrap().to_string();
        if name == "Resources" && file.file_type().unwrap().is_dir() {
            has_resources = true;
        } else if name == "GeometryDash" && file.file_type().unwrap().is_file() {
            has_gd_exe = true;
        }
    }

    if !has_resources { /*panic!("no resources folder");*/ }
    if !has_gd_exe {
        panic!("no gd exe");
    }

    let gd_exe = directory.join("GeometryDash.exe");

    let resources = directory.join("Resources");

    let exe_patch = patch_exe(gd_exe);
    if exe_patch.is_ok() && *exe_patch.as_ref().unwrap() {
        println!("patched exe");
    } else if exe_patch.is_err() {
        println!("failed to patch exe: {}", exe_patch.err().unwrap());
    } else {
        println!("exe already patched");
    }

    let resources_patch = patch_resources(resources);
    if resources_patch.is_ok() && *resources_patch.as_ref().unwrap() {
        println!("patched resources");
    } else if resources_patch.is_err() {
        println!("failed to patch resources: {}", resources_patch.err().unwrap());
    } else {
        println!("resources already patched");
    }
}

fn patch_exe(gd_exe: PathBuf) -> Result<bool, Box<dyn Error>> {
    let gd_exe_size = gd_exe.metadata()?.len();

    if gd_exe_size != SIZE {
        println!(
            "invalid gd executable, expected size {}, got {}",
            SIZE, gd_exe_size
        );
        return Err(PatchError::new(String::from("invalid gd executable")).into());
    }

    let mut reader = BufReader::new(File::open(&gd_exe)?);
    reader.seek(SeekFrom::Start(OFFSET))?;

    let mut buf = [0; 1];
    reader.read(&mut buf)?;

    println!("value: {:#04x}", buf[0]);

    if buf[0] == PATCHED {
        return Ok(false);
    }

    if buf[0] != ORIGINAL {
        return Err(PatchError::new(format!(
            "invalid value found at {:#x} - expected 0x60, got {:#04x}",
            OFFSET, buf[0]
        ))
        .into());
    }

    let mut writer = OpenOptions::new().write(true).open(gd_exe)?;
    writer.seek(SeekFrom::Start(OFFSET))?;
    writer.write(&[0x00])?;

    Ok(true)
}

fn patch_resources(directory: PathBuf) -> Result<bool, Box<dyn Error>> {
    let gj_gamesheet_uhd = include_bytes!("./../res/GJ_GameSheet-uhd.png");
    let gj_gamesheet_uhd_path = directory.join("GJ_GameSheet-uhd.png");

    let mut writer = OpenOptions::new().write(true).truncate(true).open(gj_gamesheet_uhd_path)?;
    writer.write(gj_gamesheet_uhd)?;

    Ok(true)
}
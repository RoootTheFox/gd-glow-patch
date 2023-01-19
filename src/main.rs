mod app;
mod err;
mod platform;

use crate::app::GDGlowPatchApp;
use crate::err::{PatchError, TargetState};
use lazy_static_include::lazy_static_include_bytes;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const SIZE: u64 = 6854144; // file size in bytes
const OFFSET: u64 = 0x3C1F7; // sprite encoder image type
const ORIGINAL: u8 = 0x06; // value to replace (=RGBA4444)
const PATCHED: u8 = 0x00; // value to replace with (=RGBA8888)

const GAMESHEET_SIZE: u64 = 2865699; // GJ_GameSheet-uhd.png (unmodified) size in bytes

lazy_static_include_bytes! {
    PATCHED_GAMESHEET => "res/GJ_GameSheet-uhd.png"
}

fn main() {
    let window_size = egui::vec2(280.0, 320.0);
    let options = eframe::NativeOptions {
        initial_window_size: Some(window_size),
        min_window_size: Some(window_size),
        max_window_size: Some(window_size),
        ..Default::default()
    };

    eframe::run_native(
        "GD Glow Patch",
        options,
        Box::new(|_cc| Box::new(GDGlowPatchApp::default())),
    );
}

fn check_gd_exe(app: &mut GDGlowPatchApp, gd_exe: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(&gd_exe)?);
    reader.seek(SeekFrom::Start(OFFSET))?;

    let mut buf = [0; 1];
    reader.read(&mut buf)?;

    if buf[0] == PATCHED {
        app.exe_state = TargetState::Patched;
    } else if buf[0] != ORIGINAL {
        app.exe_state = TargetState::Invalid;
    } else {
        app.exe_state = TargetState::Present;
    }

    Ok(())
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

fn patch_resources(path: PathBuf) -> Result<bool, Box<dyn Error>> {
    let mut writer = OpenOptions::new().write(true).truncate(true).open(path)?;
    writer.write(&PATCHED_GAMESHEET)?;

    Ok(true)
}

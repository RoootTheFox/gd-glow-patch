use std::path::PathBuf;

pub(crate) fn get_gd_directory() -> Option<PathBuf> {
    let mut gd_path:Option<PathBuf> = None;
    
    #[cfg(target_os = "linux")] {
        let home = std::env::var("HOME").unwrap();
        gd_path = Some(PathBuf::from(home)
            .join(".steam")
            .join("steam")
            .join("steamapps")
            .join("common")
            .join("Geometry Dash"));
    }
    #[cfg(target_os = "windows")] {
        gd_path = Some(PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Geometry Dash"));
    }

    #[cfg(not(any(target_os = "linux", windows)))] {
        compile_error!("Unsupported platform!");
    }

    gd_path
}
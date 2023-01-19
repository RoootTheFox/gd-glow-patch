use crate::err::TargetState;
use crate::{
    check_gd_exe, patch_exe, patch_resources, DIRECTORY, GAMESHEET_SIZE, PATCHED_GAMESHEET, SIZE,
};
use catppuccin_egui::MOCHA;
use std::path::PathBuf;

pub(crate) struct GDGlowPatchApp {
    pub(crate) exe_state: TargetState,
    pub(crate) exe_path: PathBuf,
    pub(crate) gamesheet_state: TargetState,
    pub(crate) gamesheet_path: PathBuf,
}

impl Default for GDGlowPatchApp {
    fn default() -> Self {
        let mut app = Self {
            exe_state: TargetState::Missing,
            exe_path: DIRECTORY.join("GeometryDash.exe"),
            gamesheet_state: TargetState::Missing,
            gamesheet_path: DIRECTORY.join("Resources").join("GJ_GameSheet-uhd.png"),
        };

        update_ui_states(&mut app);

        app
    }
}

fn update_ui_states(app: &mut GDGlowPatchApp) {
    let mut has_gd_exe = false;
    let mut has_gamesheet = false;

    if app.exe_path.exists() {
        has_gd_exe = true;
    }

    if app.gamesheet_path.exists() {
        has_gamesheet = true;
    }

    if !has_gamesheet {
        app.gamesheet_state = TargetState::Missing;
    } else {
        let metadata = app.gamesheet_path.metadata();
        let gamesheet_size;

        if metadata.is_ok() {
            gamesheet_size = metadata.unwrap().len();

            if gamesheet_size == GAMESHEET_SIZE {
                app.gamesheet_state = TargetState::Present;
            } else if gamesheet_size == PATCHED_GAMESHEET.len() as u64 {
                app.gamesheet_state = TargetState::Patched;
            } else {
                app.gamesheet_state = TargetState::Invalid;
            }
        } else {
            app.gamesheet_state = TargetState::Invalid;
        }
    }
    if !has_gd_exe {
        app.exe_state = TargetState::Missing;
    } else {
        let gd_exe = app.exe_path.clone();
        let metadata = gd_exe.metadata();
        let gd_exe_size;

        if metadata.is_ok() {
            gd_exe_size = metadata.unwrap().len();

            if gd_exe_size != SIZE {
                app.exe_state = TargetState::Invalid;
            } else {
                if check_gd_exe(app, &gd_exe).is_err() {
                    app.exe_state = TargetState::Invalid;
                }
            }
        } else {
            app.exe_state = TargetState::Invalid;
        }
    }
}

impl eframe::App for GDGlowPatchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(&ctx, MOCHA);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GD Glow Patch");
            if ui
                .button("refresh")
                .on_hover_text("reloads the UI state (you probably won't need this)")
                .clicked()
            {
                update_ui_states(self);
            }
            ui.separator();

            let mut exe_missing = false;
            let mut exe_patched = false;
            let mut gamesheet_missing = false;
            let mut gamesheet_patched = false;

            match self.exe_state {
                TargetState::Present => {
                    ui.colored_label(MOCHA.sky, "GeometryDash.exe is present");
                    if ui
                        .button("Patch exe")
                        .on_hover_text("Patches the GeometryDash.exe file")
                        .clicked()
                    {
                        let exe_patched = patch_exe(self.exe_path.clone());
                        if !exe_patched.is_ok() {
                            ui.colored_label(MOCHA.red, "Failed to patch GeometryDash.exe!");
                        }
                        update_ui_states(self);
                    }
                }
                TargetState::Missing => {
                    ui.colored_label(MOCHA.red, "GeometryDash.exe not found!");
                    exe_missing = true;
                }
                TargetState::Invalid => {
                    ui.colored_label(MOCHA.red, "GeometryDash.exe is invalid!");
                }
                TargetState::Patched => {
                    ui.colored_label(MOCHA.green, "GeometryDash.exe is patched!");
                    exe_patched = true;
                }
            }

            match self.gamesheet_state {
                TargetState::Present => {
                    ui.colored_label(MOCHA.sky, "GJ_GameSheet-uhd.png is present");
                    if ui
                        .button("Patch gamesheet")
                        .on_hover_text("Patches GJ_GameSheet-uhd.png for better glow")
                        .clicked()
                    {
                        let res_patched = patch_resources(self.gamesheet_path.clone());
                        if !res_patched.is_ok() {
                            ui.colored_label(MOCHA.red, "Failed to patch gamesheet!");
                        }
                        update_ui_states(self);
                    }
                }
                TargetState::Missing => {
                    ui.colored_label(MOCHA.red, "GJ_GameSheet-uhd.png not found!");
                    gamesheet_missing = true;
                }
                TargetState::Invalid => {
                    ui.colored_label(MOCHA.red, "GJ_GameSheet-uhd.png is invalid!");
                    if ui
                        .button("Patch gamesheet anyway")
                        .on_hover_text("Patches GJ_GameSheet-uhd.png for better glow")
                        .clicked()
                    {
                        let res_patched = patch_resources(self.gamesheet_path.clone());
                        if !res_patched.is_ok() {
                            ui.colored_label(MOCHA.red, "Failed to patch gamesheet!");
                        }
                        update_ui_states(self);
                    }
                }
                TargetState::Patched => {
                    ui.colored_label(MOCHA.green, "GJ_GameSheet-uhd.png is patched!");
                    gamesheet_patched = true;
                }
            }

            if exe_missing || gamesheet_missing {
                ui.separator();
                ui.colored_label(
                    MOCHA.peach,
                    "Make you you are running this program in the same folder as GeometryDash.exe!",
                );
            } else if exe_patched && gamesheet_patched {
                ui.separator();
                ui.colored_label(MOCHA.teal, "The patch is applied, you are good to go!");
            } else if gamesheet_patched && (!exe_patched && !exe_missing) {
                ui.separator();
                ui.colored_label(
                    MOCHA.yellow,
                    "The gamesheet is patched, but the exe is not!\n\
                You can still play the game, but glow will look bad.",
                );
            } else if exe_patched && (!gamesheet_patched && !gamesheet_missing) {
                ui.separator();
                ui.colored_label(
                    MOCHA.yellow,
                    "The exe is patched, but the gamesheet is not!\n\
                Your game performance will be better but glow will look bad.",
                );
            }
        });
    }
}

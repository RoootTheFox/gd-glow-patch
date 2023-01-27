use crate::err::TargetState;
use crate::{
    check_gd_exe, patch_exe, patch_resources, platform, GAMESHEET_SIZE, PATCHED_GAMESHEET, SIZE,
    WINDOW_SIZE,
};
use catppuccin_egui::MOCHA;
use egui::WidgetText;
use std::ops::Sub;
use std::path::PathBuf;

pub(crate) struct GDGlowPatchApp {
    pub(crate) gd_path: PathBuf,
    pub(crate) exe_state: TargetState,
    pub(crate) exe_path: PathBuf,
    pub(crate) gamesheet_state: TargetState,
    pub(crate) gamesheet_path: PathBuf,
}

impl Default for GDGlowPatchApp {
    fn default() -> Self {
        let target_dir;

        let current_dir = std::env::current_dir().unwrap();
        if current_dir.join("GeometryDash.exe").exists() {
            target_dir = current_dir;
        } else {
            let gd_dir = platform::get_gd_directory();
            if gd_dir.is_some() && gd_dir.as_ref().unwrap().is_dir() {
                println!(
                    "Found Geometry Dash directory: {:?}",
                    gd_dir.as_ref().unwrap()
                );
                target_dir = gd_dir.unwrap();
            } else {
                // fall back to current directory so we can show the user helpful error messages
                target_dir = current_dir;
            }
        }

        let mut app = Self {
            gd_path: target_dir.clone(),
            exe_state: TargetState::Missing,
            exe_path: target_dir.join("GeometryDash.exe"),
            gamesheet_state: TargetState::Missing,
            gamesheet_path: target_dir.join("Resources").join("GJ_GameSheet-uhd.png"),
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(&ctx, MOCHA);

        custom_window_frame(ctx, frame, "GD Glow Patch", |ui| {
            /*
            let height = 28.0;
            let button_size = height - 4.0;
            let rect = ui.max_rect();

            // a bit hacky but it works
            let reload_response = ui.put(
                Rect::from_min_size(rect.left_top().sub(Pos2::new(4.0, height + 6.0)).to_pos2(), Vec2::splat(height)),
                Button::new(RichText::new("🔄").size(button_size - 4.0)).frame(false),
            ).on_hover_text("reloads the UI state (you probably won't need this)");
            if reload_response.clicked() {
                println!("Reloading UI state");
                update_ui_states(self);
            }
            ui.add_space(4.0);
            */

            ui.label(format!("{}", self.gd_path.to_str().unwrap()))
                .on_hover_text("Geometry Dash directory");
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
                        let exe_patched = patch_exe(&self.exe_path.clone());
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

            // move this to the bottom of the window
            ui.separator();

            ui.hyperlink_to(
                WidgetText::from("Made by Rooot"),
                "https://github.com/RoootTheFox",
            );
            ui.colored_label(MOCHA.sky, "Credits:");
            ui.hyperlink_to(
                WidgetText::from("Patch found by SMJS"),
                "https://github.com/SMJSGaming"
            );
            ui.hyperlink_to(
                WidgetText::from("Gamesheet by noah_endy"),
                "https://twitter.com/noah_endy"
            );
            ui.hyperlink_to(
                WidgetText::from("Source Code"),
                "https://github.com/RoootTheFox/gd-glow-patch"
            );
        });
    }
}

// adapted from https://github.com/emilk/egui/blob/master/examples/custom_window_frame/src/main.rs
fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    let text_color = ctx.style().visuals.text_color();

    // title bar height
    let height = 28.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint the frame:
            painter.rect(
                rect.shrink(1.0),
                4.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );

            // Paint the title:
            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height * 0.8),
                text_color,
            );

            // Paint the line under the title:
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_color),
            );

            // Interact with the title bar (drag to move window):
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };

            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            // Add the close button:
            let button_size = height - 4.0;
            let close_response = ui.put(
                Rect::from_min_size(
                    rect.right_top().sub(pos2(height, 0.0)).to_pos2(),
                    Vec2::splat(button_size),
                ),
                Button::new(RichText::new("❌").size(button_size)).frame(false),
            );
            if close_response.clicked() {
                frame.close();
            }

            // Add the contents:
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                /*rect.max.y = title_bar_rect.max.y;
                rect.min.x = rect.min.x + 4.0;
                rect.set_width(rect.width());*/

                rect
            }
            .shrink(6.0);

            ui.vertical_centered(|ui| {
                ui.set_width(content_rect.width());
                let mut content_ui = ui.child_ui(content_rect, Layout::top_down(Align::LEFT));
                add_contents(&mut content_ui);
            });
        });
}

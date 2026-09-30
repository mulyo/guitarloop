// ======================================================================
// ui/menu.rs - Top menu bar (File, Edit, View, Help)
// ======================================================================

use crate::app::GuitarLoopApp;
use eframe::egui;

pub fn render(app: &mut GuitarLoopApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            // ---- File ----
            ui.menu_button("File", |ui| {
                if ui
                    .add(
                        egui::Button::new("🗁  Open video/audio...")
                            .shortcut_text("Ctrl+O"),
                    )
                    .clicked()
                {
                    app.open_file();
                    ui.close_menu();
                }
                ui.separator();
                if ui.add(egui::Button::new("⏹  Close file")).clicked() {
                    app.reset_playback();
                    app.set_status("File ditutup.");
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add(
                        egui::Button::new("⏻  Exit")
                            .shortcut_text("Alt+F4"),
                    )
                    .clicked()
                {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            // ---- Playback ----
            ui.menu_button("Playback", |ui| {
                if ui
                    .add(egui::Button::new("▶ Play").shortcut_text("Space"))
                    .clicked()
                {
                    app.playback_state = crate::app::PlaybackState::Playing;
                    app.set_status("Playing");
                    ui.close_menu();
                }
                if ui
                    .add(egui::Button::new("⏸ Pause").shortcut_text("Space"))
                    .clicked()
                {
                    app.playback_state = crate::app::PlaybackState::Paused;
                    app.set_status("Paused");
                    ui.close_menu();
                }
                if ui
                    .add(egui::Button::new("⏹ Stop").shortcut_text("S"))
                    .clicked()
                {
                    app.playback_state = crate::app::PlaybackState::Stopped;
                    app.current_position = std::time::Duration::ZERO;
                    app.set_status("Stopped");
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add(
                        egui::Button::new("⇤ Seek -5s")
                            .shortcut_text("←"),
                    )
                    .clicked()
                {
                    app.current_position = app
                        .current_position
                        .saturating_sub(std::time::Duration::from_secs(5));
                    ui.close_menu();
                }
                if ui
                    .add(
                        egui::Button::new("Seek +5s ⇥")
                            .shortcut_text("→"),
                    )
                    .clicked()
                {
                    app.current_position = app
                        .current_position
                        .saturating_add(std::time::Duration::from_secs(5));
                    ui.close_menu();
                }
            });

            // ---- Practice Tools ----
            ui.menu_button("Tools", |ui| {
                ui.checkbox(&mut app.pitch_correction, "Pitch Correction");
                ui.separator();
                let mut markers = app.loop_markers;
                if ui.button("Set Mark A (loop start)").clicked() {
                    markers.a = Some(app.current_position);
                    markers.enabled = true;
                    app.set_status(format!(
                        "Mark A diset @ {:?}",
                        app.current_position
                    ));
                }
                if ui.button("Set Mark B (loop end)").clicked() {
                    markers.b = Some(app.current_position);
                    markers.enabled = true;
                    app.set_status(format!(
                        "Mark B diset @ {:?}",
                        app.current_position
                    ));
                }
                if ui.button("Clear loop markers").clicked() {
                    markers = crate::app::LoopMarkers::default();
                    app.set_status("Loop markers dihapus.");
                }
                app.loop_markers = markers;
            });

            // ---- Help ----
            ui.menu_button("Help", |ui| {
                if ui.button("About GuitarLoop").clicked() {
                    app.set_status(format!(
                        "GuitarLoop v{}  [Rust build: {} {}]",
                        env!("CARGO_PKG_VERSION"),
                        option_env!("BUILD_DATE").unwrap_or(env!("CARGO_PKG_VERSION")),
                        option_env!("PROFILE").unwrap_or("dev"),
                    ));
                    ui.close_menu();
                }
                ui.separator();
                ui.label(format!("GStreamer: {}", gstreamer::version_string()));
                ui.label("egui (GUI framework): 0.28 (dari Cargo.toml)");
            });
        });
    });
}

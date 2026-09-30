// ======================================================================
// ui/status_bar.rs - Bottom status bar
// ======================================================================

use crate::app::GuitarLoopApp;
use eframe::egui;

pub fn render(app: &mut GuitarLoopApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status_bar")
        .resizable(false)
        .min_height(22.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let state_color = match app.playback_state {
                    crate::app::PlaybackState::Playing => egui::Color32::from_rgb(80, 200, 80),
                    crate::app::PlaybackState::Paused => egui::Color32::from_rgb(230, 180, 50),
                    crate::app::PlaybackState::Loading => egui::Color32::from_rgb(80, 160, 255),
                    crate::app::PlaybackState::Error => egui::Color32::from_rgb(255, 80, 80),
                    crate::app::PlaybackState::Stopped => egui::Color32::from_rgb(140, 140, 140),
                };
                let state_label = format!("{:?}", app.playback_state).to_uppercase();

                ui.label(
                    egui::RichText::new(format!("● {} ", state_label))
                        .monospace()
                        .size(11.0)
                        .color(state_color),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(&app.status_message)
                        .size(11.0)
                        .color(egui::Color32::from_rgb(190, 175, 155)),
                );

                // Kanan: info
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        let db_status = match crate::db::database_available() {
                            true => "🗄 DB: Ready",
                            false => "🗄 DB: Not init",
                        };
                        ui.label(
                            egui::RichText::new(db_status)
                                .small()
                                .color(egui::Color32::from_rgb(140, 120, 100)),
                        );
                    },
                );
            });
        });
}

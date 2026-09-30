// ======================================================================
// ui/main_panel.rs - Central panel: video area + transport controls
// ======================================================================

use crate::app::GuitarLoopApp;
use eframe::egui;
use std::time::Duration;

pub fn render(app: &mut GuitarLoopApp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        // ---- Video area ----
        let video_area = ui
            .vertical_centered(|ui| {
                let (resp, painter) = build_video_surface(ui);
                render_video_placeholder(app, &painter, resp.rect);
                resp
            })
            .inner;

        ui.add_space(10.0);

        // ---- Transport controls (row 1: play/pause/stop) ----
        ui.horizontal(|ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center)
                    .with_main_align(egui::Align::Center),
                |ui| {
                    ui.horizontal(|ui| {
                        // Prev / back 10s
                        if ui
                            .add(egui::Button::new("⏪").min_size(egui::vec2(44.0, 36.0)))
                            .on_hover_text("Seek -10s (←)")
                            .clicked()
                        {
                            app.current_position = app
                                .current_position
                                .saturating_sub(Duration::from_secs(10));
                        }

                        // Play / Pause toggle
                        let play_btn = match app.playback_state {
                            crate::app::PlaybackState::Playing => "⏸",
                            _ => "▶",
                        };
                        if ui
                            .add(
                                egui::Button::new(play_btn)
                                    .min_size(egui::vec2(64.0, 40.0))
                                    .fill(egui::Color32::from_rgb(180, 100, 30)),
                            )
                            .on_hover_text("Play/Pause (Space)")
                            .clicked()
                        {
                            toggle_play_pause(app);
                        }

                        // Stop
                        if ui
                            .add(egui::Button::new("⏹").min_size(egui::vec2(44.0, 36.0)))
                            .on_hover_text("Stop (S)")
                            .clicked()
                        {
                            app.playback_state = crate::app::PlaybackState::Stopped;
                            app.current_position = Duration::ZERO;
                            app.set_status("Stopped");
                        }

                        // Fwd / next 10s
                        if ui
                            .add(egui::Button::new("⏩").min_size(egui::vec2(44.0, 36.0)))
                            .on_hover_text("Seek +10s (→)")
                            .clicked()
                        {
                            app.current_position = app
                                .current_position
                                .saturating_add(Duration::from_secs(10));
                        }
                    });
                },
            );
        });

        ui.add_space(6.0);

        // ---- Seek bar ----
        render_seekbar(app, ui);

        ui.add_space(4.0);

        // ---- Timecode display ----
        render_timecode(app, ui);

        // ---- Keyboard shortcut hint ----
        ui.vertical_centered(|ui| {
            ui.add_space(6.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new("Shortcut: Space=Play/Pause  ←→=±5s  A=MarkA  B=MarkB  S=Stop")
                        .small()
                        .color(egui::Color32::from_rgb(140, 120, 100)),
                )
                .sense(egui::Sense::hover()),
            );
        });

        // Let the UI know video area exists (for future texture handle)
        let _ = video_area;
    });
}

// ----------------------------------------------------------------------
// build_video_surface - allocate area for video rendering
// ----------------------------------------------------------------------
fn build_video_surface(ui: &mut egui::Ui) -> (egui::Response, egui::Painter) {
    // Catatan: egui 0.28 Vec2 tidak punya .width/.height, pakai .x (lebar) dan .y (tinggi)
    let available = ui.available_size_before_wrap();
    let target_h = (available.x / 16.0) * 9.0; // 16:9 aspect ratio
    let size = egui::vec2(
        available.x,
        target_h.min(available.y - 200.0).max(240.0),
    );
    let (rect, resp) = ui.allocate_exact_size(
        size,
        egui::Sense::drag() | egui::Sense::click(),
    );
    let painter = ui.painter_at(rect);
    (resp, painter)
}

// ----------------------------------------------------------------------
// render_video_placeholder - placeholder texture sampai video sink
// ----------------------------------------------------------------------
fn render_video_placeholder(
    app: &GuitarLoopApp,
    painter: &egui::Painter,
    rect: egui::Rect,
) {
    // Background gelap
    painter.rect_filled(
        rect,
        egui::Rounding::ZERO,
        egui::Color32::from_rgb(12, 10, 8),
    );

    // Border
    painter.rect_stroke(
        rect,
        egui::Rounding::ZERO,
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(80, 55, 30)),
    );

    // Status text ditengah
    let center = rect.center();
    let status = match app.metadata.path {
        Some(_) => match app.playback_state {
            crate::app::PlaybackState::Loading => "⏳  Memuat file...",
            crate::app::PlaybackState::Playing => "▶  Video Playing",
            crate::app::PlaybackState::Paused => "⏸  Video Paused",
            crate::app::PlaybackState::Stopped => "⏹  Video loaded (tekan Play)",
            crate::app::PlaybackState::Error => "⚠  Error playback",
        },
        None => "♪  GuitarLoop • Pilih File > Open untuk mulai latihan",
    };

    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        status,
        egui::FontId::proportional(18.0),
        egui::Color32::from_rgb(200, 170, 130),
    );

    // Judul file
    if !app.metadata.title.is_empty() {
        painter.text(
            egui::pos2(center.x, center.y + 30.0),
            egui::Align2::CENTER_CENTER,
            &app.metadata.title,
            egui::FontId::proportional(12.0),
            egui::Color32::from_rgb(140, 120, 100),
        );
    }
}

// ----------------------------------------------------------------------
// toggle_play_pause
// ----------------------------------------------------------------------
fn toggle_play_pause(app: &mut GuitarLoopApp) {
    use crate::app::PlaybackState::*;
    match app.playback_state {
        Playing => {
            app.playback_state = Paused;
            app.set_status("Paused");
        }
        Paused | Stopped | Loading => {
            if app.metadata.path.is_some() {
                app.playback_state = Playing;
                app.set_status("Playing");
            } else {
                app.set_status("Buka file video terlebih dahulu!");
            }
        }
        Error => {
            app.set_status("Ada error playback, coba buka file ulang.");
        }
    }
}

// ----------------------------------------------------------------------
// render_seekbar
// ----------------------------------------------------------------------
fn render_seekbar(app: &mut GuitarLoopApp, ui: &mut egui::Ui) {
    let duration = app.metadata.duration.unwrap_or(Duration::from_secs(300));
    let max_secs = duration.as_secs_f32().max(1.0);
    let mut pos_secs = app.current_position.as_secs_f32().min(max_secs);

    ui.horizontal(|ui| {
        ui.label(" ");
        let resp = ui.add(
            egui::Slider::new(&mut pos_secs, 0.0..=max_secs)
                .show_value(false)
                .trailing_fill(true),
        );
        ui.label(" ");

        if resp.drag_stopped() || resp.changed() {
            app.current_position = Duration::from_secs_f32(pos_secs);
        }

        // Gambar marker A/B overlay di seekbar nanti via custom widget
        // (placeholder: cukup logika di tools panel)
    });
}

// ----------------------------------------------------------------------
// render_timecode
// ----------------------------------------------------------------------
fn render_timecode(app: &GuitarLoopApp, ui: &mut egui::Ui) {
    let cur = format_duration(app.current_position);
    let tot = match app.metadata.duration {
        Some(d) => format_duration(d),
        None => "--:--".to_string(),
    };
    let speed = format!("{:.2}×", app.speed);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{} / {}", cur, tot))
                .monospace()
                .size(14.0)
                .color(egui::Color32::from_rgb(210, 180, 140)),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(speed)
                    .monospace()
                    .color(egui::Color32::from_rgb(255, 150, 50)),
            );
        });
    });
}

// ----------------------------------------------------------------------
// format_duration -> MM:SS or HH:MM:SS
// ----------------------------------------------------------------------
fn format_duration(d: Duration) -> String {
    let total = d.as_secs();
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}

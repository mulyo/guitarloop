// ======================================================================
// ui/tools_panel.rs - Practice tools side panel
// Speed, volume, loop markers, metronome, transpose placeholder
// ======================================================================

use crate::app::GuitarLoopApp;
use eframe::egui;
use std::time::Duration;

pub fn render(app: &mut GuitarLoopApp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.heading("🎛 Practice Tools");
        ui.add_space(4.0);

        // ---- Speed Control ----
        ui.collapsing("⏩ Speed", |ui| {
            ui.horizontal(|ui| {
                for &s in &[0.25f32, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0] {
                    if ui
                        .selectable_label(
                            (app.speed - s).abs() < 0.01,
                            format!("{:.2}×", s),
                        )
                        .clicked()
                    {
                        app.speed = s;
                        app.set_status(format!("Speed: {:.2}×", s));
                    }
                }
            });
            ui.add_space(4.0);
            ui.add(egui::Slider::new(&mut app.speed, 0.25..=2.0).step_by(0.05));
            ui.checkbox(
                &mut app.pitch_correction,
                "🔊 Pitch Correction (pertahankan nada)",
            );
        });

        ui.separator();

        // ---- Volume ----
        ui.collapsing("🔊 Volume", |ui| {
            ui.add(
                egui::Slider::new(&mut app.volume, 0.0..=1.0)
                    .show_value(true)
                    .suffix(" %")
                    .custom_formatter(|v, _| format!("{:.0}", v * 100.0)),
            );
        });

        ui.separator();

        // ---- Loop A->B ----
        ui.collapsing("🔁 Loop A→B", |ui| {
            let mut markers = app.loop_markers;
            ui.horizontal(|ui| {
                if ui.button("📍 Mark A (current)").clicked() {
                    markers.a = Some(app.current_position);
                    markers.enabled = true;
                    app.set_status(format!(
                        "Mark A: {}",
                        fmt_dur(app.current_position)
                    ));
                }
                ui.label(fmt_dur_opt(markers.a));
            });
            ui.horizontal(|ui| {
                if ui.button("📍 Mark B (current)").clicked() {
                    markers.b = Some(app.current_position);
                    markers.enabled = true;
                    app.set_status(format!(
                        "Mark B: {}",
                        fmt_dur(app.current_position)
                    ));
                }
                ui.label(fmt_dur_opt(markers.b));
            });
            ui.add_space(4.0);
            ui.checkbox(&mut markers.enabled, "Enable loop");
            ui.horizontal(|ui| {
                ui.label("Repeat:");
                ui.add(egui::DragValue::new(&mut markers.repeat_count).range(0..=9999));
                ui.label("× (0=infinite)");
            });
            if ui.button("🧹 Clear markers").clicked() {
                markers = crate::app::LoopMarkers::default();
                app.set_status("Loop markers cleared");
            }
            app.loop_markers = markers;
        });

        ui.separator();

        // ---- Metronome (placeholder) ----
        ui.collapsing("🥁 Metronome", |ui| {
            ui.label("Coming soon in v1.0 (Fase 2)");
            ui.add(egui::Slider::new(&mut 120u32, 40..=240).text("BPM"));
        });

        // ---- Transpose (placeholder) ----
        ui.collapsing("🎼 Transpose", |ui| {
            ui.label("Coming soon in v1.0 (Fase 2)");
            ui.add(egui::Slider::new(&mut 0i32, -12..=12).text("Semitones"));
        });

        // ---- Waveform (placeholder) ----
        ui.collapsing("〰 Waveform", |ui| {
            ui.label("Coming soon in v1.0 (Fase 2)");
            ui.add_space(50.0);
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            ui.separator();
            ui.label(
                egui::RichText::new("GuitarLoop MVP • Fase 1 setup")
                    .small()
                    .color(egui::Color32::from_rgb(100, 85, 70)),
            );
        });
    });
    let _ = Duration::ZERO; // keep import
}

// --- helpers ---
fn fmt_dur(d: Duration) -> String {
    let t = d.as_secs();
    let m = t / 60;
    let s = t % 60;
    let ms = d.subsec_millis();
    format!("{:02}:{:02}.{:03}", m, s, ms)
}
fn fmt_dur_opt(o: Option<Duration>) -> String {
    match o {
        Some(d) => fmt_dur(d),
        None => "--:--.---".to_string(),
    }
}

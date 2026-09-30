// ======================================================================
// GuitarLoop - app.rs
// Core application state + eframe App implementation.
// ======================================================================

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

// ----------------------------------------------------------------------
// PlaybackState - status player
// ----------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Loading,
    Error,
}

// ----------------------------------------------------------------------
// Markers - Loop A->B markers
// ----------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct LoopMarkers {
    pub a: Option<Duration>,
    pub b: Option<Duration>,
    pub enabled: bool,
    pub repeat_count: u32,
}

// ----------------------------------------------------------------------
// VideoMetadata - info video yang diload
// ----------------------------------------------------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub path: Option<PathBuf>,
    pub title: String,
    pub duration: Option<Duration>,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
}

// ----------------------------------------------------------------------
// GuitarLoopApp - state utama
// ----------------------------------------------------------------------
pub struct GuitarLoopApp {
    // -- playback --
    pub playback_state: PlaybackState,
    pub current_position: Duration,
    pub speed: f32,          // 0.25 - 2.0
    pub volume: f32,         // 0.0 - 1.0
    pub pitch_correction: bool,
    pub loop_markers: LoopMarkers,
    pub metadata: VideoMetadata,

    // -- UI state --
    pub open_file_dialog: bool,
    pub status_message: String,

    // -- async runtime handle --
    pub tokio_rt: tokio::runtime::Handle,
}

impl GuitarLoopApp {
    // ------------------------------------------------------------------
    // Konstruktor - dipanggil eframe::run_native
    // ------------------------------------------------------------------
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setup styling awal (opsional)
        setup_custom_styles(&cc.egui_ctx);

        // Spawn Tokio runtime global (untuk player/db async tasks)
        let tokio_rt = match tokio::runtime::Handle::try_current() {
            Ok(handle) => handle,
            Err(_) => {
                let rt = tokio::runtime::Runtime::new()
                    .expect("Failed to create Tokio runtime");
                let handle = rt.handle().clone();
                // Leak runtime supaya hidup selama app berjalan
                std::mem::forget(rt);
                handle
            }
        };

        log::info!("App state initialized");

        Self {
            playback_state: PlaybackState::Stopped,
            current_position: Duration::ZERO,
            speed: 1.0,
            volume: 1.0,
            pitch_correction: true,
            loop_markers: LoopMarkers::default(),
            metadata: VideoMetadata::default(),
            open_file_dialog: false,
            status_message: "Siap. Pilih File > Open untuk memuat video.".to_string(),
            tokio_rt,
        }
    }

    // ------------------------------------------------------------------
    // Helper: buka file dialog
    // ------------------------------------------------------------------
    pub fn open_file(&mut self) {
        self.open_file_dialog = true;
    }

    // ------------------------------------------------------------------
    // Helper: set status bar
    // ------------------------------------------------------------------
    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = msg.into();
        log::info!("Status: {}", self.status_message);
    }

    // ------------------------------------------------------------------
    // Helper: reset playback state ketika buka file baru
    // ------------------------------------------------------------------
    pub fn reset_playback(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.current_position = Duration::ZERO;
        self.loop_markers = LoopMarkers::default();
        self.metadata = VideoMetadata::default();
    }
}

// ----------------------------------------------------------------------
// eframe App trait
// ----------------------------------------------------------------------
impl eframe::App for GuitarLoopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ---- Top menu bar ----
        crate::ui::menu::render(self, ctx);

        // ---- Main central panel (video area + controls) ----
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::main_panel::render(self, ui);
        });

        // ---- Bottom status bar ----
        crate::ui::status_bar::render(self, ctx);

        // ---- Side panel: practice tools ----
        egui::SidePanel::right("tools_panel")
            .default_width(280.0)
            .resizable(true)
            .show(ctx, |ui| {
                crate::ui::tools_panel::render(self, ui);
            });

        // ---- File dialog handling ----
        if self.open_file_dialog {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter(
                    "Video / Audio",
                    &["mp4", "mkv", "avi", "mov", "webm", "mp3", "wav", "ogg", "flac"],
                )
                .set_title("Buka file video/audio latihan")
                .pick_file()
            {
                self.reset_playback();
                self.metadata.path = Some(path.clone());
                self.metadata.title = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                self.playback_state = PlaybackState::Loading;
                self.set_status(format!(
                    "Memuat: {}",
                    path.to_string_lossy()
                ));
                // TODO: pass ke player module untuk actual load
            }
            self.open_file_dialog = false;
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        log::info!("GuitarLoop shutting down...");
    }
}

// ----------------------------------------------------------------------
// setup_custom_styles
// ----------------------------------------------------------------------
fn setup_custom_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Warna tema gitar - warm brown/orange
    style.visuals.dark_mode = true;
    style.visuals.widgets.noninteractive.bg_stroke.color =
        egui::Color32::from_rgb(80, 55, 30);
    style.visuals.hyperlink_color = egui::Color32::from_rgb(255, 150, 50);
    style.visuals.selection.bg_fill = egui::Color32::from_rgb(180, 100, 30);
    style.visuals.faint_bg_color = egui::Color32::from_rgb(35, 28, 22);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(22, 18, 15);

    ctx.set_style(style);
}

// ======================================================================
// GuitarLoop - main.rs
// Entry point aplikasi: setup logging, inisialisasi GStreamer,
// dan spawn window eframe/egui.
// ======================================================================

use std::env;

mod app;
mod db;
mod player;
mod ui;
mod utils;

use app::GuitarLoopApp;
use eframe::egui;

// ----------------------------------------------------------------------
// main
// ----------------------------------------------------------------------
fn main() -> eframe::Result<()> {
    // 1. Setup logger (RUST_LOG=info / debug / trace)
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "guitarloop=info,warn");
    }
    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp_millis()
        .init();

    log::info!("=================================================");
    log::info!("  GuitarLoop v{} starting...", env!("CARGO_PKG_VERSION"));
    log::info!("=================================================");

    // 2. Inisialisasi GStreamer (penting: harus sebelum pakai API gst apapun)
    if let Err(e) = gstreamer::init() {
        log::error!("GStreamer init failed: {}", e);
        eprintln!("ERROR: GStreamer tidak bisa diinisialisasi: {}", e);
        eprintln!("Pastikan GStreamer runtime sudah terinstall (lihat README).");
        std::process::exit(1);
    }
    let gst_ver = gstreamer::version_string();
    log::info!("GStreamer version: {}", gst_ver);

    // 3. Konfigurasi window eframe
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 750.0])
            .with_min_inner_size([800.0, 500.0])
            .with_title("GuitarLoop - Practice Player")
            .with_icon(load_icon()),
        persist_window: true,
        centered: true,
        ..Default::default()
    };

    // 4. Run eframe event loop
    log::info!("Spawning eframe window...");
    eframe::run_native(
        "GuitarLoop",
        native_options,
        Box::new(move |cc| Ok(Box::new(GuitarLoopApp::new(cc)))),
    )
}

// ----------------------------------------------------------------------
// load_icon - load icon dari assets (opsional, fallback default)
// ----------------------------------------------------------------------
fn load_icon() -> egui::IconData {
    // Jika assets/icon.png ada, pakai; selain itu pakai placeholder 1x1
    match std::fs::read("assets/icon.png") {
        Ok(bytes) => match image::load_from_memory(&bytes) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                egui::IconData {
                    rgba: rgba.into_raw(),
                    width: w,
                    height: h,
                }
            }
            Err(_) => egui::IconData {
                rgba: vec![0u8; 4],
                width: 1,
                height: 1,
            },
        },
        Err(_) => egui::IconData {
            rgba: vec![0u8; 4],
            width: 1,
            height: 1,
        },
    }
}

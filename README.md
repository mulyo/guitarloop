# 🎸 GuitarLoop

> **Video player khusus latihan gitar** dengan loop presisi A→B, speed control tanpa pitch shift, waveform visualization, metronome, dan transpose.
>
> Dibangun dengan **Rust 2021**, **egui + eframe** untuk GUI, **GStreamer** untuk playback, **SQLite** untuk database, dan **Tokio** async runtime.

---

## ✨ Fitur (Roadmap per Fase)

| Fase | Versi | Fitur |
|------|-------|-------|
| Fase 1 | MVP | Open video lokal (MP4/MKV/AVI/MOV/WEBM), Play/Pause/Stop, Seek, Speed 0.25×–2.0× |
| Fase 2 | v1.0 | Mark A/B drag, Loop A→B repeat, Waveform, Metronome, Transpose ±12 semitone, Speed Ramp |
| Fase 3 | v1.1 | Chord detection, Chord diagram overlay, Tab overlay sync, Pitch detection, Tuner |
| Fase 4 | v1.2 | User account, Cloud sync preset & progress, Multi-device |
| Fase 5 | v1.3 | YouTube import via yt-dlp, Cache management, Download queue |

> Lihat `PRD.md` untuk detail lengkap 13 fase development.

---

## 🛠 Tech Stack

| Komponen | Teknologi |
|----------|-----------|
| Language | Rust (edition 2021) |
| GUI | [egui](https://github.com/emilk/egui) + [eframe](https://crates.io/crates/eframe) 0.28 |
| Media Playback | [GStreamer](https://gstreamer.freedesktop.org/) 1.22+ via [gstreamer-rs](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs) 0.22 |
| Database | [SQLite](https://www.sqlite.org/) via [rusqlite](https://github.com/rusqlite/rusqlite) 0.32 **(bundled, no system SQLite required)** |
| Async | [Tokio](https://tokio.rs/) 1.x (full features) |
| File Dialog | [rfd](https://github.com/PolyMeilex/rfd) 0.14 (native dialog) |

---

## 📁 Struktur Project

```
guitarloop/
├── Cargo.toml                # Konfigurasi crate + dependencies
├── PRD.md                    # Product Requirements Document
├── README.md                 # File ini
│
├── src/
│   ├── main.rs               # Entry point: init logger + GStreamer + spawn eframe window
│   ├── app.rs                # Core state GuitarLoopApp + eframe::App impl
│   │
│   ├── player/
│   │   └── mod.rs            # GStreamerPlayer: load/play/pause/seek/speed/volume
│   │
│   ├── ui/
│   │   ├── mod.rs            # Re-export submodule UI
│   │   ├── menu.rs           # Top menu bar (File, Playback, Tools, Help)
│   │   ├── main_panel.rs     # Video area + transport control + seekbar + timecode
│   │   ├── tools_panel.rs    # Practice tools (speed, volume, loop A→B, metronome, transpose)
│   │   └── status_bar.rs     # Bottom status bar (state + message + DB status)
│   │
│   ├── db/
│   │   └── mod.rs            # SQLite init + schema + migration + CRUD video_library
│   │
│   └── utils/
│       └── mod.rs            # Time format, clamp, volume curve, human_bytes, media ext
│
└── assets/                   # Icon, font, gambar, resource statis (opsional)
    └── icon.png              # Window icon (jika ada)
```

---

## 🔧 Prerequisites (Build & Run)

### 🪟 Windows (direkomendasikan)

1. **Rust toolchain** (stable)
   ```powershell
   winget install Rustlang.Rustup
   rustup default stable
   rustc --version   # harusnya >= 1.75
   ```

2. **GStreamer 1.22+ runtime + dev package** (KRITIS — tanpa ini `cargo build` akan gagal link)

   **Cara termudah (official installer):**
   - Download **MSVC 64-bit** (runtime + development installers) dari:
     👉 https://gstreamer.freedesktop.org/download/#windows
   - Install **kedua file** (urutan tidak masalah):
     1. `gstreamer-1.0-msvc-x86_64-1.22.x.msi` (runtime) — centang **"Complete"** / install semua fitur
     2. `gstreamer-1.0-devel-msvc-x86_64-1.22.x.msi` (dev files) — centang **"Complete"**
   - Set environment variable (**PowerShell**):
     ```powershell
     $env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = "C:\gstreamer\1.0\msvc_x86_64"
     # Tambahkan ke PATH
     $env:PATH = "C:\gstreamer\1.0\msvc_x86_64\bin;" + $env:PATH
     ```
   - **Permanen** (set via System Properties → Environment Variables) atau tambahkan ke `$PROFILE`.
   - Verify:
     ```powershell
     gst-launch-1.0 --version
     # harus muncul: gst-launch-1.0 version 1.22.x
     ```

3. **Build tools C/C++ (MSVC)** — biasanya sudah terpasang saat install rustup.
   Jika belum:
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   # Di installer, centang "Desktop development with C++"
   ```

### 🍎 macOS

```bash
# 1. Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. GStreamer via Homebrew
brew install gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly gst-libav pkg-config

# 3. Verify
pkg-config --modversion gstreamer-1.0   # harusnya >= 1.22
```

### 🐧 Ubuntu / Debian

```bash
# 1. Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. GStreamer + dev headers
sudo apt update && sudo apt install -y \
  libgstreamer1.0-0 libgstreamer1.0-dev \
  gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
  gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
  gstreamer1.0-libav gstreamer1.0-tools \
  libgst-play-1.0-0 libgst-play-1.0-dev \
  libgtk-3-dev pkg-config build-essential

# 3. Verify
gst-launch-1.0 --version
pkg-config --modversion gstreamer-1.0
```

---

## 🚀 Build & Run

### ⚡ Development build (cepat, untuk debugging)

```bash
# Pertama kali: set env GStreamer (Windows PowerShell contoh)
$env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = "C:\gstreamer\1.0\msvc_x86_64"
$env:PATH = "C:\gstreamer\1.0\msvc_x86_64\bin;" + $env:PATH

# Build
cargo build

# Langsung run
cargo run

# Dengan logging detail
$env:RUST_LOG="guitarloop=debug,info"
cargo run
```

### 🔨 Release build (optimized — untuk distribusi)

```bash
cargo build --release
# Hasil binary di: target/release/guitarloop.exe (Windows)
#                       target/release/guitarloop      (macOS/Linux)
```

### 🧪 Testing

```bash
# Jalankan semua unit test (termasuk module utils)
cargo test

# Test module tertentu
cargo test --lib utils

# Lihat output log ketika test
cargo test -- --nocapture
```

### 📋 Clippy lint

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 📦 Database Location

SQLite database otomatis dibuat di lokasi berikut (sesuai OS):

| OS      | Path |
|---------|------|
| Windows | `%APPDATA%\GuitarLoop\data\guitarloop.db` |
| macOS   | `~/Library/Application Support/GuitarLoop/data/guitarloop.db` |
| Linux   | `~/.local/share/GuitarLoop/data/guitarloop.db` |

**Schema awal (v1):**
- `_meta` — metadata key-value (schema_version dll)
- `video_library` — daftar video yang pernah dibuka + play count + favorit
- `loop_presets` — preset Mark A/B per video

---

## ⌨ Default Keyboard Shortcuts

| Shortcut  | Action |
|-----------|--------|
| `Space`   | Play / Pause toggle |
| `←`       | Seek -5 detik |
| `→`       | Seek +5 detik |
| `S`       | Stop |
| `A`       | Set Mark A (posisi sekarang) |
| `B`       | Set Mark B (posisi sekarang) |
| `Ctrl+O`  | Open file dialog |
| `Alt+F4`  | Exit aplikasi |

> Implementasi shortcut global akan ditambahkan di Fase 2 (saat ini lewat menu).

---

## 🐛 Troubleshooting

### ❌ `GStreamer tidak bisa diinisialisasi` saat startup

Pastikan **GStreamer runtime terinstall** AND `bin` folder-nya ada di `%PATH%`. Cek:
```powershell
gst-launch-1.0 --version
```
Jika command tidak ditemukan: **reinstall GStreamer** dan verify env var.

### ❌ `cargo build` error linking `gstreamer-1.0.lib` (Windows)

1. Pastikan **file dev installer GStreamer** juga di-install (bukan runtime saja).
2. Pastikan env var `GSTREAMER_1_0_ROOT_MSVC_X86_64` di-set **sebelum** `cargo build`.
3. Restart terminal / IDE setelah set env var.
4. Jalankan `cargo clean && cargo build`.

### ❌ Video tidak play (hanya placeholder hitam)

1. Cek codec: video MP4 dengan H.264 + AAC paling aman.
2. Install gst-plugins-ugly / gst-libav untuk codec tambahan.
3. Check log dengan `RUST_LOG=debug cargo run`.

### ❌ Database rusak / corupt

Hapus file `guitarloop.db` di lokasi di atas. Aplikasi akan recreate schema fresh saat start berikutnya (video library hilang, tapi file video di-disk tetap aman).

---

## 📄 License

MIT License — lihat file `LICENSE` (jika ada).

---

## 🤝 Kontribusi

1. Fork repo ini
2. Buat feature branch (`git checkout -b fitur/loop-presets`)
3. Commit perubahan (`git commit -am 'tambah preset loop'`)
4. Push ke branch (`git push origin fitur/loop-presets`)
5. Buka Pull Request

---

**🎯 Next step setelah setup sukses?** Lanjut ke **Fase 1 MVP** — integrate `player::GStreamerPlayer` dengan `app.rs` state dan hookup video sink ke area video egui OpenGL texture!


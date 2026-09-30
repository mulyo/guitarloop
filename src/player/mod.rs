// ======================================================================
// player/mod.rs
// GStreamer-based media player module.
// Placeholder untuk implementasi playback di Fase 1 MVP.
// ======================================================================

use anyhow::Result;
use gstreamer::prelude::*;
use std::path::Path;
use std::time::Duration;

// Playbin flag bitmask (dari gstplay-enum.h GstPlayFlags)
//   VIDEO = 0x01, AUDIO = 0x02, TEXT = 0x04, VIS = 0x08, SOFT_VOLUME = 0x10, NATIVE_AUDIO = 0x20, NATIVE_VIDEO = 0x40, DOWNLOAD = 0x80, BUFFERING = 0x100, DEINTERLACE = 0x200, SOFT_COLORBALANCE = 0x400, FORCE_FILTERS = 0x800, FORCE_SW_DECODERS = 0x1000
const PLAYBIN_FLAGS_DEFAULT: u32 = 0x01 | 0x02 | 0x04; // VIDEO | AUDIO | TEXT

// ----------------------------------------------------------------------
// GStreamerPlayer - struct utama player
// ----------------------------------------------------------------------
pub struct GStreamerPlayer {
    playbin: Option<gstreamer::Element>,
}

impl Default for GStreamerPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl GStreamerPlayer {
    // ------------------------------------------------------------------
    pub fn new() -> Self {
        log::debug!("GStreamerPlayer::new()");
        Self { playbin: None }
    }

    // ------------------------------------------------------------------
    /// Build pipeline playbin untuk file target.
    /// TODO: integrasi dengan egui area (OpenGL sink) nanti di Fase 1.
    // ------------------------------------------------------------------
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        log::info!("GStreamerPlayer::load -> {}", path_str);

        // Gunakan playbin3 jika tersedia, fallback ke playbin lama.
        // ElementFactory::make() -> ElementBuilder, jadi kita .build() dulu baru match Result.
        let playbin = match gstreamer::ElementFactory::make("playbin3").build() {
            Ok(el) => el,
            Err(_) => {
                gstreamer::ElementFactory::make("playbin")
                    .build()
                    .map_err(|e| anyhow::anyhow!("GStreamer playbin not found: {}", e))?
            }
        };

        // Uri format: file:///absolute/path
        let uri = format!(
            "file://{}",
            path.as_ref()
                .canonicalize()?
                .to_string_lossy()
                .replace('\\', "/")
        );
        playbin.set_property("uri", &uri);

        // Set flag: video + audio + text (gunakan raw u32 bitmask, stabil di semua versi)
        playbin.set_property("flags", &PLAYBIN_FLAGS_DEFAULT);

        self.playbin = Some(playbin);
        Ok(())
    }

    // ------------------------------------------------------------------
    pub fn play(&mut self) -> Result<()> {
        if let Some(pb) = &self.playbin {
            pb.set_state(gstreamer::State::Playing)?;
            log::debug!("play() -> Playing");
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    pub fn pause(&mut self) -> Result<()> {
        if let Some(pb) = &self.playbin {
            pb.set_state(gstreamer::State::Paused)?;
            log::debug!("pause() -> Paused");
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    pub fn stop(&mut self) -> Result<()> {
        if let Some(pb) = &self.playbin {
            pb.set_state(gstreamer::State::Ready)?;
            log::debug!("stop() -> Ready");
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    pub fn seek(&mut self, position: Duration) -> Result<()> {
        if let Some(pb) = &self.playbin {
            let clock_time = gstreamer::ClockTime::from_nseconds(
                position.as_nanos().try_into().unwrap_or(0),
            );
            pb.seek_simple(
                gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT,
                clock_time,
            )?;
            log::trace!("seek -> {:?}", position);
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    pub fn set_speed(&mut self, speed: f32) -> Result<()> {
        if let Some(_pb) = &self.playbin {
            // TODO: implement seek with rate (pitch correction via scaletempo/pitch)
            log::debug!("set_speed -> {:.2}x (stub)", speed);
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        if let Some(pb) = &self.playbin {
            pb.set_property("volume", &(volume as f64));
            log::trace!("set_volume -> {:.2}", volume);
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    pub fn query_position(&self) -> Option<Duration> {
        let pb = self.playbin.as_ref()?;
        let pos = pb.query_position::<gstreamer::ClockTime>()?;
        Some(Duration::from_nanos(pos.nseconds()))
    }

    // ------------------------------------------------------------------
    pub fn query_duration(&self) -> Option<Duration> {
        let pb = self.playbin.as_ref()?;
        let dur = pb.query_duration::<gstreamer::ClockTime>()?;
        Some(Duration::from_nanos(dur.nseconds()))
    }
}

// Safety: GStreamer elements itu thread-safe di level C API.
unsafe impl Send for GStreamerPlayer {}
unsafe impl Sync for GStreamerPlayer {}

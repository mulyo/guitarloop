// ======================================================================
// utils/mod.rs - Shared utilities: time format, math, path helpers
// ======================================================================

use std::path::Path;
use std::time::Duration;

// ----------------------------------------------------------------------
/// Format Duration ke string `MM:SS.mmm` atau `HH:MM:SS.mmm`
// ----------------------------------------------------------------------
pub fn format_duration_precise(d: Duration) -> String {
    let total = d.as_secs();
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    let ms = d.subsec_millis();
    if h > 0 {
        format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
    } else {
        format!("{:02}:{:02}.{:03}", m, s, ms)
    }
}

// ----------------------------------------------------------------------
/// Format Duration tanpa ms (MM:SS / HH:MM:SS)
// ----------------------------------------------------------------------
pub fn format_duration_short(d: Duration) -> String {
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

// ----------------------------------------------------------------------
/// Parse string "MM:SS" atau "HH:MM:SS" ke Duration.
/// Return None jika invalid format.
// ----------------------------------------------------------------------
pub fn parse_timecode(s: &str) -> Option<Duration> {
    let parts: Vec<&str> = s.split(':').collect();
    let total_secs: u64 = match parts.len() {
        2 => {
            let m: u64 = parts[0].parse().ok()?;
            let sec: f64 = parts[1].parse().ok()?;
            m * 60 + sec as u64
        }
        3 => {
            let h: u64 = parts[0].parse().ok()?;
            let m: u64 = parts[1].parse().ok()?;
            let sec: f64 = parts[2].parse().ok()?;
            h * 3600 + m * 60 + sec as u64
        }
        _ => return None,
    };
    Some(Duration::from_secs(total_secs))
}

// ----------------------------------------------------------------------
/// Clamp `value` ke range `[lo, hi]` (generic, untuk f32/f64/i32 etc).
// ----------------------------------------------------------------------
pub fn clamp<T: PartialOrd>(value: T, lo: T, hi: T) -> T {
    if value < lo {
        lo
    } else if value > hi {
        hi
    } else {
        value
    }
}

// ----------------------------------------------------------------------
/// Normalize `value` (0..1) ke audio logarithmic volume curve
/// (karena telinga manusia logaritmis).
// ----------------------------------------------------------------------
pub fn linear_to_log_volume(linear: f32) -> f32 {
    let l = clamp(linear, 0.0, 1.0);
    if l <= 0.0 {
        0.0
    } else {
        (l * l).sqrt() // simple log-ish approximation
    }
}

// ----------------------------------------------------------------------
/// Beri representasi ukuran file human-readable (KB, MB, GB).
// ----------------------------------------------------------------------
pub fn human_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let i = (bytes as f64).log(1024.0).floor() as usize;
    let i = i.min(UNITS.len() - 1);
    let v = bytes as f64 / 1024_f64.powi(i as i32);
    format!("{:.2} {}", v, UNITS[i])
}

// ----------------------------------------------------------------------
/// Ekstensi file yang didukung untuk video/audio latihan.
// ----------------------------------------------------------------------
pub fn is_supported_media<P: AsRef<Path>>(p: P) -> bool {
    match p
        .as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some(
            "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv" | "wmv" | "m4v" | "mpg" | "mpeg"
            | "mp3" | "wav" | "flac" | "ogg" | "opus" | "m4a" | "aac" | "wma",
        ) => true,
        _ => false,
    }
}

// ----------------------------------------------------------------------
/// File size dari path (helper)
// ----------------------------------------------------------------------
pub fn file_size<P: AsRef<Path>>(p: P) -> std::io::Result<u64> {
    Ok(std::fs::metadata(p)?.len())
}

// ======================================================================
// Unit tests (cargo test --lib utils)
// ======================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(
            format_duration_short(Duration::from_secs(65)),
            "01:05"
        );
        assert_eq!(
            format_duration_short(Duration::from_secs(3661)),
            "01:01:01"
        );
    }

    #[test]
    fn test_parse_timecode() {
        assert_eq!(
            parse_timecode("01:30"),
            Some(Duration::from_secs(90))
        );
        assert_eq!(
            parse_timecode("1:02:03"),
            Some(Duration::from_secs(3723))
        );
        assert!(parse_timecode("invalid").is_none());
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(100u32, 0, 10), 10);
    }

    #[test]
    fn test_human_bytes() {
        assert!(human_bytes(0).starts_with('0'));
        assert!(human_bytes(1024).contains("KB"));
        assert!(human_bytes(1048576).contains("MB"));
    }
}

// ======================================================================
// db/mod.rs - SQLite (rusqlite) module
// Menyimpan library video, practice history, preset loop, chord data.
// ======================================================================

use anyhow::{Context, Result};
use directories::ProjectDirs;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

// ----------------------------------------------------------------------
// Schema version - increment untuk migration
// ----------------------------------------------------------------------
const SCHEMA_VERSION: u32 = 1;

// ----------------------------------------------------------------------
// VideoLibraryEntry - satu baris di tabel video_library
// ----------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoLibraryEntry {
    pub id: Option<i64>,
    pub file_path: String,
    pub title: String,
    pub duration_ms: Option<i64>,
    pub last_played_ms: Option<i64>,
    pub play_count: i64,
    pub favorite: bool,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ----------------------------------------------------------------------
// Global DB connection wrapper
// ----------------------------------------------------------------------
static DB_CONN: std::sync::OnceLock<Mutex<Option<Connection>>> =
    std::sync::OnceLock::new();

// ----------------------------------------------------------------------
// database_available - cek apakah db sudah diinisialisasi
// ----------------------------------------------------------------------
pub fn database_available() -> bool {
    DB_CONN
        .get()
        .and_then(|m| m.lock().ok())
        .map(|guard| guard.is_some())
        .unwrap_or(false)
}

// ----------------------------------------------------------------------
// db_path() - lokasi file SQLite sesuai OS
//   Windows: %APPDATA%\GuitarLoop\data\guitarloop.db
//   macOS:   ~/Library/Application Support/GuitarLoop/data/guitarloop.db
//   Linux:   ~/.local/share/GuitarLoop/data/guitarloop.db
// ----------------------------------------------------------------------
pub fn db_path() -> Result<PathBuf> {
    let proj = ProjectDirs::from("com", "GuitarLoop", "GuitarLoop")
        .context("Tidak bisa menentukan direktori aplikasi (ProjectDirs)")?;
    let dir = proj.data_dir().join("data");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Gagal buat direktori DB: {}", dir.display()))?;
    Ok(dir.join("guitarloop.db"))
}

// ----------------------------------------------------------------------
// init() - buka koneksi + buat schema jika belum ada
// Bisa dipanggil beberapa kali (idempotent).
// ----------------------------------------------------------------------
pub fn init() -> Result<()> {
    let path = db_path()?;
    log::info!("SQLite DB path: {}", path.display());

    let conn =
        Connection::open(&path).with_context(|| format!("Buka DB gagal: {}", path.display()))?;

    // Enable WAL untuk concurrent read + write
    conn.pragma_update(None, "journal_mode", "WAL")
        .context("Set journal_mode=WAL gagal")?;
    conn.pragma_update(None, "foreign_keys", "ON")
        .context("Enable foreign_keys gagal")?;

    apply_schema(&conn)?;
    migrate(&conn)?;

    // Simpan ke global
    let _ = DB_CONN.set(Mutex::new(Some(conn)));
    log::info!("SQLite initialized (schema v{})", SCHEMA_VERSION);
    Ok(())
}

// ----------------------------------------------------------------------
// with_conn - helper untuk execute closure dengan koneksi DB
// ----------------------------------------------------------------------
pub fn with_conn<F, R>(f: F) -> Result<R>
where
    F: FnOnce(&Connection) -> Result<R>,
{
    let guard = DB_CONN
        .get()
        .context("DB belum diinisialisasi - panggil db::init()")?
        .lock()
        .map_err(|_| anyhow::anyhow!("DB mutex poisoned"))?;
    let conn = guard.as_ref().context("DB connection tidak tersedia")?;
    f(conn)
}

// ----------------------------------------------------------------------
// apply_schema - DDL awal
// ----------------------------------------------------------------------
fn apply_schema(conn: &Connection) -> Result<()> {
    // Tabel metadata (untuk versioning)
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _meta (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );"
    )?;

    // Tabel library video
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS video_library (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            file_path       TEXT    NOT NULL UNIQUE,
            title           TEXT    NOT NULL,
            duration_ms     INTEGER,
            last_position_ms INTEGER DEFAULT 0,
            last_played_ms  INTEGER,
            play_count      INTEGER NOT NULL DEFAULT 0,
            favorite        INTEGER NOT NULL DEFAULT 0,
            tags            TEXT,
            created_at      TEXT    NOT NULL DEFAULT (datetime('now')),
            updated_at      TEXT    NOT NULL DEFAULT (datetime('now'))
        );"
    )?;

    // Tabel loop preset (Mark A/B per video)
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS loop_presets (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            video_id    INTEGER NOT NULL REFERENCES video_library(id) ON DELETE CASCADE,
            name        TEXT    NOT NULL,
            mark_a_ms   INTEGER NOT NULL,
            mark_b_ms   INTEGER NOT NULL,
            speed       REAL    NOT NULL DEFAULT 1.0,
            repeat_count INTEGER NOT NULL DEFAULT 0,
            created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
        );"
    )?;

    // Index
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_lib_title ON video_library(title);",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_presets_video ON loop_presets(video_id);",
        [],
    )?;

    Ok(())
}

// ----------------------------------------------------------------------
// migrate - cek & update schema version
// ----------------------------------------------------------------------
fn migrate(conn: &Connection) -> Result<()> {
    let cur: Option<String> = conn
        .query_row("SELECT value FROM _meta WHERE key = 'schema_version'", [], |r| {
            r.get(0)
        })
        .optional()?;

    match cur {
        None => {
            conn.execute(
                "INSERT INTO _meta(key, value) VALUES ('schema_version', ?1)",
                params![SCHEMA_VERSION.to_string()],
            )?;
            log::debug!("Fresh DB: schema_version = {}", SCHEMA_VERSION);
        }
        Some(v) => {
            let vnum: u32 = v.parse().unwrap_or(0);
            if vnum < SCHEMA_VERSION {
                log::warn!(
                    "Schema outdated (v{}) < current (v{}): migration placeholder",
                    vnum,
                    SCHEMA_VERSION
                );
                conn.execute(
                    "UPDATE _meta SET value = ?1 WHERE key = 'schema_version'",
                    params![SCHEMA_VERSION.to_string()],
                )?;
            }
        }
    }
    Ok(())
}

// ----------------------------------------------------------------------
// insert_or_update_video - upsert ke video_library
// ----------------------------------------------------------------------
pub fn insert_or_update_video(entry: &VideoLibraryEntry) -> Result<i64> {
    with_conn(|conn| {
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO video_library
                (file_path, title, duration_ms, last_played_ms, play_count, favorite, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(file_path) DO UPDATE SET
                title = excluded.title,
                duration_ms = COALESCE(excluded.duration_ms, video_library.duration_ms),
                last_played_ms = COALESCE(excluded.last_played_ms, video_library.last_played_ms),
                play_count = video_library.play_count + CASE WHEN excluded.play_count > 0 THEN 1 ELSE 0 END,
                favorite = excluded.favorite,
                tags = COALESCE(excluded.tags, video_library.tags),
                updated_at = ?9
            ",
            params![
                entry.file_path,
                entry.title,
                entry.duration_ms,
                entry.last_played_ms,
                entry.play_count,
                entry.favorite as i32,
                entry.tags,
                now,
                now,
            ],
        )?;
        let id = conn.last_insert_rowid();
        Ok(id)
    })
}

// ----------------------------------------------------------------------
// list_all_videos - query seluruh library
// ----------------------------------------------------------------------
pub fn list_all_videos() -> Result<Vec<VideoLibraryEntry>> {
    with_conn(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id, file_path, title, duration_ms, last_played_ms,
                    play_count, favorite, tags, created_at, updated_at
             FROM video_library
             ORDER BY COALESCE(last_played_ms, 0) DESC, created_at DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(VideoLibraryEntry {
                id: Some(r.get(0)?),
                file_path: r.get(1)?,
                title: r.get(2)?,
                duration_ms: r.get(3)?,
                last_played_ms: r.get(4)?,
                play_count: r.get(5)?,
                favorite: r.get::<_, i32>(6)? != 0,
                tags: r.get(7)?,
                created_at: r.get(8)?,
                updated_at: r.get(9)?,
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    })
}

// ----------------------------------------------------------------------
// get_video_by_path - cari by file path (unique)
// ----------------------------------------------------------------------
pub fn get_video_by_path<P: AsRef<Path>>(p: P) -> Result<Option<VideoLibraryEntry>> {
    let path_str = p.as_ref().to_string_lossy().to_string();
    with_conn(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT id, file_path, title, duration_ms, last_played_ms,
                    play_count, favorite, tags, created_at, updated_at
             FROM video_library WHERE file_path = ?1",
        )?;
        let row = stmt
            .query_row(params![path_str], |r| {
                Ok(VideoLibraryEntry {
                    id: Some(r.get(0)?),
                    file_path: r.get(1)?,
                    title: r.get(2)?,
                    duration_ms: r.get(3)?,
                    last_played_ms: r.get(4)?,
                    play_count: r.get(5)?,
                    favorite: r.get::<_, i32>(6)? != 0,
                    tags: r.get(7)?,
                    created_at: r.get(8)?,
                    updated_at: r.get(9)?,
                })
            })
            .optional()?;
        Ok(row)
    })
}

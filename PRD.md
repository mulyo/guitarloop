# PRODUCT REQUIREMENTS DOCUMENT (PRD)
# GuitarLoop -- Video Player untuk Latihan Gitar

**Versi:** 1.0
**Tanggal:** 2026
**Author:** Founder GuitarLoop
**Status:** Draft untuk Development

---

## 1. EXECUTIVE SUMMARY

GuitarLoop adalah aplikasi video/audio player yang dirancang khusus 
untuk gitaris yang ingin berlatih dengan efektif. Berbeda dengan 
video player biasa (VLC, MPC-HC) atau aplikasi belajar gitar 
(Yousician, Fender Play), GuitarLoop fokus pada **latihan repetitif 
bagian sulit** dengan kontrol presisi.

**Value Proposition:**
- Loop A→B dengan presisi (Mark1/Mark2 yang bisa digeser)
- Speed control 0.25× - 2.0× tanpa pitch shift
- Waveform & chord detection otomatis
- Metronome & transpose terintegrasi
- Sync antar device (desktop, mobile)
- Komunitas untuk sharing preset latihan

**Target User:**
- Gitaris beginner sampai advanced
- Usia 15-45 tahun
- Punya laptop/PC + smartphone
- Latihan minimal 3× seminggu
- Familiar dengan YouTube untuk belajar

**Business Model:**
- Freemium: Free tier + Premium (4.99/bulan)+Pro(9.99/bulan)
- Marketplace preset (creator dapat 85%)
- Tips untuk streamer (platform fee 5%)
- Affiliate program

---

## 2. PROBLEM STATEMENT

### Masalah yang Ada:
1. **Video player biasa tidak punya loop presisi** -- VLC loop 
   seluruh video, bukan segmen tertentu.
2. **YouTube tidak bisa slow down dengan pitch correction** -- 
   speed 0.5× bikin suara aneh.
3. **Aplikasi belajar gitar terlalu mahal** -- Yousician $20/bulan, 
   Fender Play $10/bulan.
4. **Tidak ada tool untuk analisis lagu** -- chord detection, 
   waveform, transpose terpisah-pisah.
5. **Tidak bisa sync progress antar device** -- latihan di desktop, 
   lanjut di HP harus setup ulang.

### Solusi GuitarLoop:
- Loop A→B dengan Mark1/Mark2 yang bisa di-drag
- Speed control dengan pitch correction (via GStreamer/libmpv)
- Waveform + chord detection + pitch detection
- Metronome + transpose + tuner terintegrasi
- Cloud sync (opt-in) untuk multi-device
- Komunitas untuk sharing preset

---

## 3. TARGET USER (PERSONA)

### Persona 1: Budi -- Beginner (60% user)
- **Umur:** 22 tahun, mahasiswa
- **Skill:** Baru belajar gitar 6 bulan
- **Goal:** Bisa main Wonderwall, Hotel California
- **Pain:** Susah slow down YouTube tanpa suara aneh
- **Device:** Laptop Windows + Android
- **Budget:** Rp 50.000/bulan max

### Persona 2: Siti -- Intermediate (30% user)
- **Umur:** 28 tahun, karyawan
- **Skill:** 3 tahun main, bisa solo dasar
- **Goal:** Improve speed & accuracy
- **Pain:** Susah latihan solo cepat, butuh loop per section
- **Device:** MacBook + iPhone + iPad
- **Budget:** $10/bulan

### Persona 3: Agus -- Advanced (10% user)
- **Umur:** 35 tahun, semi-pro
- **Skill:** 10+ tahun, main di band
- **Goal:** Transkripsi lagu, belajar teknik baru
- **Pain:** Butuh tab generation otomatis
- **Device:** Windows + audio interface
- **Budget:** $20/bulan

---

## 4. FITUR UTAMA (MVP → V2)

### Fase 1: Video Player Dasar (MVP)
- Buka file video lokal (MP4, MKV, AVI, MOV, WEBM)
- Play, pause, stop, seek
- Speed control 0.25× - 2.0×
- Audio tetap keluar di semua speed

### Fase 2: Practice Tools (V1.0)
- Mark1 & Mark2 (set, drag, delete)
- Loop A→B dengan repeat count
- Waveform audio visualization
- Metronome terintegrasi
- Transpose -12 s/d +12 semitone
- Speed ramp

### Fase 3: AI & Analysis (V1.1)
- Chord detection otomatis
- Chord diagram overlay
- Tab overlay & sync
- Pitch detection
- Tuning detector
- Note segmentation

### Fase 4: Cloud Sync (V1.2)
- User account
- Sync preset & progress
- Multi-device support
- API backend

### Fase 5: YouTube Import (V1.3)
- Import dari YouTube via yt-dlp
- Cache management
- Download queue
- Subtitle download

### Fase 6: Community (V2.0)
- User profile
- Follow system
- Feed aktivitas
- Preset sharing
- Comments & reviews

### Fase 7: Live Streaming (V2.1)
- Go live dari aplikasi
- Nonton live stream
- Real-time chat
- Scheduled events

### Fase 8: Mobile Companion (V2.2)
- Android & iOS app
- Audio-only practice mode
- Background playback
- Sync dengan desktop

### Fase 9: AI Personalization (V2.3)
- Adaptive practice tutor
- Style detection
- Auto-generate tab
- Smart recommendations

### Fase 10: Marketplace (V3.0)
- Premium subscription
- Preset marketplace
- Tips untuk streamer
- Affiliate program

### Fase 11: Hardware Integration (V3.1)
- Foot pedal support
- Audio interface
- MIDI controller
- Expression pedal

### Fase 12: Education Platform (V3.2)
- Structured courses
- Certification
- Teacher marketplace

### Fase 13: Global Expansion (V4.0)
- Multi-language
- Multi-currency
- Regional compliance

---

## 5. TECHNICAL ARCHITECTURE

### Desktop (Windows/macOS/Linux)
- **Language:** Rust
- **GUI:** egui + eframe
- **Media:** GStreamer (dengan pitch correction)
- **Audio Output:** WASAPI (Windows), CoreAudio (macOS)
- **Database:** SQLite (via rusqlite)
- **Cache:** File-based + SQLite metadata

### Mobile (Android/iOS)
- **Framework:** Flutter
- **State Management:** Riverpod
- **Audio:** just_audio + audio_service
- **Database:** Drift (SQLite-based)
- **Sync:** HTTP client + background sync

### Backend
- **Language:** Rust + Axum
- **Database:** PostgreSQL + Redis
- **Storage:** S3-compatible (MinIO/R2)
- **Payment:** Stripe (global) + Midtrans (Indonesia)
- **Streaming:** MediaMTX + FFmpeg

---

## 6. SUCCESS METRICS

### Product Metrics
- Daily Active Users (DAU)
- Weekly Active Users (WAU)
- Retention (D1, D7, D30)
- Session duration
- Feature adoption rate
- Practice session per user per week

### Business Metrics
- Monthly Recurring Revenue (MRR)
- Churn rate (< 5%/bulan)
- Customer Acquisition Cost (CAC)
- Lifetime Value (LTV)
- LTV:CAC ratio (> 3:1)
- Conversion rate (free → paid > 3%)

### Technical Metrics
- Crash rate (< 1%)
- Audio latency (< 30ms)
- Cold start (< 2s)
- Battery usage (< 10%/jam)
- Sync success rate (> 99%)

---

## 7. ROADMAP

| Fase | Target | Estimasi |
|------|--------|----------|
| Fase 1: Video Player | Q1 2026 | 2-3 minggu |
| Fase 2: Practice Tools | Q1 2026 | 3-4 minggu |
| Fase 3: AI & Analysis | Q2 2026 | 4-6 minggu |
| Fase 4: Cloud Sync | Q2 2026 | 3-4 minggu |
| Fase 5: YouTube Import | Q2 2026 | 2-3 minggu |
| Fase 6: Community | Q3 2026 | 4-6 minggu |
| Fase 7: Live Streaming | Q3 2026 | 6-8 minggu |
| Fase 8: Mobile | Q4 2026 | 8-10 minggu |
| Fase 9: AI Personalization | Q4 2026 | 6-8 minggu |
| Fase 10: Marketplace | Q1 2027 | 6-8 minggu |
| Fase 11: Hardware | Q1 2027 | 4-6 minggu |
| Fase 12: Education | Q2 2027 | 8-10 minggu |
| Fase 13: Global | Q2 2027 | 6-8 minggu |

---

## 8. RISIKO & MITIGASI

| Risiko | Dampak | Mitigasi |
|--------|--------|----------|
| GStreamer rumit di Windows | Tinggi | Bundle runtime, dokumentasi |
| AI detection tidak akurat | Sedang | Beri confidence score |
| Biaya streaming mahal | Tinggi | CDN cost-effective, batasi free tier |
| DMCA / copyright | Tinggi | Content ID, takedown process |
| Churn tinggi | Tinggi | Engagement, community |
| Legal LKP Indonesia | Sedang | Konsultasi Dinas Pendidikan |
| Payment gateway reject | Sedang | Multiple gateway |

---

## 9. COMPETITOR ANALYSIS

| Kompetitor | Harga | Kelebihan | Kekurangan |
|------------|-------|-----------|------------|
| Yousician | $20/bulan | Gamified, kursus | Mahal, tidak bisa pakai lagu sendiri |
| Fender Play | $10/bulan | Brand kuat | Terbatas Fender |
| Simply Guitar | $15/bulan | Mobile-first | Tidak ada desktop |
| Transcribe! | $39 sekali | Fitur lengkap | UI kuno |
| Amazing Slow Downer | $50 sekali | Audio only | Tidak ada video |
| Songsterr | $10/bulan | Tab lengkap | Bukan player |

**Positioning GuitarLoop:** 
"Video player + practice tools + community, dengan harga 
terjangkau. Bisa pakai lagu apapun (lokal atau YouTube)."

---

## 10. LEGAL & COMPLIANCE

### Indonesia
- **Badan Usaha:** PT Perorangan (opsi termurah & tercepat)
- **NPWP Badan:** Wajib
- **PKP:** Wajib jika omzet > Rp4.8M
- **PPN:** 11% untuk produk digital
- **TD PSE:** Wajib untuk aplikasi digital
- **KBLI:** 58290 (Penerbitan Perangkat Lunak) + 62199
- **LKP:** Wajib untuk fitur education

### Global
- **GDPR:** Data user EU di EU
- **CCPA:** Disclosure untuk user California
- **COPPA:** Age verification untuk minor
- **DMCA:** Takedown process untuk copyright

### Etika
- Privacy-first: default private
- Moderasi UGC: report, block, mute
- Minor protection: parent consent
- No dark pattern

---

## 11. TIM & RESOURCE

### Fase MVP (Fase 1-2)
- 1 Rust developer (full-time)
- 1 Designer (part-time)
- 1 QA (part-time)

### Fase Growth (Fase 3-6)
- 2 Rust developers
- 1 Flutter developer
- 1 Designer
- 1 Community manager
- 1 Support

### Fase Scale (Fase 7-13)
- 3 Rust developers
- 2 Flutter developers
- 1 DevOps
- 1 Designer
- 2 Community managers
- 2 Support
- 1 Marketing
- 1 Legal/Finance

---

## 12. LAMPIRAN

- [Daftar Prompt per Fase]
- [Database Schema]
- [API Specification]
- [UI Wireframes]
- [Financial Projection]


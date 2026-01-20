# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.16.2] - 2026-01-20

### Fixed
- Panic while skip streams.

## [0.16.1] - 2026-01-03

### Added

- `--auto-encs` (restored, was removed in 0.16.0).

## [0.16.0] - 2026-01-01

### Added

- Common sort internal & external fonts and attachs.
- Static build via `ffmpeg-static` and `ffmpeg-build` features.

### Changed

- Auto `default`: first track of each type.
- Decrease default jobs to 1 (was 4).
- Indexes of each stream type starts from 0.
- Rename `--json` to `--load`.
- Rename `--threads` to `--jobs`.
- Replace `--rm-segments` to `--parts`.
- Replace dependencies on external tools to ffmpeg library.
- Short `--list-langs` to ISO639-1 and extend auto-langs to its.
- Support any format language string.

### Removed

- External tools (ffprobe, ffmpeg and mkvmerge).
- Output containers except Matroska.
- Reencoding.
- Tracks (streams) `enabled` flags.
- `--auto-charsets`.
- `--raws`.
- `--chapters`.

### Fixed

- MediaInfo durations.
- Retiming SSA/ASS subtitles.
- Retiming single part.
- Retiming sync.


## [0.15.0] - 2025-09-27

### Added

- Parallel muxing.
- Limitation parallel muxing with the `--threads <n>` option.
- Retiming (restored, was removed in 0.13.0).
- Remove external linked Matroska segments with the `--no-linked` flag.
- Remove Matroska segments with the `--rm-segments <n[,m]...>` option.

### Changed

- Auto-read `mux-media.json` config.
- Save config with the `--save-config` flag.
- Load config with the `--json <json>` option.
- Rename `--specials` option to `--raws`.
- Short version of `--locale <lng>` option.
- Short version of `--target <trg>...` option.

### Removed

- Button tracks settings.


## [0.14.3] - 2025-08-18

### Added

- Support for single-media groups with the `--solo` flag.


## [0.14.2] - 2025-08-15

### Fixed

- Remove created directories on failure.


## [0.14.1] - 2025-08-10

### Fixed

- Publication on crates.io.


## [0.14.0] - 2025-08-10

### Added

- Reencoding.


## [0.13.6] - 2025-07-25

## [0.13.5] - 2025-07-25

## [0.13.4] - 2025-07-21

## [0.13.3] - 2025-07-18

## [0.13.2] - 2025-07-17

## [0.13.1] - 2025-07-16

## [0.13.0] - 2025-07-15

### Changed

- First Rust version.
- Rename to mux-media.

### Removed

- Retiming.


## [0.12.7] - 2025-04-19 [YANKED]

## [0.12.6] - 2025-04-18 [YANKED]

## [0.12.5] - 2025-04-06 [YANKED]

## [0.12.4] - 2025-04-06 [YANKED]

## [0.12.3] - 2025-03-27 [YANKED]

## [0.12.2] - 2025-03-27 [YANKED]

## [0.12.1] - 2025-03-27 [YANKED]

## [0.12.0] - 2025-03-27 [YANKED]

## [0.11.3] - 2025-02-12 [YANKED]

## [0.11.2] - 2025-02-11 [YANKED]

## [0.11.1] - 2025-02-10 [YANKED]

## [0.11.0] - 2025-02-09 [YANKED]

## [0.10.8] - 2025-01-25 [YANKED]

## [0.10.7] - 2025-01-24 [YANKED]

## [0.10.6] - 2025-01-23 [YANKED]

## [0.10.5] - 2025-01-23 [YANKED]

## [0.10.4] - 2025-01-22 [YANKED]

## [0.10.3] - 2025-01-22 [YANKED]

## [0.10.2] - 2025-01-22 [YANKED]

## [0.10.1] - 2025-01-21 [YANKED]

## [0.10.0] - 2025-01-21 [YANKED]

## [0.9.3] - 2025-01-13 [YANKED]

## [0.9.2] - 2025-01-12 [YANKED]

## [0.9.1] - 2025-01-12 [YANKED]

## [0.9.0] - 2025-01-12 [YANKED]

## [0.8.1] - 2025-01-11 [YANKED]

## [0.8.0] - 2025-01-11 [YANKED]

## [0.7.5] - 2025-01-09 [YANKED]

## [0.7.4] - 2025-01-06 [YANKED]

## [0.7.3] - 2025-01-05 [YANKED]

## [0.7.2] - 2025-01-04 [YANKED]

## [0.7.1] - 2025-01-04 [YANKED]

## [0.7.0] - 2025-01-03 [YANKED]

## [0.6.3] - 2024-12-06 [YANKED]

## [0.6.2] - 2024-12-06 [YANKED]

## [0.6.1] - 2024-12-06 [YANKED]

## [0.6.0] - 2024-12-05 [YANKED]

## [0.5.8] - 2024-12-05 [YANKED]

## [0.5.7] - 2024-11-30 [YANKED]

## [0.5.6] - 2024-11-30 [YANKED]

## [0.5.5] - 2024-11-29 [YANKED]

## [0.5.4] - 2024-11-28 [YANKED]

## [0.5.3] - 2024-11-28 [YANKED]

## [0.5.2] - 2024-11-27 [YANKED]

## [0.5.1] - 2024-11-25 [YANKED]

## [0.5.0] - 2024-11-22 [YANKED]

## [0.4.11] - 2024-11-03 [YANKED]

## [0.4.10] - 2024-11-03 [YANKED]

## [0.4.9] - 2024-11-02 [YANKED]

## [0.4.8] - 2024-11-02 [YANKED]

## [0.4.7] - 2024-11-01 [YANKED]

## [0.4.6] - 2024-11-01 [YANKED]

## [0.4.5] - 2024-10-31 [YANKED]

## [0.4.4] - 2024-10-30 [YANKED]

## [0.4.3] - 2024-10-29 [YANKED]

## [0.4.2] - 2024-10-26 [YANKED]

## [0.4.1] - 2024-10-26 [YANKED]

## [0.4.0] - 2024-10-24 [YANKED]

## [0.3.2] - 2024-10-13 [YANKED]

## [0.3.1] - 2024-09-22 [YANKED]

## [0.3.0] - 2024-09-20 [YANKED]

## [0.2.2] - 2024-09-17 [YANKED]

## [0.2.1] - 2024-09-14 [YANKED]

## [0.2.0] - 2024-09-14 [YANKED]

## [0.1.0] - 2024-09-05 [YANKED]

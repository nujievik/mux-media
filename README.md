# mux-media

A CLI utility for muxing media (e.g. video, audio, subtitles).

[![Tests](https://github.com/nujievik/mux-media/actions/workflows/tests.yml/badge.svg)](https://github.com/nujievik/mux-media/actions/workflows/tests.yml)


## Quick Start

1. [Download](https://github.com/nujievik/mux-media/releases) the
archive for your system. [For Windows see old version](
https://github.com/nujievik/mux-media/releases/download/v0.15.0/mux-media-win64-full.zip).
2. Unpack it.
3. Run the unpacked `mux-media` in a media directory.


## Default Behaviour

- Saves muxed files to the `muxed` subdirectory.
- Muxes all media extension files in a CWD directory and 16 its
subdirectories.
- Muxes all files with a same filename prefix 
(eg., **Death Note - 01**.mkv and **Death Note - 01**.eng.aac) to
single MKV-container.
- Skips orphan files without an other same filename prefix file.
- Attachs all founded `*.OTF` & `*.TTF` fonts to each output MKV.

## Supported Extensions 📁

### Input

| File Type     | Extensions                                                                      |
|---------------|---------------------------------------------------------------------------------|
| Media         | 264 265 3GP AAC AC3 ASS AV1 AVC AVI CAF DTS DTSHD EAC3 EC3 F4V FLAC FLV H264 H265 HEVC IVF M2TS M2V M4A M4V MKA MKS MKV MLP MOV MP2 MP3 MP4 MPA MPEG MPG MPV OBU OGG OGM OGV OPUS RA SRT SSA SUB SUP THD TRUEHD TS TTA VC1 VTT WAV WEBA WEBM WMA WMV X264 X265 |
| Font          | OTF TTF |

### Output

MKV container.


## Advanced Use 🤓

Run `mux-media -h` to display help.

|                              | Description                                                      |
|------------------------------|------------------------------------------------------------------|
| I/O options:                 | |
| `-i, --input <dir>` | Top-level media directory |
| `-o, --output <out[,put]>` | Output paths pattern: `out{num}[put]` |
| `-r, --range <n[-m]>` | Number range of media-files |
| `--skip <n[,m]...>` | Patterns of files to skip | 
| `--depth <n>` | Scan subdirectories up to this depth |
| `--solo` | Process media without external tracks |
| | |
| Global options: | |
| `-l, --locale <lng>` | Locale language (for logging and sort) |
| `-j, --jobs <n>` | Max parallel muxing |
| `-v, --verbose...` | Increase logging |
| `-q, --quiet` | Suppress logging |
| `-e, --exit-on-err` | Skip muxing next files if error occurs |
| `--load <json>` | Load config from JSON |
| `--save-config` | Save config to JSON in the input directory |
| | |
| Auto flags: | |
| `-p, --pro` | Disable all auto below |
| `--auto-defaults / --no-auto-defaults` | Auto set default flags |
| `--auto-forceds / --no-auto-forceds` | Auto set forced flags |
| `--auto-names / --no-auto-names` | Auto set stream names |
| `--auto-langs / --no-auto-langs` | Auto set stream langs |
| | |
| Save streams: | |
| `-a, --audio <[!]n[,m]...>` | `[!]Save audio streams` |
| `-A, --no-audio` | Don't save any audio stream |
| `-s, --subs <[!]n[,m]...>` | `[!]Save subtitle streams` |
| `-S, --no-subs` | Don't save any subtitle stream |
| `-d, --video <[!]n[,m]...>` | `[!]Save video streams` |
| `-D, --no-video` | Don't save any video stream |
| `-f, --fonts <[!]n[,m]...>` | `[!]Save font attachments` |
| `-F, --no-fonts` | Don't save any font attachment |
| `-m, --attachs <[!]n[,m]...>` | `[!]Save other attachments` |
| `-M, --no-attachs` | Don't save any other attachment |
| | |
| Target options:
| `-t, --target <trg>...` | Set next options for target |
| `--list-targets` | Show supported targets |
| `--streams <[!]n[,m]...>` | `[!]Save streams` |
| `--no-streams` | Don't save any stream |
| `-C, --no-chapters` | Don't save chapters |
| `--defaults <[n:]B[,m:B]...>` | Set default flags |
| `--max-defaults <n>` | Max auto-enabled default |
| `--forceds <[n:]B[,m:B]...>` | Set forced flags |
| `--max-forceds <n>` | Max auto-enabled forced |
| `--names <[n:]N[,m:N]...>` | Set stream names |
| `--langs <[n:]L[,m:L]...>` | Set stream languages |
| | |
| Retiming options: | |
| `--parts <[!]n[,m]...>` | `[!]Save parts for chapter names` |
| `--no-linked` | Remove matroska linked parts |
| | |
| Other options: | |
| `--list-langs` | Show supported language codes |
| `-V, --version` | Show version |
| `-h, --help` | Show help |

Alternative CLI, you can configure a [JSON file](
https://github.com/nujievik/mux-media/blob/main/mux-media.json) in a
top-level media directory.


## Manual Build 🤓

See examples in [workflows](
https://github.com/nujievik/mux-media/blob/main/.github/workflows).

### Shared Build

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. [Configure ffmpeg-next build](
https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building)

3. Clone the repo:
```
git clone https://github.com/nujievik/mux-media --depth 1
```

4. Enter the project directory:
```
cd mux-media
```

5. Build:
```
cargo build --release --locked
```

6. On success, the binary will be in `target/release/mux-media`

### Static Build

Use ffmpeg-build feature:
```
cargo build --release --locked --features ffmpeg-build
```

Fucking ffmpeg-next not builds on Windows in current. Use v0.15.0
version or shared build instead.


## Alternative GUI Utilities

There are alternative utilities with user-friendly GUI interfaces,
though they offer less automation:

- [mkv-muxing-batch-gui](
https://github.com/yaser01/mkv-muxing-batch-gui) - advanced GUI

- [py-mkvmerger-auto](https://github.com/LedyBacer/py-mkvmergre-auto) -
simple GUI

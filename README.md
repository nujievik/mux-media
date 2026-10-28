# mux-media

CLI utility to mux (merge) video, audio, and subtitles.

[![Tests](https://github.com/nujievik/mux-media/actions/workflows/tests.yml/badge.svg)](
https://github.com/nujievik/mux-media/actions/workflows/tests.yml)


## Quick Start

1. [Download](https://github.com/nujievik/mux-media/releases) an
archive for your system.
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
| `-i, --input <path>` | Set input file OR directory with files |
| `-o, --output <dir>` | Save muxed files to directory |
| `-w, --overwrite` | Overwrite input files |
| `-r, --range <n[-m]>` | Mux media with filename in-range only |
| `    --skip <n[,m]...>` | Set patterns of filepaths to skip |
| `    --depth <n>` | Scan subdirectories up to this depth |
| `    --orphans-too` | Mux orphan mediafiles too |
| | |
| Global options: | |
| `-l, --locale <lng>` | Set priority streams language & log language |
| `    --locale-streams <lng>` | Set priority streams language only |
| `    --locale-log <lng>` | Set log language only (eng or rus) |
| `-v, --verbose...` | Increase logging |
| `-q, --quiet` | Suppress logging |
| `-j, --jobs <n>` | Set max parallel muxing |
| `-e, --exit-on-err` | Skip muxing next files if error occurs |
| `    --load <txt>` | Load .txt config |
| `    --save-config` | Save config to .txt in the input directory |
| | |
| Auto flags: | |
| `    --no-auto` | Disable all auto below |
| `    --auto-defaults / --no-auto-defaults` | Auto set default flags |
| `    --auto-forceds / --no-auto-forceds` | Auto set forced flags |
| `    --auto-titles / --no-auto-titles` | Auto set stream titles |
| `    --auto-langs / --no-auto-langs` | Auto set stream langs |
| `    --auto-encs / --no-auto-encs` | Auto set subs encodings |
| `    --auto-ts-offsets` | Auto set audio ts-offsets |
| `    --no-auto-ts-offsets` | No auto set audio ts-offsets |
| | |
| Save streams: | |
| `-a, --audio <[!]n[,m]...>` | `[!]Save audio streams` |
| `-A, --no-audio` | Don't save any audio stream |
| `-s, --subs <[!]n[,m]...>` | `[!]Save subtitle streams` |
| `-S, --no-subs` | Don't save any subtitle stream |
| `-d, --video <[!]n[,m]...>` | `[!]Save video streams` |
| `-D, --no-video` | Don't save any video stream |
| `-f, --fonts <[!]n[,m]...>` | `[!]Save attached fonts` |
| `-F, --no-fonts` | Don't save any attached font |
| `-p, --pics <[!]n[,m]...>` | `[!]Save attached pictures` |
| `-P, --no-pics` | Don't save any attached picture |
| `    --clean-flags` | Save known flags of streams only |
| `    --clean-meta` | Save known metadata of streams only |
| | |
| Target options:
| `-t, --target <trg>...` | Set next options for target |
| `    --list-targets` | Show supported targets |
| `    --streams <[!]n[,m]...>` | `[!]Save streams` |
| `    --no-streams` | Don't save any stream |
| `-c, --chapters <[T:]n-m...>` | Set chapters |
| `-C, --no-chapters` | Don't save chapters |
| `    --order <[n:]V[,m:V]...>` | Set streams order |
| `    --defaults <[n:]B[,m:B]...>` | Set default flags |
| `    --max-defaults <n>` | Max auto-enabled default |
| `    --forceds <[n:]B[,m:B]...>` | Set forced flags |
| `    --max-forceds <n>` | Max auto-enabled forced |
| `    --titles <[n:]T[,m:T]...>` | Set stream titles |
| `    --langs <[n:]L[,m:L]...>` | Set stream languages |
| `    --subs-encoding <n>` | Decode subs as n-encoding |
| `    --ts-offsets <[n:]T[,m:T]...>` | Set streams timestamps offsets (ms) |
| | |
| Retiming options: | |
| `    --parts <[!]n[,m]...>` | `[!]Save parts for chapter names` |
| `    --no-linked` | Remove matroska linked parts |
| | |
| Other options: | |
| `    --list-langs` | Show supported language codes |
| `-V, --version` | Show version |
| `-h, --help` | Show help |

Alternatively, you can create a [config file](
https://github.com/nujievik/mux-media/blob/main/.mux-media/config.txt)
in the .mux-media subdirectory of your media directory.


## Alternative GUI Utilities

There are alternative utilities with user-friendly GUI interfaces,
though they offer less automation:

- [mkv-muxing-batch-gui](
https://github.com/yaser01/mkv-muxing-batch-gui) - advanced GUI

- [py-mkvmerger-auto](https://github.com/LedyBacer/py-mkvmergre-auto) -
simple GUI

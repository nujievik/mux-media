# mux-media

A simple automated solution for muxing media (e.g. video, audio,
subtitles).

## How to use

### Windows

1. [Download](https://github.com/nujievik/mux-media/releases) the
archive for your system (`mux-media-win64.zip` or `mux-media-win32.zip`)

2. Unpack it

3. Run unpacked `mux-media.exe` in the media directory

### Other system

1. Install [MKVToolNix](https://mkvtoolnix.download/)

2. [Download](https://github.com/nujievik/mux-media/releases) the
archive for your system

3. Unpack it

4. Run unpacked `mux-media` in the media directory

Or you can [build manually](#manual-build).

Usage `mux-media -h` for help.

Default settings can be overridden:
- via CLI arguments
- by configuring a [JSON file](
https://github.com/nujievik/mux-media/blob/main/mux-media.json) in a
media directory and loading it using `mux-media -j`

## Notices

- Media files must share the same filename prefix
(eg. **Death Note - 01**.mkv and **Death Note - 01**.eng.aac)

- Media is searched in:
  - the start directory
  - all its subdirectories up to given depth (default 16)

## Manual Build

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. Clone the repo:
```
git clone https://github.com/nujievik/mux-media
```

3. Enter the project directory:
```
cd mux-media
```

4. Build:
```
cargo build --release
```

5. On success, the binary will be in `target/release/mux-media`

## Alternative GUI solutions

There are alternative solutions with user-friendly GUI interfaces,
though they offer less automation:

- [mkv-muxing-batch-gui](
https://github.com/yaser01/mkv-muxing-batch-gui) - advanced GUI

- [py-mkvmergre-auto](https://github.com/LedyBacer/py-mkvmergre-auto) -
simple GUI

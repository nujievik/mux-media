# mux-media

A simple automated solution for muxing media (e.g. video, audio,
subtitles).


## Quick start

### Windows

1. [Download](https://github.com/nujievik/mux-media/releases) the full
archive for your Windows system
(`mux-media-win64-full.zip` or `mux-media-win32-full.zip`)

2. Unpack it

3. Run the unpacked `mux-media.exe` in the media directory

### Other system

1. Install [MKVToolNix](https://mkvtoolnix.download/)

2. [Download](https://github.com/nujievik/mux-media/releases) the
archive for your system

3. Unpack it

4. Run the unpacked `mux-media` in the media directory


## Custom settings

Use `mux-media -h` to display help.

Custom settings can be specified:

- via CLI arguments

- by configuring a [JSON file](
https://github.com/nujievik/mux-media/blob/main/mux-media.json) in a
media directory and loading it using `mux-media -j`


## Notices

- Media files must share the same filename prefix.
(eg. **Death Note - 01**.mkv and **Death Note - 01**.eng.aac)

- Media is searched in:
  - the start directory
  - all its subdirectories up to the given depth (default 16)

- Container:
  - default is Matroska (.mkv)
  - other custom containers may not support all options

- Re-encoding:
  - by default, no re-encoding is performed
  
  - custom containers may re-encode unsupported tracks - use only if
  necessary, as this may significantly degrade quality.


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


### Features with_embedded_bins

Available **only for Windows** builds (x86_64-pc-windows and
i686-pc-windows). Embeds `ffmpeg.exe` and `mkvmerge.exe` into the
binary.

1. Ensure that `ffmpeg.exe` and `mkvmerge.exe` are available in
`mux-media/assets/` for the current target:

  - Automatic (from system PATH):

    - Make sure `ffmpeg` and `mkvmerge` are available in your system's
      PATH.

    - The `mux-media/build.rs` script will automatically copy the
      required binaries from PATH.

  - Manually copy `ffmpeg.exe` and `mkvmerge.exe` to either:

    - `mux-media/assets/win64` for x86_64-pc-windows target
    
    - `mux-media/assets/win32` for i686-pc-windows target

2. Follow steps 1–3 from [Manual Build](#manual-build).

3. Build with the feature:
```
cargo build --release --features with_embedded_bins
```

4. On success, the binary will be in `target/release/mux-media`.


## Alternative GUI solutions

There are alternative solutions with user-friendly GUI interfaces,
though they offer less automation:

- [mkv-muxing-batch-gui](
https://github.com/yaser01/mkv-muxing-batch-gui) - advanced GUI

- [py-mkvmerger-auto](https://github.com/LedyBacer/py-mkvmergre-auto) -
simple GUI

# mux-media

A simple automated solution for muxing media (e.g. video, audio,
subtitles).


## Quick Start

### Windows

1. [Download](https://github.com/nujievik/mux-media/releases) the
**full** archive for your Windows system
(`mux-media-win64-full.zip` or `mux-media-win32-full.zip`).

2. Unpack it.

3. Run the unpacked `mux-media.exe` in a media directory.

### Other Systems

1. Install [MKVToolNix](https://mkvtoolnix.download/).

2. [Download](https://github.com/nujievik/mux-media/releases) the
archive for your system.

3. Unpack it.

4. Run the unpacked `mux-media` in a media directory.


## Notices

- Media files must share the same filename prefix.
(eg., **Death Note - 01**.mkv and **Death Note - 01**.eng.aac)

- Media is searched in:
  - the start directory
  - all its subdirectories up to the given depth (default: 16)
  

## Advanced Use 🤓

Run `mux-media -h` to display help.

Custom settings can be specified:

- via CLI arguments

- by configuring a [JSON file](
https://github.com/nujievik/mux-media/blob/main/mux-media.json) in a
media directory and loading it using `mux-media -j`

### Windows

- The **full** version for Windows includes bundled `mkvmerge` and
`ffmpeg`.

- Use the system's `mkvmerge` and `ffmpeg` by running
`mux-media --user-tools`.

- The non-**full** version for Windows requires manually installing
[MKVToolNix](https://mkvtoolnix.download/) and [FFmpeg](
https://ffmpeg.org/) (for custom containers).

### Custom Output Containers

- The default container is Matroska (.mkv).

- Other supported containers: `.avi`, `.mp4`, `.webm`.

- Install [FFmpeg](https://ffmpeg.org/) to use a custom container if
you are use not using the **full** version.

- Custom containers may require reencoding of unsupported tracks - use
only if necessary, as this can significantly degrade quality.

### Reencoding

- By default, no reencoding is performed.

- When using a custom container, reencoding is automatically performed
if needed.

- Use `mux-media --reencode` with a custom container to force
reencoding.


## Manual Build 🤓

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


## Alternative GUI Solutions

There are alternative solutions with user-friendly GUI interfaces,
though they offer less automation:

- [mkv-muxing-batch-gui](
https://github.com/yaser01/mkv-muxing-batch-gui) - advanced GUI

- [py-mkvmerger-auto](https://github.com/LedyBacer/py-mkvmergre-auto) -
simple GUI

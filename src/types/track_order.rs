mod new;
mod to_args;

use crate::{ArcPathBuf, TrackType};
use std::path::PathBuf;

/// A sorted order of the media files and track.
///
/// # Sorting Rules
///
/// Sorts by the following rules (from highest to lowest priority):
///
/// 1. `TrackType`:
///     - `Video`
///     - `Audio`
///     - `Sub`
///     - `Button`
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("track_type");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     # let args = [Path::new("-i"), &dir];
///     # let cfg = MuxConfig::parse_from(args);
///     #
///     let video = ArcPathBuf::from(dir.join("3.mkv"));
///     let audio = ArcPathBuf::from(dir.join("1.ogg"));
///     let subs = ArcPathBuf::from(dir.join("2.srt"));
///
///     let mut mi = MediaInfo::from(&cfg);
///     mi.try_insert(subs.clone()).unwrap();
///     mi.try_insert(audio.clone()).unwrap();
///     mi.try_insert(video.clone()).unwrap();
///
///     let m = TrackOrder::try_from(&mut mi).unwrap();
///     assert_eq!(&m[0].media, &video);
///     assert_eq!(&m[1].media, &audio);
///     assert_eq!(&m[2].media, &subs);
///     ```
///
/// 2. `TrackFlagType::Default`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     let first = ArcPathBuf::from(dir.join("1.srt"));
///     let second = ArcPathBuf::from(dir.join("2.srt"));
///
///     let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--defaults"), p("true")];
///     let cfg = MuxConfig::parse_from(args);
///
///     let mut mi = MediaInfo::from(&cfg);
///     mi.try_insert(first.clone()).unwrap();
///     mi.try_insert(second.clone()).unwrap();
///
///     let m = TrackOrder::try_from(&mut mi).unwrap();
///     assert_eq!(&m[0].media, &second);
///     assert_eq!(&m[1].media, &first);
///     ```
///
/// 3. `TrackFlagType::Forced`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = ArcPathBuf::from(dir.join("1.srt"));
///     # let second = ArcPathBuf::from(dir.join("2.srt"));
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--forceds"), p("true")];
///     # let cfg = MuxConfig::parse_from(args);
///     #
///     # let mut mi = MediaInfo::from(&cfg);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let m = TrackOrder::try_from(&mut mi).unwrap();
///     assert_eq!(&m[0].media, &second);
///     assert_eq!(&m[1].media, &first);
///     ```
///
/// 4. `TrackFlagType::Enabled`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = ArcPathBuf::from(dir.join("1.srt"));
///     # let second = ArcPathBuf::from(dir.join("2.srt"));
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--enableds"), p("true")];
///     # let cfg = MuxConfig::parse_from(args);
///     #
///     # let mut mi = MediaInfo::from(&cfg);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let m = TrackOrder::try_from(&mut mi).unwrap();
///     assert_eq!(&m[0].media, &second);
///     assert_eq!(&m[1].media, &first);
///     ```
///
/// 5.  It `Sub` track and signs:
///     - `true`
///     - `false`.
///
///     Its affected only `Sub` tracks if they has same 1-4.
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("3");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     let first = ArcPathBuf::from(dir.join("1.srt"));
///     let second = ArcPathBuf::from(dir.join("1_signs.srt"));
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir];
///     # let cfg = MuxConfig::parse_from(args);
///     #
///     # let mut mi = MediaInfo::from(&cfg);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let m = TrackOrder::try_from(&mut mi).unwrap();
///     assert_eq!(&m[0].media, &second);
///     assert_eq!(&m[1].media, &first);
///     ```
///
/// 6. Track language `LangCode`:
///     - `locale` language
///     - `Und` (undefined language)
///     - Other languages (excluding `Jpn`)
///     - `Jpn` (Japanese)
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = ArcPathBuf::from(dir.join("1.srt"));
///     # let second = ArcPathBuf::from(dir.join("2.srt"));
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let mut args = vec![p("-i"), &dir, p("--locale"), p("eng")];
///     args.extend([p("--target"), &second, p("--langs"), p("eng")]);
///     # let cfg = MuxConfig::parse_from(args);
///     #
///     # let mut mi = MediaInfo::from(&cfg);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let m = TrackOrder::try_from(&mut mi).unwrap();
///     assert_eq!(&m[0].media, &second);
///     assert_eq!(&m[1].media, &first);
///     ```
///
/// 7. Path name.
///
///    ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = ArcPathBuf::from(dir.join("1.srt"));
///     # let second = ArcPathBuf::from(dir.join("2.srt"));
///     #
///     let args = [Path::new("-i"), &dir];
///     # let cfg = MuxConfig::parse_from(args);
///     #
///     # let mut mi = MediaInfo::from(&cfg);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let m = TrackOrder::try_from(&mut mi).unwrap();
///     assert_eq!(&m[0].media, &first);
///     assert_eq!(&m[1].media, &second);
///     ```
#[derive(Clone, Debug)]
pub struct TrackOrder(pub Vec<TrackOrderItem>);

/// A [`TrackOrder`] item.
#[derive(Clone, Debug)]
pub struct TrackOrderItem {
    /// A media file existing before the program started.
    pub media: ArcPathBuf,

    /// A unique number assigned to the media at its first appearance in the [`TrackOrder`] vector.
    /// NOT an index in the vector.
    /// For example, `vec[2].number == 1` if `vec[0].media == vec[1].media`.
    pub number: u64,

    /// Whether this is the first occurrence of the media in the vector.
    pub is_first_entry: bool,

    /// The track number within the media.
    pub track: u64,

    /// The type of the track.
    pub ty: TrackType,

    /// Retimed parts of the track, if any.
    pub retimed: Option<TrackOrderItemRetimed>,
}

#[derive(Clone, Debug)]
pub struct TrackOrderItemRetimed {
    pub parts: Vec<PathBuf>,
    pub no_retiming: Vec<bool>,
    pub ty: TrackType,
}

deref_singleton_tuple_struct!(TrackOrder, Vec<TrackOrderItem>);

impl TrackOrder {
    pub(crate) fn iter_first_entries(&self) -> impl Iterator<Item = &TrackOrderItem> {
        self.0.iter().filter(|m| m.is_first_entry)
    }
}

impl TrackOrderItemRetimed {
    pub(crate) fn new(parts: Vec<PathBuf>, no_retiming: bool, ty: TrackType) -> Self {
        let no_retiming = vec![no_retiming; parts.len()];
        Self {
            parts,
            no_retiming,
            ty,
        }
    }
}

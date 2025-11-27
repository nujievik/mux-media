mod new;
mod to_ffmpeg_args;

use crate::{ArcPathBuf, Duration, StreamType};
use std::path::{Path, PathBuf};

/// A sorted order of streams.
///
/// # Sorting Rules
///
/// Sorts by the following rules (from highest to lowest priority):
///
/// 1. [`StreamType`]:
///     - `Video`
///     - `Audio`
///     - `Sub`
///     - `Other`
///     - `Font`
///     - `Attach`
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("streams_order")
///     #     .join("ty");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     # let args = [Path::new("-i"), &dir];
///     # let cfg = Config::parse_from(args);
///     #
///     let vid_attach = dir.join("vid_x1_attach_x1.mkv");
///     let audio = dir.join("ogg.ogg");
///     let sub_font = dir.join("sub_x1_font_x1.mks");
///
///     let mut mi = MediaInfo::new(&cfg, 0);
///     mi.try_insert(&vid_attach).unwrap();
///     mi.try_insert(&audio).unwrap();
///     mi.try_insert(&sub_font).unwrap();
///     let o = StreamsOrder::new(&mut mi).unwrap();
///
///     assert_eq!(o[0].ty, StreamType::Video);
///     assert_eq!(o[1].ty, StreamType::Audio);
///     assert_eq!(o[2].ty, StreamType::Sub);
///     assert_eq!(o[3].ty, StreamType::Font);
///     assert_eq!(o[4].ty, StreamType::Attach);
///
///     assert_eq!(&o[0].src(), &vid_attach);
///     assert_eq!(&o[1].src(), &audio);
///     assert_eq!(&o[2].src(), &sub_font);
///     assert_eq!(&o[3].src(), &sub_font);
///     assert_eq!(&o[4].src(), &vid_attach);
///     ```
///
/// 2. Dispositions `default` value:
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
///     #     .join("streams_order")
///     #     .join("srt_pair");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     let first = dir.join("1.srt");
///     let second = dir.join("2.srt");
///
///     let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--defaults"), p("true")];
///     let cfg = Config::parse_from(args);
///
///     let mut mi = MediaInfo::new(&cfg, 0);
///     mi.try_insert(&first).unwrap();
///     mi.try_insert(&second).unwrap();
///     let o = StreamsOrder::new(&mut mi).unwrap();
///
///     assert_eq!(&o[0].src(), &second);
///     assert_eq!(&o[1].src(), &first);
///     ```
///
/// 3. Dispositions `forced` value:
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
///     #     .join("streams_order")
///     #     .join("srt_pair");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = dir.join("1.srt");
///     # let second = dir.join("2.srt");
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--forceds"), p("true")];
///     # let cfg = Config::parse_from(args);
///     #
///     # let mut mi = MediaInfo::new(&cfg, 0);
///     # mi.try_insert(&first).unwrap();
///     # mi.try_insert(&second).unwrap();
///     # let o = StreamsOrder::new(&mut mi).unwrap();
///     #
///     assert_eq!(&o[0].src(), &second);
///     assert_eq!(&o[1].src(), &first);
///     ```
///
/// 4. It `Sub` stream and has an signs-key in stream name or Path:
///     - true
///     - false
///
///    Its affected only `Sub` streams if they has same 1-3.
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("streams_order")
///     #     .join("srt_pair");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = dir.join("1.srt");
///     # let second = dir.join("2.srt");
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--names"), p("has key signs")];
///     # let cfg = Config::parse_from(args);
///     #
///     # let mut mi = MediaInfo::new(&cfg, 0);
///     # mi.try_insert(&first).unwrap();
///     # mi.try_insert(&second).unwrap();
///     mi.try_finalize_init_streams().unwrap();
///     # let o = StreamsOrder::new(&mut mi).unwrap();
///     #
///     assert_eq!(&o[0].src(), &second);
///     assert_eq!(&o[1].src(), &first);
///     ```
///
/// 5. Stream language:
///     - Same as `locale` language
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
///     #     .join("streams_order")
///     #     .join("srt_pair");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = dir.join("1.srt");
///     # let second = dir.join("2.srt");
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let mut args = vec![p("-i"), &dir, p("--locale"), p("eng")];
///     args.extend([p("--target"), &second, p("--langs"), p("eng")]);
///     # let cfg = Config::parse_from(args);
///     #
///     # let mut mi = MediaInfo::new(&cfg, 0);
///     # mi.try_insert(&first).unwrap();
///     # mi.try_insert(&second).unwrap();
///     mi.try_finalize_init_streams().unwrap();
///     # let o = StreamsOrder::new(&mut mi).unwrap();
///     #
///     assert_eq!(&o[0].src(), &second);
///     assert_eq!(&o[1].src(), &first);
///     ```
///
/// 6. Path name.
///
///     ```
///     # use mux_media::*;
///     # use clap::Parser;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("streams_order")
///     #     .join("srt_pair");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = dir.join("1.srt");
///     # let second = dir.join("2.srt");
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir];
///     # let cfg = Config::parse_from(args);
///     #
///     # let mut mi = MediaInfo::new(&cfg, 0);
///     # mi.try_insert(&first).unwrap();
///     # mi.try_insert(&second).unwrap();
///     # let o = StreamsOrder::new(&mut mi).unwrap();
///     #
///     assert_eq!(&o[0].src(), &first);
///     assert_eq!(&o[1].src(), &second);
///     ```
#[derive(Clone, Debug)]
pub struct StreamsOrder(pub Vec<StreamsOrderItem>);

/// A [`StreamsOrder`] item.
#[derive(Clone, Debug)]
pub struct StreamsOrderItem {
    /// A type of stream.
    pub ty: StreamType,

    /// A file path cached in [`MediaInfo`](crate::MediaInfo).
    pub key: ArcPathBuf,

    /// A stream index of file cached in [`MediaInfo`](crate::MediaInfo).
    pub key_i_stream: usize,

    /// A source of stream if the path is different key.
    pub src: Option<PathBuf>,

    /// The stream index within the src.
    pub i_stream: usize,

    /// A time of src part if not requires full src.
    pub src_time: Option<(Duration, Duration)>,

    /// A unique number assigned to the src at its first appearance in the [`StreamsOrder`] vector.
    /// NOT an index in the vector.
    /// For example, `vec[2].src_num == 1` if `vec[0].src() == vec[1].src()`.
    pub src_num: usize,

    /// Whether this is the first occurrence of the src in the [`StreamsOrder`].
    pub is_first_entry: bool,
}

deref_singleton_tuple_struct!(StreamsOrder, Vec<StreamsOrderItem>);

impl StreamsOrder {
    pub(crate) fn iter_first_entries(&self) -> impl Iterator<Item = &StreamsOrderItem> {
        self.0.iter().filter(|m| m.is_first_entry)
    }

    pub(crate) fn iter_track(&self) -> impl Iterator<Item = &StreamsOrderItem> {
        self.0.iter().filter(|m| m.ty.is_track())
    }
}

impl StreamsOrderItem {
    /// Returns a src file path.
    pub fn src(&self) -> &Path {
        match self.src.as_ref() {
            Some(p) => p,
            None => self.key.as_path(),
        }
    }
}

use mux_media::TrackType;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[test]
fn test_default() {
    assert_eq!(TrackType::NonCustomizable, TrackType::default());
}

#[test]
fn test_iter() {
    let expected = vec![
        TrackType::Video,
        TrackType::Audio,
        TrackType::Sub,
        TrackType::Button,
        TrackType::NonCustomizable,
    ];

    let variants: Vec<TrackType> = TrackType::iter().collect();

    assert_eq!(expected, variants);
}

macro_rules! build_test_map {
    ( $fn:ident; $( $ty:ty ),* ) => {
        #[test]
        fn $fn() {
            $(
                let exp = <$ty>::default();
                let map = TrackType::map::<$ty>();
                TrackType::iter().for_each(|tt| {
                    assert_eq!(exp.clone(), map[tt]);
                });
            )*
        }
    };
}

build_test_map!(
    test_map;
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, usize, isize, bool, char,
    String, Vec<u8>, Vec<i32>, Vec<String>, Option<u8>, Option<String>,
    HashMap<String, u8>, HashSet<String>, BTreeMap<String, u8>, BTreeSet<String>,
    Box<u32>, std::rc::Rc<u32>, std::sync::Arc<u32>, std::cell::RefCell<u32>,
    (u8,), (u8, i32), (String, u64, bool), [u8; 0], [u8; 1], [u8; 4], [u8; 8]
);

#[test]
fn test_from_matroska_tracktype() {
    assert_eq!(TrackType::Video, matroska::Tracktype::Video.into());
    assert_eq!(TrackType::Audio, matroska::Tracktype::Audio.into());
    assert_eq!(TrackType::Sub, matroska::Tracktype::Subtitle.into());
    assert_eq!(TrackType::Button, matroska::Tracktype::Buttons.into());

    assert_eq!(
        TrackType::NonCustomizable,
        matroska::Tracktype::Complex.into()
    );
    assert_eq!(TrackType::NonCustomizable, matroska::Tracktype::Logo.into());
    assert_eq!(
        TrackType::NonCustomizable,
        matroska::Tracktype::Control.into()
    );
    assert_eq!(
        TrackType::NonCustomizable,
        matroska::Tracktype::Unknown.into()
    );
}

use crate::TrackType;

#[derive(Default)]
pub struct TFlagsCounts {
    audio_default: u32,
    subs_default: u32,
    video_default: u32,
    buttons_default: u32,

    audio_forced: u32,
    subs_forced: u32,
    video_forced: u32,
    buttons_forced: u32,

    audio_enabled: u32,
    subs_enabled: u32,
    video_enabled: u32,
    buttons_enabled: u32,
}

macro_rules! add_and_get_any_track_type {
    ($($add:ident, $get:ident, $audio:ident, $subs:ident, $video:ident, $buttons:ident;)*) => {
        impl TFlagsCounts {
            $(
                pub fn $add(&mut self, tt: TrackType) {
                    match tt {
                        TrackType::Audio => self.$audio += 1,
                        TrackType::Sub => self.$subs += 1,
                        TrackType::Video => self.$video += 1,
                        TrackType::Button => self.$buttons += 1,
                    }
                }

                pub fn $get(&self, tt: TrackType) -> u32 {
                    match tt {
                        TrackType::Audio => self.$audio,
                        TrackType::Sub => self.$subs,
                        TrackType::Video => self.$video,
                        TrackType::Button => self.$buttons,
                    }
                }
            )*
        }
    };
}

add_and_get_any_track_type!(
    add_default, get_default, audio_default, subs_default, video_default, buttons_default;
    add_forced, get_forced, audio_forced, subs_forced, video_forced, buttons_forced;
    add_enabled, get_enabled, audio_enabled, subs_enabled, video_enabled, buttons_enabled;
);

use super::{FontAttachs, OtherAttachs};
use crate::to_json_args;

to_json_args!(@tracks_or_attachs, FontAttachs, Fonts, NoFonts);
to_json_args!(@tracks_or_attachs, OtherAttachs, Attachs, NoAttachs);

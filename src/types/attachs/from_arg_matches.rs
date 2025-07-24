use super::{FontAttachs, OtherAttachs};
use crate::from_arg_matches;

from_arg_matches!(@impl, FontAttachs, Fonts, NoFonts);
from_arg_matches!(@impl, OtherAttachs, Attachs, NoAttachs);

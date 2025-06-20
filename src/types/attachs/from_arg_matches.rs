use super::{Attachs, FontAttachs, OtherAttachs};
use crate::{cli_args, from_arg_matches};

cli_args!(Attachs, AttachsArg; Attachs => "", "-m", NoAttachs => "", "-M");
cli_args!(FontAttachs, FontAttachsArg; Fonts => "fonts", NoFonts => "no-fonts");
cli_args!(OtherAttachs, OtherAttachsArg; Attachs => "attachs", NoAttachs => "no-attachs");

impl clap::FromArgMatches for FontAttachs {
    from_arg_matches!(@unrealized_fns);
    from_arg_matches!(@fn_mut, Fonts, NoFonts);
}

impl clap::FromArgMatches for OtherAttachs {
    from_arg_matches!(@unrealized_fns);
    from_arg_matches!(@fn_mut, Attachs, NoAttachs);
}

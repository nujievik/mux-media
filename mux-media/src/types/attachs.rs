pub(crate) mod attach_type;
mod from_arg_matches;
mod from_str;
pub(crate) mod id;
mod save;
mod to_args;

use crate::{IsDefault, deref_singleton_tuple_fields};
use id::AttachID;
use std::collections::HashSet;

/// Settings for saving font attachments from media.
#[derive(Clone)]
pub struct FontAttachs(Attachs);

/// Settings for saving non-font attachments from media.
#[derive(Clone)]
pub struct OtherAttachs(Attachs);

/// Settings for saving media attachments.
#[derive(Clone, Default, PartialEq, IsDefault)]
pub struct Attachs {
    no_flag: bool,
    inverse: bool,
    ids_hashed: Option<HashSet<AttachID>>,
    ids_unhashed: Option<Vec<AttachID>>,
}

deref_singleton_tuple_fields!(FontAttachs, Attachs, @all, no_flag: bool);
deref_singleton_tuple_fields!(OtherAttachs, Attachs, @all, no_flag: bool);

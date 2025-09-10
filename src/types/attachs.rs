pub(crate) mod attach_type;
mod from_str;
pub(crate) mod id;
mod save;
mod to_args;

use crate::{IsDefault, deref_singleton_tuple_struct};
use id::AttachID;
use std::collections::HashSet;

/// Settings for saving font attachments from media.
#[derive(Clone, Debug, PartialEq, IsDefault)]
pub struct FontAttachs(pub Attachs);

/// Settings for saving non-font attachments from media.
#[derive(Clone, Debug, PartialEq, IsDefault)]
pub struct OtherAttachs(pub Attachs);

/// Settings for saving media attachments.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Attachs {
    pub no_flag: bool,
    pub inverse: bool,
    pub ids_hashed: Option<HashSet<AttachID>>,
    pub ids_unhashed: Option<Vec<AttachID>>,
}

deref_singleton_tuple_struct!(FontAttachs, Attachs, @all);
deref_singleton_tuple_struct!(OtherAttachs, Attachs, @all);

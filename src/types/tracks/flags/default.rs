use crate::{DefaultTFlags, EnabledTFlags, ForcedTFlags, TFlags};

macro_rules! flags_default_is_default {
    ( $( $type:ident ),* $(,)? ) => { $(
        impl Default for $type {
            fn default() -> Self {
                let lim = Self::default_lim_for_unset();
                Self(TFlags::default()).lim_for_unset(lim)
            }
        }

        impl $crate::IsDefault for $type {
            fn is_default(&self) -> bool {
                self.unmapped.is_none()
                    && self.map_hashed.is_none()
                    && self.map_unhashed.is_none()
                    && self.lim_for_unset == Self::default_lim_for_unset()
            }
        }
    )* };
}

flags_default_is_default!(DefaultTFlags, ForcedTFlags, EnabledTFlags);

impl DefaultTFlags {
    #[inline]
    pub(in super::super) fn default_lim_for_unset() -> u32 {
        1
    }
}

impl ForcedTFlags {
    #[inline]
    pub(in super::super) fn default_lim_for_unset() -> u32 {
        0
    }
}

impl EnabledTFlags {
    #[inline]
    pub(in super::super) fn default_lim_for_unset() -> u32 {
        u32::MAX
    }
}

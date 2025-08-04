/// Initializes a [`MediaInfo`](crate::MediaInfo) value if need and immutably borrows it.
#[macro_export]
macro_rules! immut {
    (@try, $ref_mut:ident, $marker:path) => {
        match $ref_mut.try_init_cmn::<$marker>() {
            Ok(()) => $ref_mut.try_immut_cmn::<$marker>(),
            Err(e) => Err(e),
        }
    };

    ($ref_mut:ident, $marker:path) => {
        match $ref_mut.init_cmn::<$marker>() {
            Some(()) => $ref_mut.immut_cmn::<$marker>(),
            None => None,
        }
    };

    (@try, $ref_mut:ident, $marker:path, $media:expr) => {
        match $ref_mut.try_init::<$marker>($media) {
            Ok(()) => $ref_mut.try_immut::<$marker>($media),
            Err(e) => Err(e),
        }
    };

    ($ref_mut:ident, $marker:ident, $media:expr) => {
        match $ref_mut.init::<$marker>($media) {
            Some(()) => $ref_mut.immut::<$marker>($media),
            None => None,
        }
    };

    (@try, $ref_mut:ident, $marker:path, $media:expr, $track:expr) => {
        match $ref_mut.try_init_ti::<$marker>($media, $track) {
            Ok(()) => $ref_mut.try_immut_ti::<$marker>($media, $track),
            Err(e) => Err(e),
        }
    };

    ($ref_mut:ident, $marker:path, $media:expr, $track:expr) => {
        match $ref_mut.init_ti::<$marker>($media, $track) {
            Some(()) => $ref_mut.immut_ti::<$marker>($media, $track),
            None => None,
        }
    };
}

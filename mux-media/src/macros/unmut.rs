/// Caches a [`MediaInfo`](crate::MediaInfo) value and immutably borrows it.
#[macro_export]
macro_rules! unmut {
    ($ref_mut:ident, $marker:ident) => {
        match $ref_mut.get_cmn::<$marker>() {
            Some(_) => $ref_mut.unmut_cmn::<$marker>()
            None => None,
        }
    };

    (@try, $ref_mut:ident, $marker:ident) => {{
        let _ = $ref_mut.try_get_cmn::<$marker>()?;
        $ref_mut.unmut_cmn::<$marker>().ok_or("Unexpected None")
    }};

    ($ref_mut:ident, $marker:ident, $media:expr) => {
        match $ref_mut.get::<$marker>($media) {
            Some(_) => $ref_mut.unmut::<$marker>($media),
            None => None,
        }
    };

    (@try, $ref_mut:ident, $marker:ident, $media:expr) => {{
        let _ = $ref_mut.try_get::<$marker>($media)?;
        $ref_mut.unmut::<$marker>($media).ok_or("Unexpected None")
    }};

    ($ref_mut:ident, $marker:ident, $media:expr, $track:expr) => {
        match $ref_mut.get_ti::<$marker>($media, $track) {
            Some(_) => $ref_mut.unmut_ti::<$marker>($media, $track)
            None => None,
        }
    };

    (@try, $ref_mut:ident, $marker:ident, $media:expr, $track:expr) => {{
        let _ = $ref_mut.try_get_ti::<$marker>($media, $track)?;
        $ref_mut.unmut_ti::<$marker>($media, $track).ok_or("Unexpected None")
    }};
}

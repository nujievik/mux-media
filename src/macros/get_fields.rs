#[doc(hidden)]
#[macro_export]
macro_rules! unmut_get {
    ($ref_mut:ident, $marker:ident, $path:expr) => {{
        // First mut get, this caches unhashed value if needed. Then unmut_get
        if $ref_mut.get::<$marker>($path).is_none() {
            None
        } else {
            $ref_mut.unmut_get::<$marker>($path)
        }
    }};

    (@try, $ref_mut:ident, $marker:ident, $path:expr) => {{
        let _ = $ref_mut.try_get::<$marker>($path)?;
        $ref_mut
            .unmut_get::<$marker>($path)
            .ok_or("Unexpected None")
    }};
}

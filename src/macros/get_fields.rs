#[macro_export]
macro_rules! unmut_get {
    ($ref_mut_self:ident, $marker:ident, $path:expr) => {{
        // First mut get, this caches unhashed value if needed. Then unmut_get
        if $ref_mut_self.get::<$marker>($path).is_none() {
            None
        } else {
            $ref_mut_self.unmut_get::<$marker>($path)
        }
    }};

    (@try, $ref_mut_self:ident, $marker:ident, $path:expr) => {{
        let _ = $ref_mut_self.try_get::<$marker>($path)?;
        $ref_mut_self
            .unmut_get::<$marker>($path)
            .ok_or("Unexpected None Targets")
    }};
}

#[macro_export]
macro_rules! get_fields {
    // trait GetField only
    ($type:ident;
    $( $field_name:ident, $field_ty:ty => $marker:ident ),* $(,)?
    ) => {
        $(
            pub struct $marker;

            impl $crate::GetField<$marker> for $type {
                type FieldType = $field_ty;
                fn get(&self) -> &Self::FieldType {
                    &self.$field_name
                }
            }
        )*
    };

    // trait GetField + trait GetOptField
    ($type:ident, $opt_type:ident;
    $( $field_name:ident, $field_ty:ty => $marker:ident ),* $(,)?
    ) => {
        $crate::get_fields!($type; $( $field_name, $field_ty => $marker ),*);

        $(
            impl $crate::GetOptField<$marker> for $opt_type {
                type FieldType = $field_ty;
                fn get(&self) -> Option<&Self::FieldType> {
                    self.$field_name.as_ref()
                }
            }
        )*
    };
}

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

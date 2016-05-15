#[macro_export]
macro_rules! create_type_parsing_impls {
    ($type_name:ident,
     $parsed_type_name:ident,
     $($i:ident: $ty:ty, $def:expr);+ $(;)*) => (
        impl $type_name {
            fn fill_from_parsed(mut self, parsed: $parsed_type_name) -> $type_name {
            $(
                if let Some(val) = parsed.$i {
                    self.$i = val;
                }
            )+
                self
            }

            pub fn from_json(json: &str) -> $type_name {
                use serde_json;

                let parsed = serde_json::from_str(json).expect("Could not parse JSON");
                $type_name::default().fill_from_parsed(parsed)
            }
        }

        // Template for the default struct
        impl Default for $type_name {
            fn default() -> Self {
                $type_name {
                    $(
                        $i: $def,
                    )+
                }
            }
        }
    )
}

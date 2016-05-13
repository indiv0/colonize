#[macro_export]
macro_rules! create_config {
    ($($i:ident: $ty:ty, $def:expr, $( $dstring:expr ),+ );+ $(;)*) => (
        #[derive(Deserialize, Serialize)]
        pub struct Config {
            $(pub $i: $ty),+
        }

        // Just like the `Config` struct but with each property wrapped in an
        // `Option<T>`. This is used to parse a `colonize.json` which doesn't
        // specify all the properties of `Config`.
        // We first parse into `ParsedConfig`, then create a default `Config`
        // and overwrite the properties with corresponding values from
        // `ParsedConfig`.
        #[derive(Deserialize, Serialize)]
        pub struct ParsedConfig {
            $(pub $i: Option<$ty>),+
        }

        impl Config {
            fn fill_from_parsed_config(mut self, parsed: ParsedConfig) -> Config {
            $(
                if let Some(val) = parsed.$i {
                    self.$i = val;
                }
            )+
                self
            }

            pub fn from_json(json: &str) -> Config {
                use serde_json;

                let parsed_config = serde_json::from_str(json).expect("Could not parse JSON");
                Config::default().fill_from_parsed_config(parsed_config)
            }
        }

        // Template for the default configuration
        impl Default for Config {
            fn default() -> Self {
                Config {
                    $(
                        $i: $def,
                    )+
                }
            }
        }
    )
}

#[macro_export]
macro_rules! create_localization {
    ($($i:ident: $ty:ty, $def:expr, $( $dstring:expr ),+ );+ $(;)*) => (
        #[derive(Deserialize, Serialize)]
        pub struct Localization {
            $(pub $i: $ty),+
        }

        // Just like the `Localization` struct but with each property wrapped in
        // an `Option<T>`. This is used to parse a `LANG.json` which doesn't
        // specify all the properties of `Localization`.
        // We first parse into `ParsedLocalization`, then create a default
        // `Localization` and overwrite the properties with corresponding values
        // from `ParsedLocalization`.
        #[derive(Deserialize, Serialize)]
        pub struct ParsedLocalization {
            $(pub $i: Option<$ty>),+
        }

        impl Localization {
            fn fill_from_parsed_localization(mut self, parsed: ParsedLocalization) -> Localization {
            $(
                if let Some(val) = parsed.$i {
                    self.$i = val;
                }
            )+
                self
            }

            pub fn from_json(json: &str) -> Localization {
                use serde_json;

                let parsed_localization = serde_json::from_str(json).expect("Could not parse JSON");
                Localization::default().fill_from_parsed_localization(parsed_localization)
            }
        }

        // Template for the default localization
        impl Default for Localization {
            fn default() -> Self {
                Localization {
                    $(
                        $i: $def,
                    )+
                }
            }
        }
    )
}

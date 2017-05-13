use std::path::Path;
use serde_yaml;

use super::file_utils;


/// Constructs a configuration struct with accessors to each field.
macro_rules! configuration {
    (
        $(
            $name:ident, $type:ty, $default:expr
        );*
    )=> {

            /// Configuration type to be serialized. Allows for keys to not be present in
            /// the configuration file.
            #[derive(Serialize, Deserialize, Debug)]
            struct RawConfiguration {
                $(
                    $name: Option<$type>,
                )*
            }

            impl RawConfiguration {
                /// Construct a `RawConfiguration` from the provided path. Will return an
                /// error if unable to parse the YAML file.
                fn from<P: AsRef<Path>>(config_path: P) -> Result<RawConfiguration, serde_yaml::Error> {
                    serde_yaml::from_str(&file_utils::read_from_file(config_path))
                }
            }
            #[derive(Debug)]
            pub struct Configuration {
                $(
                    $name: $type,
                )*
            }

            impl Default for Configuration {
                fn default() -> Configuration {
                    Configuration {
                        $(
                            $name: $default,
                        )*
                    }
                }
            }

            impl Configuration {
                /// Obtain a `Configuration` from the path provided.
                pub fn from<P: AsRef<Path>>(path: P) -> Result<Configuration, serde_yaml::Error> {
                    let raw_config = RawConfiguration::from(path)?;
                    let def_config = Configuration::default();
                    $(
                        let $name = raw_config.$name.unwrap_or(def_config.$name);
                    )*
                    let config = Configuration {
                        $(
                            $name,
                        )*
                    };
                    Ok(config)
                }

                $(
                    /// Returns the $name value of the configuration
                    pub fn $name(&self) -> $type {
                        self.$name.clone()
                    }
                )*
            }
    };
}

fn test_config_macro() {
    configuration!{
        stylesheet, String, "".to_string();
        gen_index, bool, false;
        out_dir, String, "out".to_string();
        copy_resources, bool, true;
        title, String, "title".to_string()
    }
}

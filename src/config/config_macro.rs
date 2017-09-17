/// Constructs a configuration struct with accessors to each field and default fields.
/// The macro will implement the `Default` trait for the values provided.
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
                    file_utils::read_from_file(config_path)
                        .map_err(|err| serde_yaml::Error::io(err))
                        .and_then(|contents| serde_yaml::from_str(&contents))
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
#[cfg(test)]
mod tests {
    use std::path::Path;
    use serde_yaml;

    use file_utils;

    configuration!{
        stylesheet, String, "".to_string();
        gen_index, bool, false;
        out_dir, String, "out".to_string();
        copy_resources, bool, true;
        title, String, "title".to_string()
    }
    #[test]
    fn test_config_macro_default() {
        let config_def = Configuration::default();
        assert_eq!(config_def.stylesheet(), "".to_string());
        assert_eq!(config_def.gen_index(), false);
        assert_eq!(config_def.out_dir(), "out".to_string());
        assert_eq!(config_def.copy_resources(), true);
        assert_eq!(config_def.title(), "title".to_string());
    }
}

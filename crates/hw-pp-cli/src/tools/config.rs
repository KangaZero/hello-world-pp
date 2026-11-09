use std::{fs, io, path::PathBuf};

use clap::ValueEnum;
use hw_pp_core::data::{
    common::{DEFAULT_HUMAN_LANGUAGE, DEFAULT_PROGRAMMING_LANGUAGE},
    human_languages::HumanLanguageName,
    notation::Notation,
    programming_languages::ProgrammingLanguageName,
    theme::{Theme, ThemeColors},
};
use hw_pp_macros::name_enum;
use serde::{Deserialize, Serialize};

use crate::{constants::cli_info::APP_NAME, tools::color::Color};

/// Where the colors for CLI output come from: a built-in preset or a user-defined palette.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ThemeChoice {
    /// A built-in theme, e.g. `mocha`.
    Preset(#[serde(with = "by_name")] Theme),
    /// A user-defined palette, replacing every preset color.
    Custom(#[serde(with = "palette")] ThemeColors),
}

impl Default for ThemeChoice {
    fn default() -> Self {
        Self::Preset(Theme::default())
    }
}

/// How command output is rendered.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Human-readable text.
    #[default]
    Plain,
    /// Machine-readable JSON, for scripts and pipes.
    Json,
}

/// Defaults for programming snippets.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SnippetConfig {
    /// Language used when no language is given on the command line.
    #[serde(with = "by_name")]
    pub language: ProgrammingLanguageName,
    /// Prefix each snippet line with its line number.
    pub line_numbers: bool,
}

impl Default for SnippetConfig {
    fn default() -> Self {
        Self {
            language: DEFAULT_PROGRAMMING_LANGUAGE,
            line_numbers: false,
        }
    }
}

/// User-level settings; CLI flags and env vars override these.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Schema version of the config file, bumped on breaking changes.
    pub version: u32,
    pub color: Color,
    #[serde(with = "by_name")]
    pub language: HumanLanguageName,
    pub theme: ThemeChoice,
    #[serde(with = "by_name")]
    pub notation: Notation,
    pub output: OutputFormat,
    pub snippet: SnippetConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            color: Color::default(),
            language: DEFAULT_HUMAN_LANGUAGE,
            theme: ThemeChoice::default(),
            notation: Notation::default(),
            output: OutputFormat::default(),
            snippet: SnippetConfig::default(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config: {0}")]
    Parse(#[from] toml_edit::de::Error),
    #[error("failed to edit config: {0}")]
    Edit(#[from] toml_edit::TomlError),
    #[error("failed to serialize config: {0}")]
    Serialize(#[from] toml_edit::ser::Error),
}

mod by_name {
    use std::{fmt::Display, str::FromStr};

    use serde::{Deserialize, Deserializer, Serializer, de};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(<D::Error as de::Error>::custom)
    }
}

mod palette {
    use anstyle::{Color as AnsiColor, Effects, RgbColor, Style};
    use hw_pp_core::data::theme::ThemeColors;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser};

    #[derive(Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    struct StyleDef {
        fg: String,
        #[serde(default)]
        bold: bool,
    }

    impl StyleDef {
        fn from_style(style: Style) -> Option<Self> {
            let Some(AnsiColor::Rgb(RgbColor(r, g, b))) = style.get_fg_color() else {
                return None;
            };
            Some(Self {
                fg: format!("#{r:02x}{g:02x}{b:02x}"),
                bold: style.get_effects().contains(Effects::BOLD),
            })
        }

        fn to_style(&self) -> Option<Style> {
            let hex = self
                .fg
                .strip_prefix('#')
                .filter(|hex| hex.len() == 6 && hex.bytes().all(|byte| byte.is_ascii_hexdigit()))?;
            let [_, r, g, b] = u32::from_str_radix(hex, 16).ok()?.to_be_bytes();
            let style = RgbColor(r, g, b).on_default();
            Some(if self.bold { style.bold() } else { style })
        }
    }

    macro_rules! palette_def {
        ($($field:ident),+ $(,)?) => {
            #[derive(Deserialize, Serialize)]
            #[serde(deny_unknown_fields)]
            struct PaletteDef {
                $($field: StyleDef,)+
            }

            pub fn serialize<S: Serializer>(
                colors: &ThemeColors,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                PaletteDef {
                    $($field: StyleDef::from_style(colors.$field).ok_or_else(|| {
                        <S::Error as ser::Error>::custom(concat!(
                            "theme.",
                            stringify!($field),
                            ": only RGB colors can be written as \"#rrggbb\"",
                        ))
                    })?,)+
                }
                .serialize(serializer)
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<ThemeColors, D::Error> {
                let def = PaletteDef::deserialize(deserializer)?;
                Ok(ThemeColors {
                    $($field: def.$field.to_style().ok_or_else(|| {
                        <D::Error as de::Error>::custom(format_args!(
                            concat!("theme.", stringify!($field), ": expected \"#rrggbb\", got {:?}"),
                            def.$field.fg,
                        ))
                    })?,)+
                })
            }
        };
    }

    palette_def!(
        header,
        usage,
        literal,
        placeholder,
        error,
        valid,
        invalid,
        warning,
        banner,
    );
}

name_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ConfigPath: ParseConfigPathError("path") {
    XdgConfigHome =>"XDG_CONFIG_HOME",
    AppData => "APPDATA",
    Home => "HOME",
}
}

impl Config {
    /// Schema version this build reads and writes.
    pub const CURRENT_VERSION: u32 = 1;

    fn config_dir() -> Option<PathBuf> {
        for env in ConfigPath::ALL {
            let matching = match env {
                ConfigPath::XdgConfigHome => Self::env_path(env.as_str()),
                ConfigPath::AppData if cfg!(windows) => Self::env_path(env.as_str()),
                ConfigPath::AppData => None,
                ConfigPath::Home => Self::env_path(env.as_str()).map(|h| h.join(".config")),
            };
            if let Some(matched) = matching {
                return Some(matched);
            }
        }
        None
    }

    fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join(APP_NAME).join("config.toml"))
    }

    fn example_config_path() -> PathBuf {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../config.toml"))
    }

    pub fn load_example() -> Result<Config, ConfigError> {
        let contents = fs::read_to_string(Self::example_config_path())?;
        Ok(toml_edit::from_str(&contents)?)
    }

    fn read_file() -> io::Result<String> {
        let path = Self::config_path()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no config directory found"))?;
        fs::read_to_string(path)
    }

    pub fn load() -> Result<Config, ConfigError> {
        let contents = Self::read_file()?;
        Ok(toml_edit::from_str(&contents)?)
    }

    pub fn save() -> io::Result<()> {
        fs::write(Self::config_path()?, Self::deserialize(&Self))
    }

    fn env_path(name: &str) -> Option<PathBuf> {
        std::env::var_os(name)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
    }
}

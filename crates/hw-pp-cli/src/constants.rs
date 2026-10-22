//! Compile-time constants for the CLI.

/// App name, banner and `--help` text/styling.
pub mod cli_info {
    use anstyle::Style;
    use hw_pp_core::data::{programming_languages_token::TokenKind, theme::Theme};

    /// Binary name from `[[bin]] name` in Cargo.toml (not the package name `hw-pp-cli`).
    pub const APP_NAME: &str = env!("CARGO_BIN_NAME");

    /// ASCII-art "HELLO, WORLD!++" banner, printed by `--banner` and atop `--help`.
    ///
    /// The raw string starts with a newline only so the art lines up in source.
    pub const BANNER: &str = r"
 _   _ _____ _     _     ___    __        _____  ____  _     ____  _
| | | | ____| |   | |   / _ \   \ \      / / _ \|  _ \| |   |  _ \| |    _     _
| |_| |  _| | |   | |  | | | |   \ \ /\ / / | | | |_) | |   | | | | |  _| |_ _| |_
|  _  | |___| |___| |__| |_| |    \ V  V /| |_| |  _ <| |___| |_| |_| |_   _|_   _|
|_| |_|_____|_____|_____\___( )    \_/\_/  \___/|_| \_\_____|____/(_)   |_|   |_|
                            |/
";

    /// Colors for `-h`/`--help` and parse errors, taken from the default snippet theme so
    /// help and code look alike. Each setter styles a role (what the text *is*), so new
    /// flags and subcommands pick it up.
    pub const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
        .header(TokenKind::Keyword.style(Theme::Default)) // `Options:`, `Commands:` (bold)
        .usage(TokenKind::Keyword.style(Theme::Default)) // `Usage:`
        .literal(TokenKind::Function.style(Theme::Default).bold()) // `--color`, `list`
        .placeholder(TokenKind::Type.style(Theme::Default)) // `<LANGUAGE>`
        .error(TokenKind::Invalid.style(Theme::Default).bold()) // `error:`
        .valid(TokenKind::String.style(Theme::Default)) // suggestions in errors
        .invalid(TokenKind::Number.style(Theme::Default).bold()); // the bad input in errors

    /// Style of the banner in `--help`.
    const BANNER_STYLE: Style = TokenKind::Keyword.style(Theme::Kanga);
    /// Bold text in `--help`.
    const BOLD: Style = Style::new().bold();
    /// Italic text in `--help`.
    const ITALIC: Style = Style::new().italic();

    /// Prepends the styled [`BANNER`] (minus its leading newline) to `text`.
    fn with_banner(text: &str) -> String {
        let banner = BANNER.trim_start_matches('\n');
        format!("{BANNER_STYLE}{banner}{BANNER_STYLE:#}\n{text}")
    }

    /// Short `-h` text: banner + `description` from Cargo.toml.
    #[must_use]
    pub fn about() -> String {
        with_banner(env!("CARGO_PKG_DESCRIPTION"))
    }

    /// Long `--help` text: banner + full description.
    ///
    /// Contains ANSI styles; `anstream` (clap's output stream) strips them when
    /// stdout is not a terminal or `NO_COLOR` is set.
    #[must_use]
    pub fn long_about() -> String {
        with_banner(&format!(
            "Your {BOLD}go-to tool{BOLD:#} for all your {ITALIC}'Hello, World!'{ITALIC:#} \
             outputs and formats in all sorts of {BOLD}languages{BOLD:#}, \
             {BOLD}writing systems{BOLD:#}, {BOLD}character notations{BOLD:#} \
             and much more!"
        ))
    }
}

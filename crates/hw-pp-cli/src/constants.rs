pub mod cli_info {
    use clap::builder::styling::{AnsiColor, Style};

    /// Binary name from `[[bin]] name` in Cargo.toml (not the package name `hw-pp-cli`).
    pub const APP_NAME: &str = env!("CARGO_BIN_NAME");

    // Raw string starts with a newline only so the art lines up in source.
    const BANNER: &str = r"
 _   _ _____ _     _     ___    __        _____  ____  _     ____  _
| | | | ____| |   | |   / _ \   \ \      / / _ \|  _ \| |   |  _ \| |    _     _
| |_| |  _| | |   | |  | | | |   \ \ /\ / / | | | |_) | |   | | | | |  _| |_ _| |_
|  _  | |___| |___| |__| |_| |    \ V  V /| |_| |  _ <| |___| |_| |_| |_   _|_   _|
|_| |_|_____|_____|_____\___( )    \_/\_/  \___/|_| \_\_____|____/(_)   |_|   |_|
                            |/
";

    const BANNER_STYLE: Style = AnsiColor::Cyan.on_default().bold();
    const BOLD: Style = Style::new().bold();
    const ITALIC: Style = Style::new().italic();

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
    /// Contains ANSI styles; clap strips them automatically when stdout
    /// is not a terminal or `NO_COLOR` is set.
    #[must_use]
    pub fn long_about() -> String {
        with_banner(&format!(
            "Your {BOLD}go-to tool{BOLD:#} for all your {ITALIC}'Hello, World!'{ITALIC:#} \
             outputs and formats in all sorts of {BOLD}languages{BOLD:#}, \
             {BOLD}writing systems{BOLD:#}, {BOLD}character encoding standards{BOLD:#} \
             and much more!"
        ))
    }
}

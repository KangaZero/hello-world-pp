use clap::ValueEnum;
use hw_pp_macros::name_enum;
use std::{
    env::{self},
    ffi::OsString,
};

use crate::Args;

name_enum! {
    /// A classification of a human language; a language may have several.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ColorEnv: ParseColorEnvError("color") {
    NoColor => "NO_COLOR",       //  !(is_empty()) ? true
    CliColorForce => "CLICOLOR_FORCE", // !(0 | is_empty()) ? true
    ForceColor => "FORCE_COLOR", // Node ecosystem !(0 | false) ? true
    CLIColor => "CLICOLOR",      // 0 = false, 1 = true
    Term => "TERM",              // !(dumb | is_empty()) ? true
    ColorTerm => "COLORTERM",    // truecolor / 24bit
    CI => "CI",                  // true | 1 ? true
    //NOTE: The below are added as info for color-related env variables but hw-pp-cli will not infer
    //from them
    // CargoTermColor => "CARGO_TERM_COLOR", // auto / always / never
    // RustLogStyle => "RUST_LOG_STYLE",     // env_logger: auto / always / never
    // NodeDisableColors => "NODE_DISABLE_COLORS",
    // COLORFGBG => "COLORFGBG",    // foreground;background hint (rxvt and others)
    // TermProgram => "TERM_PROGRAM", // iTerm.app, vscode, etc.
    // WtSession => "WT_SESSION",   // set in Windows Terminal
    // ANSICON => "ANSICON",        // Windows ANSI shim
    // ConEmuANSI => "ConEmuANSI",  // ConEmu

}
}

impl ColorEnv {
    fn env(self) -> Option<OsString> {
        env::var_os(self.as_str())
    }
}
/// When to syntax-highlight programming snippets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Color {
    /// Highlight snippets
    Auto,
    Always,
    Never,
}

impl Color {
    pub fn infer(args: &Args, stream: &dyn anstream::stream::RawStream) -> bool {
        match args.color {
            Color::Always => true,
            Color::Never => false,
            Color::Auto => Self::auto_detect(stream.is_terminal()),
        }
    }

    fn auto_detect(is_tty: bool) -> bool {
        if !is_tty {
            return false;
        }
        for var in ColorEnv::ALL {
            let enabled = match var {
                ColorEnv::NoColor => var.env().is_some_and(|v| !v.is_empty()),
                ColorEnv::CliColorForce => var.env().is_some_and(|v| !v.is_empty() && v != "0"),
                ColorEnv::ForceColor => var
                    .env()
                    .is_some_and(|v| !v.is_empty() && v != "0" && v != "false"),
                ColorEnv::CLIColor => var.env().is_some_and(|v| v == "1"),
                ColorEnv::Term => var.env().is_some_and(|v| !v.is_empty() && v != "dumb"),
                ColorEnv::ColorTerm => var.env().is_some_and(|v| !v.is_empty()),
                ColorEnv::CI => var.env().is_some_and(|v| v == "true" || v == "1"),
            };
            if enabled {
                return true;
            }
        }
        false
    }
}

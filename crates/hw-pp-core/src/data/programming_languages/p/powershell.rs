//! Hello-world snippet for `ProgrammingLanguageName::PowerShell`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-host>
//! - <https://learn.microsoft.com/en-us/powershell/scripting/overview>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::DotNet, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PowerShell => "PowerShell",
    versions: [PowerShell7 => "PowerShell 7"],
    extensions: [Ps1 => "ps1", Psm1 => "psm1"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PowerShell,
        extensions: &[FileExtension::Ps1, FileExtension::Psm1],
        categories: &[Scripting],
        families: &[DotNet],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PowerShell7,
            syntax: concat!(r#"Write-Host ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "Write-Host"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    };

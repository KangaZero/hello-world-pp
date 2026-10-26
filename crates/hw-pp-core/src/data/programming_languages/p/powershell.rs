//! Hello-world snippet for `ProgrammingLanguageName::PowerShell`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-host>
//! - <https://learn.microsoft.com/en-us/powershell/scripting/overview>
//! - <https://www.wikidata.org/wiki/Q840410> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Procedural, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::DotNet, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: PowerShell => "PowerShell",
    versions: [PowerShell7 => "PowerShell 7"],
    extensions: [Ps1 => "ps1", Psm1 => "psm1"],
}

const POWER_SHELL7: &str = concat!(r#"Write-Host ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PowerShell,
        extensions: &[FileExtension::Ps1, FileExtension::Psm1],
        categories: &[Scripting],
        families: &[DotNet],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            Reflective,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PowerShell7,
            syntax: POWER_SHELL7,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POWER_SHELL7,
                    [
                        (TokenKind::Function, "Write-Host"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                    ],
                )
            },
        }],
    };

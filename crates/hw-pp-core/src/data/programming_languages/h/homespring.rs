//! Hello-world snippet for `ProgrammingLanguageName::Homespring`.
//!
//! Imported from leachim6/hello-world (`h/Homespring.homespring` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Homespring => "Homespring",
    versions: [Homespring => "Homespring"],
    extensions: [Homespring => "homespring"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Homespring,
        extensions: &[FileExtension::Homespring],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Homespring,
            syntax: r"Universe bear hatchery powers world.
    bear hatchery powers o.      bear hatchery powers hell     marshy marshy marshy a snowmelt
",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Universe bear hatchery powers world.
    bear hatchery powers o.      bear hatchery powers hell     marshy marshy marshy a snowmelt
",
            )],
        }],
    };

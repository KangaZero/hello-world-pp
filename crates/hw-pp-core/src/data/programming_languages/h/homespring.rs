//! Hello-world snippet for `ProgrammingLanguageName::Homespring`.
//!
//! Imported from leachim6/hello-world (`h/Homespring.homespring` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Homespring.homespring>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Homespring => "Homespring",
    versions: [Homespring => "Homespring"],
    extensions: [Homespring => "homespring"],
}

const HOMESPRING: &str = r"Universe bear hatchery powers world.
    bear hatchery powers o.      bear hatchery powers hell     marshy marshy marshy a snowmelt
";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Homespring,
        extensions: &[FileExtension::Homespring],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Homespring,
            syntax: HOMESPRING,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HOMESPRING,
                    [(
                        TokenKind::Plain,
                        r"Universe bear hatchery powers world.
    bear hatchery powers o.      bear hatchery powers hell     marshy marshy marshy a snowmelt
",
                    )],
                )
            },
        }],
    };

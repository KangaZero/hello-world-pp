//! Hello-world snippet for `ProgrammingLanguageName::Goldfish`.
//!
//! Imported from leachim6/hello-world (`g/Goldfish.gfi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Goldfish.gfi>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Goldfish => "Goldfish",
    versions: [Goldfish => "Goldfish"],
    extensions: [Gfi => "gfi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Goldfish,
        extensions: &[FileExtension::Gfi],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Goldfish,
            syntax: r"0 iisiiiisiiiiiiiioniiisisioiiiiiiiooiiioniisiisddddoniisiiiiisiiiiiioniiisiisddddddddddoiiioddddddoddddddddoniiisio",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"0 iisiiiisiiiiiiiioniiisisioiiiiiiiooiiioniisiisddddoniisiiiiisiiiiiioniiisiisddddddddddoiiioddddddoddddddddoniiisio",
            )],
        }],
    };

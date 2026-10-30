//! Hello-world snippet for `ProgrammingLanguageName::Hexish`.
//!
//! Imported from leachim6/hello-world (`h/Hexish.hexish` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hexish.hexish>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Hexish => "Hexish",
    versions: [Hexish => "Hexish"],
    extensions: [Hexish => "hexish"],
}

const HEXISH: &str = r"0902160E48656C6C6F20776F726C6421001001161508";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hexish,
        extensions: &[FileExtension::Hexish],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hexish,
            syntax: HEXISH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HEXISH,
                    [(
                        TokenKind::Plain,
                        r"0902160E48656C6C6F20776F726C6421001001161508",
                    )],
                )
            },
        }],
    };

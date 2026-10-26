//! Hello-world snippet for `ProgrammingLanguageName::Labyrinth`.
//!
//! Imported from leachim6/hello-world (`l/Labyrinth.labyrinth` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Labyrinth.labyrinth>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Labyrinth => "Labyrinth",
    versions: [Labyrinth => "Labyrinth"],
    extensions: [Labyrinth => "labyrinth"],
}

const LABYRINTH: &str = r"33_100_108:}_114_111:}_87_32_44{{:_101_72.......
@";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Labyrinth,
        extensions: &[FileExtension::Labyrinth],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Labyrinth,
            syntax: LABYRINTH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LABYRINTH,
                    [(
                        TokenKind::Plain,
                        r"33_100_108:}_114_111:}_87_32_44{{:_101_72.......
@",
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Thue`.
//!
//! Imported from leachim6/hello-world (`t/Thue.th` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Thue.th>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Thue => "Thue",
    versions: [Thue => "Thue"],
    extensions: [Th => "th"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Thue,
        extensions: &[FileExtension::Th],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Thue,
            syntax: concat!(
                r"a::=~",
                output_for_hello_world!(),
                r"
::=
a
"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"a::=~",
                    output_for_hello_world!(),
                    r"
::=
a
"
                ),
            )],
        }],
    };

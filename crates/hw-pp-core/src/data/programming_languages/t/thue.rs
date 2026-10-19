//! Hello-world snippet for `ProgrammingLanguageName::Thue`.
//!
//! Imported from leachim6/hello-world (`t/Thue.th` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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

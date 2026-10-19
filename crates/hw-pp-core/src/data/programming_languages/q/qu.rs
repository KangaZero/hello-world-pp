//! Hello-world snippet for `ProgrammingLanguageName::Qu`.
//!
//! Imported from leachim6/hello-world (`q/QU.qu` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QU.qu>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Qu => "QU",
    versions: [Qu => "QU"],
    extensions: [Qu => "qu"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qu,
        extensions: &[FileExtension::Qu],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qu,
            syntax: concat!(
                r"'",
                output_for_hello_world!(),
                r"'
test"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"'",
                    output_for_hello_world!(),
                    r"'
test"
                ),
            )],
        }],
    };

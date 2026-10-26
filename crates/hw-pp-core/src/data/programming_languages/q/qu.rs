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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Qu => "QU",
    versions: [Qu => "QU"],
    extensions: [Qu => "qu"],
}

const QU: &str = concat!(
    r"'",
    output_for_hello_world!(),
    r"'
test"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qu,
        extensions: &[FileExtension::Qu],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qu,
            syntax: QU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QU,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"'",
                            output_for_hello_world!(),
                            r"'
test"
                        ),
                    )],
                )
            },
        }],
    };

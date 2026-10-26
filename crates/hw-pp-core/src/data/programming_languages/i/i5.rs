//! Hello-world snippet for `ProgrammingLanguageName::I5`.
//!
//! Imported from leachim6/hello-world (`i/I5.i5` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/I5.i5>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: I5 => "I5",
    versions: [I5 => "I5"],
    extensions: [I5 => "i5"],
}

const I5: &str = concat!(
    r#"fn main() {
    $print(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::I5,
        extensions: &[FileExtension::I5],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::I5,
            syntax: I5,
            tokenized_syntax: &const {
                checked_token_lengths(
                    I5,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"fn main() {
    $print(""#,
                            output_for_hello_world!(),
                            r#"")
}"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::One`.
//!
//! Imported from leachim6/hello-world (`o/One.one` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/One.one>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: One => "One",
    versions: [One => "One"],
    extensions: [One => "one"],
}

const ONE: &str = concat!(
    r#"fn main {
	__ ""#,
    output_for_hello_world!(),
    r#""
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::One,
        extensions: &[FileExtension::One],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::One,
            syntax: ONE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ONE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"fn main {
	__ ""#,
                            output_for_hello_world!(),
                            r#""
}"#
                        ),
                    )],
                )
            },
        }],
    };

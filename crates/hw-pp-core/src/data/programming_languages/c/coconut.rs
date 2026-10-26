//! Hello-world snippet for `ProgrammingLanguageName::Coconut`.
//!
//! Imported from leachim6/hello-world (`c/Coconut.coco` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Coconut.coco>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Coconut => "Coconut",
    versions: [Coconut => "Coconut"],
    extensions: [Coco => "coco"],
}

const COCONUT: &str = concat!(r#"""#, output_for_hello_world!(), r#"" |> print"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coconut,
        extensions: &[FileExtension::Coco],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coconut,
            syntax: COCONUT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COCONUT,
                    [
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" |> "),
                        (TokenKind::Function, r"print"),
                    ],
                )
            },
        }],
    };

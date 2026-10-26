//! Hello-world snippet for `ProgrammingLanguageName::Amos`.
//!
//! Imported from leachim6/hello-world (`a/Amos` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Amos>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Basic, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Amos => "Amos",
    versions: [Amos => "Amos"],
    extensions: [],
}

const AMOS: &str = concat!(r#"Print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Amos,
        extensions: &[],
        categories: &[Historical],
        families: &[Basic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Amos,
            syntax: AMOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AMOS,
                    [
                        (TokenKind::Keyword, r"Print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Frink`.
//!
//! Imported from leachim6/hello-world (`f/Frink.frink` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Frink.frink>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Frink => "Frink",
    versions: [Frink => "Frink"],
    extensions: [Frink => "frink"],
}

const FRINK: &str = concat!(r#"println[""#, output_for_hello_world!(), r#""]"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Frink,
        extensions: &[FileExtension::Frink],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Frink,
            syntax: FRINK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FRINK,
                    [
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"["),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"]"),
                    ],
                )
            },
        }],
    };
